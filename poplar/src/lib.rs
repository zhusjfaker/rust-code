extern crate rust_ffmpeg;

#[cfg(test)]
mod tests {

  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn play() {
    println!("{}", rust_ffmpeg::gen::ffmpeg::root::AV_CPU_FLAG_SSE);
  }
}
