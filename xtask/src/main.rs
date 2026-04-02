mod commands;

fn main() {
    let cmd = std::env::args().nth(1).unwrap_or_else(|| "help".to_string());
    let result = match cmd.as_str() {
        "build-all" => commands::build::run(),
        "test-all" => commands::test::run(),
        "bench-all" => commands::bench::run(),
        "publish" => commands::publish::run(),
        "clean" => commands::clean::run(),
        "verify" => commands::verify::run(),
        "lint" => commands::lint::run(),
        _ => {
            eprintln!("Usage: cargo xtask <build-all|test-all|bench-all|publish|clean|verify|lint>");
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("xtask failed: {e}");
        std::process::exit(1);
    }
}
