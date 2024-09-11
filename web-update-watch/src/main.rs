use std::time::Duration;

#[tokio::main]
async fn main() {
    let url = "https://ufm.dk/uddannelse/videregaende-uddannelse/sogning-optag-og-vejledning/ledige-pladser";

    #[allow(unused)]
    {
        let mut prev_was_change = true;
        let mut prev = ();
    }

    loop {
        let response = reqwest::get(url).await;
        match response {
            Ok(response) => {
                let url = response.url();
                let mut buffer = String::new();
                let src = url
                    .domain()
                    .or_else(|| {
                        url.host().map(|h| {
                            buffer = h.to_string();
                            buffer.as_ref()
                        })
                    })
                    .unwrap_or_else(|| url.as_ref());

                let buffer;
                let src_2 = match url.domain() {
                    None => match url.host() {
                        None => url.as_ref(),
                        Some(host) => {
                            buffer = host.to_string();
                            buffer.as_ref()
                        }
                    },
                    Some(domain) => domain,
                };

                println!("Got response: {} @ {}{}", response.status(), src, src_2);

                let again_sleep = tokio::time::sleep(Duration::from_millis(1250));
                tokio::select! {
                    () = again_sleep => {
                        print!("Going again... ");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed HTTP GET request:\n   {}", e);

                let retry_delay = Duration::from_millis(1250);
                let retry_sleep = tokio::time::sleep(retry_delay);
                let continue_signal = tokio::signal::ctrl_c();
                tokio::select! {
                    () = retry_sleep => {
                        print!("Waited the full {} ms, trying again... ", retry_delay.as_millis());
                    }
                    Ok(()) = continue_signal => {
                        print!("Skipped wait (ctrl-c), trying again... ");
                    }
                }
            }
        }
    }
}