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
  fn env() {
    for (key, val) in std::env::vars() {
      println!("{} is {} ....", key, val);
    }
  }

  #[test]
  fn build() {
    // 下载FFmpeg 内容
    let lib_path = path_resolve("source/FFmpeg".into());
    if Path::new(&lib_path).exists() == false {
      let git_url = "git@github.com:FFmpeg/FFmpeg.git";
      let workspace = "source";
      let cwd = path_resolve(workspace.into());
      let mut task_clone_ffmpeg_lib = Command::new("git");
      task_clone_ffmpeg_lib.arg("clone").arg(git_url);
      task_clone_ffmpeg_lib.current_dir(cwd.clone());
      task_clone_ffmpeg_lib
        .status()
        .expect(&format!("git clone {} --> failed", git_url));
    } else {
      let cwd = path_resolve(lib_path);
      let mut task_clone_ffmpeg_lib = Command::new("git");
      task_clone_ffmpeg_lib.arg("pull");
      task_clone_ffmpeg_lib.current_dir(cwd.clone());
      task_clone_ffmpeg_lib
        .status()
        .expect("source/FFmpeg 下执行 git pull 失败");
    }
    // 尝试编译 FFmpeg 根据当前 OS 和 arch
    let complier_lib_path = path_resolve("ff-output".into());
    if !Path::new(&complier_lib_path).exists() {
      let mut task_complie_ffmpeg_lib = Command::new("sh");
      if cfg!(target_arch = "arm64") {
        task_complie_ffmpeg_lib.arg("clone").arg("./build.arm.sh");
      } else {
        task_complie_ffmpeg_lib.arg("clone").arg("./build.sh");
      }
      let complie_cwd_path = "script";
      let complie_cwd = path_resolve(complie_cwd_path.into());
      task_complie_ffmpeg_lib.current_dir(complie_cwd.clone());
      task_complie_ffmpeg_lib
        .status()
        .expect("执行 ffmpeg 构建编译失败!");
    }
  }
}
