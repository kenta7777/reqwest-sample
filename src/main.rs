extern crate reqwest;

use reqwest::Response;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    get_body().await;
    Ok(())
}

async fn get_body() -> Result<(), reqwest::Error> {
    println!("get");
    let response: Response = reqwest::get("https://www.rust-lang.org").await?;
    let body = response.text().await?;
    println!("response: {:?}", body);

    Ok(())
}
