extern crate reqwest;

use reqwest::Response;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    get_body().await;
    post_body().await;
    Ok(())
}

async fn get_body() -> Result<(), reqwest::Error> {
    println!("get");
    let response: Response = reqwest::get("https://www.rust-lang.org").await?;
    let body = response.text().await?;
    println!("response: {:?}", body);

    Ok(())
}

async fn post_body() -> Result<(), reqwest::Error> {
    println!("post");
    let client = reqwest::Client::new();
    let response = client.post("http://httpbin.org/post").body("the body is posted").send().await?;
    
    println!("response: {:?}", response);
    println!("status: {:?}", response.status());
    Ok(())
}
