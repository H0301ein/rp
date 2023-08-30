#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::get("https://www.cloudflare.com/cdn-cgi/trace").await?;
    let body = res.text().await?;
    println!("{}", body);
    Ok(())
}
