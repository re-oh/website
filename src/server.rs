use std::sync::Arc;
use tiny_http::{Server, Response, Request, Header};
use std::io;
use std::num::NonZero;
use std::str::FromStr;
use std::time::Duration;
use maud::Markup;
use once_cell::sync::Lazy;
use url::Url;
use uuid::Uuid;
use crate::{LoadedData, DATA};
use crate::post::Post;
use crate::templates::{error_template, homepage_template, not_found, post_template, posts_template};

pub struct WebsiteServer {
  server: Arc<Server>,
  handles: Vec<std::thread::JoinHandle<()>>,
  loaded_data: &'static LoadedData,
}

impl WebsiteServer {
  pub fn start(loaded_data: &'static LoadedData) -> WebsiteServer {
    let server = Arc::new(Server::http(&loaded_data.url).unwrap());

    println!("Server started at: {}", &loaded_data.url);

    let mut handles = Vec::new();
    let mut threads: usize = std::thread::available_parallelism().unwrap().into();
    if threads > 1 {
      threads = threads / 4;
    }
    for thread_num in 0..threads {

      let server = server.clone();

      let handle = std::thread::spawn(move || {
        let thread_num = thread_num + 1;
        loop {

          match server.recv_timeout(Duration::from_secs(1)) {
            Ok(Some(req)) => {
              println!("Received request: {:?} in thread: {}", req, thread_num);
              WebsiteServer::handle_request(&loaded_data, req);
            }
            Ok(None) => continue,
            Err(e) => {
              if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut {
                continue;
              } else {
                eprintln!("Error receiving request: {}", e);
                std::thread::sleep(Duration::from_secs(1));
              }
            }
          }

        }
      });
      handles.push(handle);
    }

    println!("Started: {} Threads", &handles.len());

    WebsiteServer {
      server,
      handles,
      loaded_data,
    }
  }



  fn handle_request(loaded_data: &'static LoadedData, request: Request) {

    let mut html_response = not_found().into_string();

    let full_url = format!("http://{}{}", loaded_data.url, request.url());

    let parsed_url = Url::parse(&full_url).unwrap();
    let path = parsed_url.path();

    println!("BOMBOCLAT PATH: {}", path);

    let query_pairs: Vec<(String, String)> = parsed_url
      .query_pairs()
      .map(|(key, value)| (key.into_owned(), value.into_owned()))
      .collect();

    let mut result = HandleResult::NotFound;
      match path {
      "/" => {
        html_response = homepage_template().into_string();
        result = HandleResult::Ok;
      },
      "/posts" => {

        if let Some((_, query_id)) = query_pairs.iter().find(|(key, _)| key == "id") {
          if let Ok(id) = Uuid::from_str(query_id) {
            if let Some(post) = loaded_data.posts.iter().find(|post| post.id() == id) {
              html_response = post_template(post).into_string();
              result = HandleResult::Ok;
            }
          } else {
            html_response = error_template("Invalid ID").into_string();
            result = HandleResult::InvalidId;
          }
        } else {
          html_response = posts_template(loaded_data.posts.iter().map(|post| (post.to_html(false), post.id())).collect::<Vec<(Markup, Uuid)>>()).into_string();
          result = HandleResult::Ok;
        }

      },
      _ => {},
    };

    let mut response= Response::from_string(html_response)
      .with_header(
        Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..])
          .unwrap());

    match result {
      HandleResult::Ok => {
        response = response.with_status_code(200);
      },
      HandleResult::InvalidId => {
        response = response.with_status_code(400);
      },
      HandleResult::NotFound => {
        response = response.with_status_code(404);
      }
    }

    if let Err(e) = request.respond(response) {
      eprintln!("Error responding to request: {}", e);
    };
  }
}

enum HandleResult {
  Ok,
  NotFound,
  InvalidId,
}