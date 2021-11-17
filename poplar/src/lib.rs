mod save;

extern crate rust_ffmpeg;

#[cfg(test)]
mod tests {
    use std::ffi;
    use std::ptr::null_mut;
    use std::slice::from_raw_parts;
    use rust_ffmpeg::gen::ffmpeg::root as ffmpeglib;
    use rust_ffmpeg::gen::ffmpeg::root::{AV_TIME_BASE, AVCodecParameters, AVMediaType_AVMEDIA_TYPE_VIDEO};
    use crate::save::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn play() {
        println!("{}", ffmpeglib::AV_CPU_FLAG_SSE);
        unsafe {
            ffmpeglib::avdevice_register_all();
            let mut video_stream_id = 0 as usize;
            let path = "/Users/zhushijie/Desktop/test/c.mp4";
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
            let value = (*ifmt_ctx).duration;
            let trade = AV_TIME_BASE as i64;
            let durtime = value / trade;
            let ctx = *ifmt_ctx;
            println!("res is {},time is {}", code, durtime);
            println!("stream is {}", ctx.nb_streams);
            let stream_count = ctx.nb_streams;
            let streams = from_raw_parts((*ifmt_ctx).streams, stream_count as usize)
                .iter()
                .map(|x| (*x).as_ref().expect("not null"))
                .collect::<Vec<&ffmpeglib::AVStream>>();

            for (index, stream_ptr) in streams.iter().enumerate() {
                let acc: *mut AVCodecParameters = stream_ptr.codecpar;
                println!("codec_type is {},index is {}", (*acc).codec_type, index);
                if (*acc).codec_type == AVMediaType_AVMEDIA_TYPE_VIDEO {
                    video_stream_id = index;
                    let codec: *const ffmpeglib::AVCodec = ffmpeglib::avcodec_find_decoder((*acc).codec_id);
                    if codec == null_mut() {
                        println!("没有该类型的解码器!");
                        break;
                    }
                    let codec_ctx: *mut ffmpeglib::AVCodecContext = ffmpeglib::avcodec_alloc_context3(codec);
                    ffmpeglib::avcodec_parameters_to_context(codec_ctx, acc);
                    let res = ffmpeglib::avcodec_open2(codec_ctx, codec, null_mut());
                    if res != 0 {}
                    println!("解码器打开成功");
                    let packet: *mut ffmpeglib::AVPacket = ffmpeglib::av_packet_alloc();
                    let pframe: *mut ffmpeglib::AVFrame = ffmpeglib::av_frame_alloc();
                    let tr_frame: *mut ffmpeglib::AVFrame = ffmpeglib::av_frame_alloc();

                    (*tr_frame).format = (*codec_ctx).pix_fmt;
                    (*tr_frame).color_range = (*codec_ctx).color_range;
                    (*tr_frame).width = (*codec_ctx).width;
                    (*tr_frame).height = (*codec_ctx).height;

                    let picturesize = ffmpeglib::av_image_get_buffer_size(
                        (*codec_ctx).pix_fmt,
                        (*codec_ctx).width,
                        (*codec_ctx).height,
                        1,
                    ) as usize;

                    let buffer = ffmpeglib::av_malloc(picturesize as u64) as *mut u8;

                    ffmpeglib::av_image_fill_arrays(
                        (*tr_frame).data.as_mut_ptr(),
                        (*tr_frame).linesize.as_mut_ptr(),
                        buffer,
                        ffmpeglib::AVPixelFormat_AV_PIX_FMT_YUVJ420P,
                        (*codec_ctx).width,
                        (*codec_ctx).height,
                        1,
                    );

                    println!(
                        "width: {} height: {} pix_fmt: {}",
                        (*codec_ctx).width,
                        (*codec_ctx).height,
                        (*codec_ctx).pix_fmt
                    );

                    let mut pic_index = 1;


                    while ffmpeglib::av_read_frame(ifmt_ctx, packet) >= 0 {
                        let stream_index = (*packet).stream_index as usize;
                        if video_stream_id == stream_index {
                            let ret_send = ffmpeglib::avcodec_send_packet(codec_ctx, packet);
                            if ret_send != 0 {
                                println!("avodec send packet error");
                                continue;
                            }
                            let receiveFrame = ffmpeglib::avcodec_receive_frame(codec_ctx, pframe);
                            if receiveFrame != 0 {
                                println!("avcodec_receive_frame error");
                                continue;
                            }
                            let img_convert_ctx: *mut ffmpeglib::SwsContext = ffmpeglib::sws_getContext(
                                (*codec_ctx).width,
                                (*codec_ctx).height,
                                (*pframe).format,
                                (*codec_ctx).width,
                                (*codec_ctx).height,
                                ffmpeglib::AVPixelFormat_AV_PIX_FMT_YUVJ420P,
                                ffmpeglib::SWS_BICUBIC as i32,
                                null_mut(),
                                null_mut(),
                                null_mut(),
                            );

                            let h = ffmpeglib::sws_scale(
                                img_convert_ctx,
                                (*pframe).data.as_ptr() as *mut *const u8,
                                (*pframe).linesize.as_ptr(),
                                0,
                                (*pframe).height,
                                (*tr_frame).data.as_ptr(),
                                (*tr_frame).linesize.as_ptr(),
                            );
                            println!("重新计算的高端:{}", h);

                            if pic_index < 4 && h > 0 {
                                /*
                                 *  ffmpeg -i c.mp4 -pix_fmt yuv420p -an -y a.mp4 (手动执行转化命令)
                                 */
                                saveframe2(tr_frame, pic_index);
                            }
                            pic_index += 1;
                        }
                    }

                    ffmpeglib::av_frame_unref(pframe);
                    ffmpeglib::av_frame_unref(tr_frame);
                    ffmpeglib::avcodec_close(codec_ctx);
                    ffmpeglib::avformat_close_input(&mut ifmt_ctx);
                }
            }
        };
    }
}
