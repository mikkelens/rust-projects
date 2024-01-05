use atom_syndication as atom;
use std::collections::VecDeque;
use std::iter;
use std::time::Duration;

fn main() {
    let mut latest = None;
    let mut just_saw_new_post = false;
    loop {
        if let Ok(feed_response) = reqwest::blocking::get("https://www.factorio.com/blog/rss") {
            let feed_body = feed_response.text().expect("parsable as text");
            let channel = atom::Feed::read_from(feed_body.as_bytes()).expect("parsable as channel");
            let mut new_posts = channel
                .entries
                .into_iter()
                .take_while(|item| {
                    !latest
                        .as_ref()
                        .is_some_and(|latest_item| latest_item == item)
                })
                .collect::<VecDeque<_>>();
            if let Some(newest) = new_posts.pop_front() {
                println!(
                    "NEW POSTS:\n{}",
                    iter::once(&newest)
                        .chain(new_posts.iter())
                        .map(|post| post.title.value.as_str())
                        .collect::<String>()
                );
                latest = Some(newest);
                just_saw_new_post = true;
            } else if just_saw_new_post {
                eprintln!("Waiting for new posts...");
                just_saw_new_post = false;
            }
        } else {
            eprintln!("No response?");
        }
        std::thread::sleep(Duration::from_secs(10));
    }
}