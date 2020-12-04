use dirs;
use std::fs;

pub fn paths() {
  let preference_dir = format!("{:?}/catsh", dirs::preference_dir().unwrap());
  fs::create_dir_all(&preference_dir);
}
