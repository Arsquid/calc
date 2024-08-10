mod calculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = calculator::run_calc(&std::env::args().collect::<Vec<String>>()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }

    Ok(())
}
