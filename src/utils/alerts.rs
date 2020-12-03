use colored::*;

pub fn warning(msg: &str) {
  println!("{} {}", "Warning:".yellow().bold(), msg);
}

pub fn error(msg: &str) {
  println!("{} {}", "Error:".red().bold(), msg);
}

pub fn done(msg: &str) {
  println!("{} {}", "Done:".green().bold(), msg);
}
