#[cfg(test)]
mod tests {

  use std::path::Path;
  use std::process::Command;

  /**
   * 计算项目的全局目录
   */
  fn path_resolve(path: String) -> String {
    return std::env::current_dir()
      .unwrap()
      .join(path)
      .into_os_string()
      .into_string()
      .unwrap();
  }

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn build() {
    let lib_path = path_resolve("/source/FFmpeg".into());
    if !Path::new(&lib_path).exists() {
      let git_url = "git@github.com:FFmpeg/FFmpeg.git";
      let mut task_clone_ffmpeg_lib = Command::new("git");
      task_clone_ffmpeg_lib.arg("clone").arg(git_url);
    }
  }
}
