use crate::post::{load_posts, Post};
use once_cell::sync::Lazy;
use std::fs::read_dir;
use std::ops::Deref;

mod server;
mod post;
mod md_parser;
mod templates;

const URL: &'static str = "127.0.0.1:8000";

pub struct LoadedData {
    url: String,
    posts: Vec<Post>,
}

static DATA: Lazy<LoadedData> = Lazy::new(|| {
    let posts = load_posts();

    println!("Data initialized: {} posts loaded.", posts.len());

    LoadedData { posts, url: String::from(URL) }

});

fn main() {
    let server = server::WebsiteServer::start(DATA.deref());

    println!("Server started. Press Ctrl+C to stop.");

    // keep alive
    loop {
        std::thread::sleep(std::time::Duration::from_millis(800));
    }
}