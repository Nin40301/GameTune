#![forbid(unsafe_code)]
use ametista_cli::{args::Options, *};
use std::{io::Write, process::ExitCode};
fn main() -> ExitCode {
    let args: Vec<_> = std::env::args_os()
        .skip(1)
        .map(|s| s.to_string_lossy().into_owned())
        .collect();
    let language = args
        .windows(2)
        .find(|p| p[0] == "--lang" && p[1] == "en")
        .map_or("pt-BR", |_| "en");
    let json = args.iter().any(|s| s == "--json");
    let locale = match Locale::load(language) {
        Ok(locale) => locale,
        Err(_) => return ExitCode::from(7),
    };
    let result = (|| -> Result<(Response, bool), Failure> {
        let options = Options::parse(&args)?;
        if let Some(reply) = static_response(&options, &locale)? {
            return Ok((reply, options.json));
        }
        let mut backend = LiveBackend::connect()?;
        Ok((execute(&options, &mut backend, &locale)?, options.json))
    })();
    let (text, code, stderr) = match result {
        Ok((report, json)) => {
            let code = report.exit_code;
            let text = if json {
                serde_json::to_string_pretty(&report).unwrap_or_default() + "\n"
            } else {
                text_report(&report, &locale)
            };
            (text, code, false)
        }
        Err(error) => {
            let text = if json {
                error_json(&error, &locale, language).to_string() + "\n"
            } else {
                locale.text(error.key) + "\n"
            };
            (text, error.code, !json)
        }
    };
    let written = if stderr {
        std::io::stderr().lock().write_all(text.as_bytes())
    } else {
        std::io::stdout().lock().write_all(text.as_bytes())
    };
    ExitCode::from(if written.is_ok() { code } else { 7 })
}
