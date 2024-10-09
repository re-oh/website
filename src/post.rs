use std::fs::write;
use std::path::{Path, PathBuf};
use maud::{html, Markup, PreEscaped};
use uuid::Uuid;
use crate::md_parser;

pub struct Post {
  title: String,
  date: String,
  body: String,
  id: Uuid,
}

impl Post {
  pub fn to_html(&self, full: bool) -> Markup {
    let post = html! {
      article {
        h2 { (self.title.clone()) }
        h3 { (self.date.clone()) }
        @if full == true {
          (md_parser::render_md(&self.body))
        } @else {
          a href={(format!("/posts?id={}", self.id))} { "Read More" }
        }
      }
    };
    println!("Rendered Post: {:#?}", &post);
    post
  }

  pub fn title(&self) -> String {
    self.title.clone()
  }

  pub fn id(&self) -> Uuid {
    self.id
  }

}


pub fn parse_post(raw: String, path: &PathBuf) -> Post {
  let mut title = String::new();
  let mut date = String::new();
  let mut body = String::new();
  let mut id: Option<Uuid> = None;
  let mut in_body = false;

  for line in raw.lines() {
    if !in_body {
      if line.starts_with("#title:") {
        title = line.trim_start_matches("#title:").trim().to_string();
      } else if line.starts_with("#date:") {
        date = line.trim_start_matches("#date:").trim().to_string();
      } else if line.starts_with("#id:") {
        id = Uuid::parse_str(line.trim_start_matches("#id:").trim()).ok();
      } else if !line.is_empty() {
        in_body = true;
        body.push_str(line);
        body.push('\n');
      }
    } else {
      body.push_str(line);
      body.push('\n');
    }
  }

  if body.ends_with('\n') {
    body.pop();
  }



  if let Some(verified_id) = id {
    Post {
      title,
      date,
      body,
      id: verified_id,
    }
  } else {
    let new_id = Uuid::new_v4();
    let new_post = Post {
      title,
      date,
      body,
      id: new_id,
    };
    let new_post_raw = format!("#title: {}\n#date: {}\n#id: {}\n{}", new_post.title, new_post.date, new_post.id, new_post.body);
    write(path, new_post_raw).unwrap();
    new_post
  }


}

fn read_posts() -> Vec<(String, PathBuf)> {
  let mut posts = Vec::new();
  let post_dir = std::env::current_dir().unwrap().join("posts");

  for entry in std::fs::read_dir(post_dir).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    let raw = std::fs::read_to_string(&path).unwrap();
    posts.push((raw, path));
  }

  posts
}

pub fn load_posts() -> Vec<Post> {
  read_posts().iter().map(|(raw, path)| parse_post(raw.to_string(), path)).collect()
}