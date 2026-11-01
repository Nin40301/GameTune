//! Offline import only: we never execute PresentMon or attach to a game.
use crate::{
    Error, Result,
    stats::{distribution, percentile},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    io::{Read, Seek, SeekFrom},
    path::Path,
};
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Capture {
    pub application: String,
    pub frames: usize,
    pub average_fps: f64,
    pub one_percent_low_fps: f64,
    pub frame_time: FrameTime,
    pub frame_time_p99: f64,
    pub interval_column: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct FrameTime {
    pub median: f64,
    pub p5: f64,
    pub p95: f64,
    pub minimum: f64,
    pub maximum: f64,
    pub mad: f64,
}
pub fn installed(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
        return false;
    };
    if !name.to_ascii_lowercase().starts_with("presentmon")
        || path
            .extension()
            .is_none_or(|x| !x.eq_ignore_ascii_case("exe"))
    {
        return false;
    }
    let Ok(mut file) = std::fs::File::open(path) else {
        return false;
    };
    let mut header = [0u8; 64];
    if file.read_exact(&mut header).is_err() || &header[..2] != b"MZ" {
        return false;
    }
    let offset = u32::from_le_bytes([header[60], header[61], header[62], header[63]]) as u64;
    if !(64..=1024 * 1024).contains(&offset) || file.seek(SeekFrom::Start(offset)).is_err() {
        return false;
    }
    let mut signature = [0; 4];
    file.read_exact(&mut signature).is_ok() && signature == *b"PE\0\0"
}
pub fn parse(reader: impl Read) -> Result<Capture> {
    let mut bytes = Vec::new();
    reader
        .take(32 * 1024 * 1024 + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| Error::PresentMon)?;
    if bytes.len() > 32 * 1024 * 1024 {
        return Err(Error::PresentMon);
    }
    let mut csv = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(bytes.as_slice());
    let headers = csv.headers().map_err(|_| Error::PresentMon)?.clone();
    if headers.len() > 100 || headers.iter().collect::<BTreeSet<_>>().len() != headers.len() {
        return Err(Error::PresentMon);
    }
    let index = |name: &str| {
        headers
            .iter()
            .position(|h| h.trim_start_matches('\u{feff}') == name)
            .ok_or(Error::PresentMon)
    };
    let app = index("Application")?;
    let pid = index("ProcessID")?;
    let swap = index("SwapChainAddress")?;
    // v1 interval between presents. Deliberately reject v2 GPU-busy durations:
    // GPU time is NOT frame time. v2 users can export with --v1_metrics.
    let interval = index("MsBetweenPresents")?;
    let time = index("TimeInSeconds")?;
    let mut identity = None;
    let mut frames = Vec::new();
    let mut previous = None;
    for (n, row) in csv.records().enumerate() {
        if n >= 1_000_000 {
            return Err(Error::PresentMon);
        }
        let row = row.map_err(|_| Error::PresentMon)?;
        let get = |i| row.get(i).ok_or(Error::PresentMon);
        let id = (
            get(app)?.to_string(),
            get(pid)?.to_string(),
            get(swap)?.to_string(),
        );
        if id.0.is_empty() || id.0.len() > 512 || id.1.parse::<u32>().is_err() {
            return Err(Error::PresentMon);
        }
        if identity.as_ref().is_some_and(|old| old != &id) {
            return Err(Error::PresentMon);
        }
        identity = Some(id);
        let t = get(time)?.parse::<f64>().map_err(|_| Error::PresentMon)?;
        let value = get(interval)?
            .parse::<f64>()
            .map_err(|_| Error::PresentMon)?;
        if !t.is_finite()
            || !value.is_finite()
            || t < 0.0
            || !(0.0..=60000.0).contains(&value)
            || previous.is_some_and(|p| t <= p)
        {
            return Err(Error::PresentMon);
        }
        // Initial interval precedes capture; discard it even if positive.
        if previous.is_some() {
            if value == 0.0 {
                return Err(Error::PresentMon);
            }
            frames.push(value);
        }
        previous = Some(t);
    }
    if frames.len() < 100 {
        return Err(Error::PresentMon);
    }
    let mut sorted = frames.clone();
    sorted.sort_by(f64::total_cmp);
    let tail = sorted.len().div_ceil(100);
    let slow = &sorted[sorted.len() - tail..];
    let d = distribution(&frames).ok_or(Error::PresentMon)?;
    let frame_time = FrameTime {
        median: d.median,
        p5: d.p5,
        p95: d.p95,
        minimum: d.minimum,
        maximum: d.maximum,
        mad: d.mad,
    };
    Ok(Capture {
        application: identity.ok_or(Error::PresentMon)?.0,
        frames: frames.len(),
        average_fps: 1000.0 / (frames.iter().sum::<f64>() / frames.len() as f64),
        one_percent_low_fps: 1000.0 / (slow.iter().sum::<f64>() / tail as f64),
        frame_time,
        frame_time_p99: percentile(&frames, 0.99).ok_or(Error::PresentMon)?,
        interval_column: "MsBetweenPresents".into(),
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn csv() -> String {
        let mut csv =
            "Application,ProcessID,SwapChainAddress,TimeInSeconds,MsBetweenPresents\n".to_string();
        for i in 0..201 {
            csv.push_str(&format!(
                "game.exe,12,0x123,{},{}\n",
                i as f64 / 60.0,
                if i == 200 { 50.0 } else { 1000.0 / 60.0 }
            ));
        }
        csv
    }
    #[test]
    fn fps_uses_intervals_and_slowest_one_percent() -> Result<()> {
        let c = parse(csv().as_bytes())?;
        assert_eq!(c.frames, 200);
        assert!((c.average_fps - 59.405940594).abs() < 0.001);
        assert!((c.one_percent_low_fps - 30.0).abs() < 0.001);
        Ok(())
    }
    #[test]
    fn rejects_multiple_swapchains_nan_and_wrong_metric() {
        assert!(
            parse(
                csv()
                    .replace("game.exe,12,0x123,3.333", "game.exe,12,0x456,3.333")
                    .as_bytes()
            )
            .is_err()
        );
        assert!(parse(csv().replace(",50\n", ",NaN\n").as_bytes()).is_err());
        assert!(parse(csv().replace("MsBetweenPresents", "MsGPUTime").as_bytes()).is_err());
    }
}
