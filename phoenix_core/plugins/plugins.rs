pub trait Plugin {
  fn build(&self, app: &mut PhoenixEngine /* PhoenixApplication */);
}