use crate::utils::alerts;

pub fn run() {
  alerts::done("bye, exit code 0");
  std::process::exit(0);
}
