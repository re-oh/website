use comrak::{markdown_to_html, ComrakOptions};
use maud::{Markup, PreEscaped};

pub fn render_md(md: &str) -> Markup {
  let res = markdown_to_html(md, &ComrakOptions::default());
  println!("Rendered MD: {:#?}", &res);
  PreEscaped(res)
}