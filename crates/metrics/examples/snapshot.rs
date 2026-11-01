#![forbid(unsafe_code)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut collector = gametune_metrics::Collector::default();
    collector.sample()?;
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("{}", serde_json::to_string_pretty(&collector.sample()?)?);
    Ok(())
}
