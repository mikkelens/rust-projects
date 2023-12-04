use reqwest::StatusCode;
use std::collections::HashMap;
use std::ops::AddAssign;
use std::time::Duration;

const URL: &str = "https://www.proshop.dk/";
const GET_REQUEST_SECS: u64 = 5;
const TRY_AGAIN_SECS: u64 = 10;

#[tokio::main]
async fn main() {
    let mut response_count: u64 = 0;
    let mut delays: u64 = 0;
    let mut responses: HashMap<StatusCode, u64> = HashMap::new();
    loop {
        while let Ok(response) = reqwest::get(URL).await {
            let code = response.status();
            responses.entry(code).or_default().add_assign(1);
            print!("\x1B[2J\x1B[1;1H"); // clears screen: https://stackoverflow.com/a/62101709
            println!(
                "Time elapsed: {} seconds",
                response_count * GET_REQUEST_SECS + delays * TRY_AGAIN_SECS
            );
            println!("Responses: {:?}", responses);
            response_count += 1;
            tokio::time::sleep(Duration::from_secs(GET_REQUEST_SECS)).await;
        }
        println!(
            "Failed to get response from url ('{}'), trying again in {} seconds.",
            URL, TRY_AGAIN_SECS
        );
        delays += 1;
        tokio::time::sleep(Duration::from_secs(TRY_AGAIN_SECS)).await;
    }
}