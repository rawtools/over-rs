use over::cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    cli::main()?;
    Ok(())
}
