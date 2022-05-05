use over::{cli, Expect};

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     cli::main()?;
//     Ok(())
// }


#[tokio::main]
async fn main() -> Expect<()> {
    cli::main().await?;
    Ok(())
}
