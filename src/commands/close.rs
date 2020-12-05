use crate::utils::alerts;

pub fn run(code: i32) {
  alerts::done("bye, exit code 0");
  std::process::exit(code);
}
