extern crate rust_ffmpeg;

#[cfg(test)]
mod tests {
    use std::ffi;
    use std::ptr::null_mut;
    use rust_ffmpeg::gen::ffmpeg::root as ffmpeglib;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn play() {
        println!("{}", ffmpeglib::AV_CPU_FLAG_SSE);
        unsafe {
            ffmpeglib::avdevice_register_all();
            let path = "/Users/zhushijie/Desktop/m3u8-demo/test.mp4";
            let c_path = ffi::CString::new(path)
                .expect("CString::new failed")
                .into_raw();
            let txt_inputformat = ffi::CString::new("video4linux2")
                .expect("CString::new failed")
                .into_raw();
            let input_fmt = ffmpeglib::av_find_input_format(txt_inputformat);
            // let audio_stream_idx = -1;
            let mut ifmt_ctx: *mut ffmpeglib::AVFormatContext = ffmpeglib::avformat_alloc_context();
            let code = ffmpeglib::avformat_open_input(&mut ifmt_ctx, c_path, input_fmt, null_mut());
            if code < 0 {
                println!("找不到视频文件");
            } else {
                println!("视频打开成功");
            }
            println!("...");
        };
    }
}
