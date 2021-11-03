extern crate reqwest;

use std::collections::HashMap;
use reqwest::Response;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    get_body().await;
    post_body().await;
    post_form_body().await;
    post_json_body().await;
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

// post the body of `name=alice&age=12` with RequestBuilder form()
async fn post_form_body() -> Result<(), reqwest::Error> {
    println!("post form data");
    let params = [("name", "alice"), ("age", "12")];
    let client = reqwest::Client::new();
    let response = client.post("http://httpbin.org/post").form(&params).send().await?;
    
    println!("response: {:?}", response);
    println!("status: {:?}", response.status());
    Ok(())
}

// post the body of json (`{"name": "alice", "age": "14"}`) with RequestBuilder json()
async fn post_json_body() -> Result<(), reqwest::Error> {
    println!("post json data");
    let mut json_map = HashMap::new();
    json_map.insert("name", "alice");
    json_map.insert("age", "12");
    let client = reqwest::Client::new();
    let response = client.post("http://httpbin.org/post").json(&json_map).send().await?;
    
    println!("response: {:?}", response);
    println!("status: {:?}", response.status());
    Ok(())
}
