pub trait HtmlTemplate {
  fn render(&self) -> String;
}

pub trait HtmlTemplateStatic: HtmlTemplate + Default {
  fn static_render() -> String {
    let tmp = Self::default();
    tmp.render()
  }
}

impl<T> HtmlTemplateStatic for T where T: HtmlTemplate + Default {}


#[derive(Default)]
struct Homepage;
impl HtmlTemplate for Homepage {
  fn render(&self) -> String { "Homepage".into() }
}