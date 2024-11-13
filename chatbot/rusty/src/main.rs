use rusty::Rusty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rusty = Rusty::new();
    rusty.run("0.0.0.0:80").await?;
    Ok(())
}
