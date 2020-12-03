use std::process::Command;

pub fn run() {
  Command::new("clear").status().unwrap();
}
