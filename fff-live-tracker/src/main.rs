use reqwest::{Response, StatusCode};
use std::time::Duration;

#[tokio::main]
async fn main() {
    match std::env::args().nth(1).map(|val| val.parse::<u32>()) {
        Some(Ok(num)) => weekly_loop(num).await,
        Some(Err(e)) => eprintln!("Could not parse input: {}", e),
        None => println!("No value to track!"),
    }
}

async fn weekly_loop(first: u32) {
    for num in first.. {
        let response =
            wait_for_new_fff(format!("https://www.factorio.com/blog/post/fff-{}", num)).await;
        println!(
            "Successful response implies that a new Factorio Friday Facts post has been released."
        );
        let text = response.text().await.unwrap();
    }
}

async fn wait_for_new_fff(url: String) -> Response {
    loop {
        if let Ok(body) = reqwest::get(&url).await {
            match body.status() {
                StatusCode::NOT_FOUND => println!("waiting..."),
                status if status.is_success() => {
                    println!("Got succes-response: {}", status);
                    break body;
                }
                other => eprintln!("Unhandled status code returned: {}", other),
            }
        } else {
            eprintln!("Could not get response...");
        }
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
}