#[cfg(test)]
mod tests {

  use std::ffi::OsString;
  use std::path::Path;
  use std::process::Command;

  /**
   * 计算项目的全局目录
   */
  fn path_resolve(path: String) -> String {
    let work_cwd = env!("CARGO_MANIFEST_DIR");
    let os_work_cwd = OsString::from(work_cwd);
    return Path::new(&os_work_cwd)
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
    let complier_lib_path = path_resolve(String::from("ff-output"));
    if !Path::new(&complier_lib_path).exists() {
      let mut task_complie_ffmpeg_lib = Command::new("sh");
      if cfg!(target_arch = "aarch64") {
        task_complie_ffmpeg_lib.arg("./build.arm.sh");
      } else {
        task_complie_ffmpeg_lib.arg("./build.sh");
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
