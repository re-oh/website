use tiny_http::{Request, StatusCode};

pub enum HandleResultErrorKind {
  ResourceNotFound(String),
}
pub enum HandleResult {
  Ok {
    code: StatusCode,
  },
  Error {
    kind: HandleResultErrorKind,
  }
}


trait Handler {
  fn handle(request: Request, route_data: DataPath) -> HandleResult {}
}