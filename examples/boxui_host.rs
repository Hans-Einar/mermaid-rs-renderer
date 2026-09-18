//! JSON protocol witness for host developers. No command dispatch or networking.
use mermaid_rs_renderer::boxui::*;
use std::io::{Read, Write};
fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    if !matches!(mode.as_str(), "parse" | "prepare") {
        eprintln!("Usage: boxui_host parse|prepare < request > result.json");
        std::process::exit(1);
    }
    let limit = if mode == "parse" {
        MAX_SOURCE_BYTES
    } else {
        MAX_FRAME_BYTES
    };
    let mut input = Vec::new();
    if let Err(e) = std::io::stdin()
        .take((limit + 1) as u64)
        .read_to_end(&mut input)
    {
        eprintln!("{e}");
        std::process::exit(1);
    }
    let result = if mode == "parse" {
        parse_boxui_bytes(&input).map(|v| serde_json::to_value(v).unwrap())
    } else {
        decode_prepare_json(&input)
            .and_then(|r| prepare_boxui(&r, &MonospaceMetrics))
            .map(|v| serde_json::to_value(v).unwrap())
    };
    let (output, status) = match result {
        Ok(v) => (v, 0),
        Err(e) => {
            let status = e.status_code() as i32;
            (
                serde_json::json!({"contract":CONTRACT,"diagnostics":[e.diagnostic]}),
                status,
            )
        }
    };
    let mut out = std::io::stdout().lock();
    if serde_json::to_writer(&mut out, &output).is_err() || out.write_all(b"\n").is_err() {
        std::process::exit(1);
    }
    std::process::exit(status);
}
