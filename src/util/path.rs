use std::collections::HashMap;
use std::hash::Hash;

/// RoutePath's are used for mapping a request route to a handler.
/// If you want to access the internal data of an incoming request's path
/// use the .path_data() method on an incoming request.
pub struct RoutePath {
  segments: Vec<RoutePathSegment>,
}

impl RoutePath {
  pub fn new(str_path: &str) -> Self {
    Self::from(str_path)
  }
}

impl From<&str> for RoutePath {
  fn from(value: &str) -> Self {
    let segments = value.split("/")
      
      .map(|seg| {
        if seg.starts_with('<') && seg.ends_with('>') {
          RoutePathSegment::Variable(seg.into())
        } else {
          RoutePathSegment::Static(seg.into())
        }
      }).collect();
    
    Self {
      segments
    }
  }
}

enum RoutePathSegment {
  Static(String),
  Variable(String),
}


pub struct DataPath {
  raw_path: Vec<String>,
  variables: HashMap<String, usize>
}

impl DataPath {
  pub fn new(url: &str, route_path: RoutePath) -> Self {
    if let Some((_, query)) = url.split_once("?") {
      
    } else {
      
    }
  }
}