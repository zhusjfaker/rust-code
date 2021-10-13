// use std::process::Command;

fn main() {
  let dir = std::env::current_dir()
    .unwrap()
    .into_os_string()
    .into_string()
    .unwrap();

  std::env::set_var("$$fflib", dir);
  // Command::new("echo")
  //   .arg(dir + "....")
  //   .spawn()
  //   .expect("failed to spawn process");
  // println!("Hi!");
  // panic!("Nope");
}
