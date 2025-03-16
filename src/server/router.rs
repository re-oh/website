




pub struct Router<F> where F: AsyncFnMut()->i32 {
  func: F
}