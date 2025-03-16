use std::collections::HashMap;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use simple_logger::SimpleLogger;
use tiny_http::{Request, Response, Server, ServerConfig};
pub mod server;
pub mod website;
pub mod util;

const addr: &'static str = "127.0.0.1:8080";


fn main() {

  SimpleLogger::new().init().unwrap();
  
  log::info!("Starting server on: {}", &addr);

  let server = Server::http(addr).unwrap_or_else(|err|{
    log::error!("Failed to start server:\n{}", err);
    panic!();
  });
  let server = Arc::new(server);
  
  let threads = std::thread::available_parallelism().unwrap().get();

  let handles: HashMap<usize, JoinHandle<()>> = (0..=threads).map(|worker_id| {
    let server = Arc::clone(&server);
    
    log::info!("Starting thread: {}", &worker_id);
    
    let handle = thread::spawn(move || {
      for request in server.incoming_requests() {
        pollster::block_on(request_handler(&worker_id, request))
      }
    });

    (worker_id, handle)
  }).collect();

  for (_, handle) in handles {
    handle.join().unwrap();
  }
}


async fn request_handler(id: &usize, request: Request) {
  log::info!("Thread {} got a request: {}", id, request.url());
  
  let response = Response::from_string("Hello from server!");
  request.respond(response).unwrap();
}