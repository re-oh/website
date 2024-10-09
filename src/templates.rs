use maud::{html, Markup, PreEscaped};
use uuid::Uuid;
use crate::post::Post;

pub fn nav_bar() -> Markup {
  html! (
    nav {
      ul {
        li { a href="/" { "Home" } }
        li { a href="/posts" { "Posts" } }
      }
    }
  )
}

pub fn homepage_template() -> Markup {
  html! (
    html {
      head {
        title { "Homepage" }
      }
      body {
        (nav_bar())
        h1 { "Homepage" }
        p { "Welcome to the homepage!" }
      }
    }
  )
}

pub fn post_template(post: &Post) -> Markup {
  html! (
    html {
      head {
        title { (post.title()) }
      }
      body {
        (nav_bar())
        a href="/posts" { "back" }
        article {
          ( post.to_html(true) )
        }
      }
    }
  )
}

pub fn error_template(err: &str) -> Markup {
  html! (
    html {
      head {
        title { "Uh oh..." }
      }
      body {
        (nav_bar())
        h1 { "Something went wrong... :<" }
        p { (err) }
      }
    }
  )
}

pub fn posts_template(posts: Vec<(Markup, Uuid)>) -> Markup {
  html!(
    html {
      head {
        title { "Posts" }
      }
      body {
        (nav_bar())
        h1 { "Posts" }
        @for (post, uuid) in posts {
          (post)
        }
      }
    }
  )
}

pub fn not_found() -> Markup {
  html! (
    html {
      head {
        title { "404 Not Found" }
      }
      body {
        (nav_bar())
        h1 { "404 Not Found" }
        p { "The page you are looking for does not exist." }
      }
    }
  )
}