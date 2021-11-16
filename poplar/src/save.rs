use std::ptr::null_mut;
use std::ffi::CString;
use rust_ffmpeg::gen::ffmpeg::root as ffmpeglib;

const PROJECT_PATH: &str = "/Users/zhushijie/Desktop/test/output";

pub fn saveframe(frame: *mut ffmpeglib::AVFrame, index: i32) {
    unsafe {
        let filepath = format!("{}/{}.jpg", PROJECT_PATH, index.to_string());
        println!("pic name is {}", filepath);
        let c_filepath = CString::new(filepath).unwrap().into_raw();
        let p_format_ctx: *mut ffmpeglib::AVFormatContext = ffmpeglib::avformat_alloc_context();
        let format = ffmpeglib::av_guess_format(
            CString::new("mjpeg").unwrap().into_raw(),
            null_mut(),
            null_mut(),
        );
        (*p_format_ctx).oformat = format;
        let write_res = ffmpeglib::avio_open(
            &mut (*p_format_ctx).pb,
            c_filepath,
            ffmpeglib::AVIO_FLAG_READ_WRITE as i32,
        );
        if write_res < 0 {
            println!("Couldn't open output file");
            return;
        }

        // let codec: *const ffmpeglib::AVCodec = ffmpeglib::avcodec_find_decoder((*(*p_format_ctx).oformat).video_codec);
        // let p_avstream = ffmpeglib::avformat_new_stream(p_format_ctx, codec);
        let avcodec = ffmpeglib::avcodec_find_encoder_by_name(CString::new("mjpeg").unwrap().into_raw());
        let p_avstream = ffmpeglib::avformat_new_stream(p_format_ctx, avcodec);
        if p_avstream == null_mut() {
            return;
        }
        let parameters = (*p_avstream).codecpar;
        (*parameters).codec_id = (*(*p_format_ctx).oformat).video_codec;
        (*parameters).codec_type = ffmpeglib::AVMediaType_AVMEDIA_TYPE_VIDEO;
        (*parameters).format = ffmpeglib::AVPixelFormat_AV_PIX_FMT_YUVJ420P;
        (*parameters).width = (*frame).width;
        (*parameters).height = (*frame).height;

        ffmpeglib::av_dump_format(p_format_ctx, 0, c_filepath, 1);

        let p_codec = ffmpeglib::avcodec_find_encoder((*(*p_avstream).codecpar).codec_id);
        if p_codec == null_mut() {
            println!("Could not find encoder.");
            return;
        }

        let p_codectx = ffmpeglib::avcodec_alloc_context3(p_codec);
        if p_codectx == null_mut() {
            println!("Could not allocate video codec context");
            return;
        }

        if ffmpeglib::avcodec_parameters_to_context(p_codectx, (*p_avstream).codecpar) < 0 {
            println!("Failed to copy,codec parameters to decoder context!");
            return;
        }

        (*p_codectx).time_base = ffmpeglib::AVRational { num: 1, den: 25 };

        let code_res = ffmpeglib::avcodec_open2(p_codectx, p_codec, null_mut());
        if code_res < 0 {
            println!("Could not open codec.");
            return;
        }

        ffmpeglib::avformat_write_header(p_format_ctx, null_mut());

        let y_size = (*frame).width * (*frame).height;
        let mut pkt: *mut ffmpeglib::AVPacket = ffmpeglib::av_packet_alloc();
        ffmpeglib::av_new_packet(pkt, y_size * 3);

        let mut ret = ffmpeglib::avcodec_send_frame(p_codectx, frame);
        if ret < 0 {
            println!("Send IFrame to decodec error....\n");
            return;
        }
        while ret >= 0 {
            ret = ffmpeglib::avcodec_receive_packet(p_codectx, pkt);
            if ret < 0 {
                println!("Error during encoding... \n");
                return;
            }
            ret = ffmpeglib::av_write_frame(p_format_ctx, pkt);
            if ret < 0 {
                println!("Write frame to output failed....\n");
                return;
            }
            ffmpeglib::av_packet_free(&mut pkt);
        }

        ffmpeglib::av_write_trailer(p_format_ctx);
        println!("Encode Successful");

        if p_avstream != null_mut() {
            ffmpeglib::avcodec_close(p_codectx);
        }
        ffmpeglib::avio_close((*p_format_ctx).pb);
        ffmpeglib::avformat_free_context(p_format_ctx);

        return;
    }
}


pub fn saveframe2(frame: *mut ffmpeglib::AVFrame, index: i32) -> i32 {
    unsafe {
        let filepath = format!("{}/{}.jpg", PROJECT_PATH, index.to_string());
        println!("pic name is {}", filepath);
        let c_filepath = CString::new(filepath).unwrap().into_raw();

        let mut pFormatCtx: *mut ffmpeglib::AVFormatContext = null_mut();
        let mut pCodecCtx: *mut ffmpeglib::AVCodecContext = null_mut();
        let mut video_st: *mut ffmpeglib::AVStream = null_mut();
        let mut pkt: *mut ffmpeglib::AVPacket = ffmpeglib::av_packet_alloc();

        let mut ret = 0;

        //根据文件后缀猜一个Format
        ffmpeglib::avformat_alloc_output_context2(&mut pFormatCtx, null_mut(), null_mut(), c_filepath);


        pCodecCtx = ffmpeglib::avcodec_alloc_context3(null_mut());
        (*pCodecCtx).codec_id = (*(*pFormatCtx).oformat).video_codec;//编码ID
        (*pCodecCtx).codec_type = ffmpeglib::AVMediaType_AVMEDIA_TYPE_VIDEO;
        (*pCodecCtx).pix_fmt = ffmpeglib::AVPixelFormat_AV_PIX_FMT_YUVJ420P;
        (*pCodecCtx).width = (*frame).width;
        (*pCodecCtx).height = (*frame).height;
        (*pCodecCtx).time_base.num = 1;
        (*pCodecCtx).time_base.den = 25;


        let pCodec = ffmpeglib::avcodec_find_encoder((*pCodecCtx).codec_id);
        if pCodec == null_mut() {
            println!("jpeg Codec not found. \n");
            return -1;
        }
        if ffmpeglib::avcodec_open2(pCodecCtx, pCodec, null_mut()) < 0 {
            println!("Could not open jpeg codec. \n");
            return -1;
        }

        //设置JPEG质量
        (*pCodecCtx).qcompress = 1 as f32; // 0~1.0, default is 0.5
        (*pCodecCtx).qmin = 2;
        (*pCodecCtx).qmax = 31;
        (*pCodecCtx).max_qdiff = 3;

        //给媒体文件创建一股流
        video_st = ffmpeglib::avformat_new_stream(pFormatCtx, null_mut());
        if video_st == null_mut() {
            return -1;
        }

        //Output some information
        ffmpeglib::av_dump_format(pFormatCtx, 0, c_filepath, 1);
        //Write Header
        ffmpeglib::avformat_write_header(pFormatCtx, null_mut());

        //预申请mjpeg压缩后的packet大小
        ffmpeglib::av_new_packet(pkt, (*pCodecCtx).width * (*pCodecCtx).height * 3);

        //Encode
        ret = ffmpeglib::avcodec_send_frame(pCodecCtx, frame);
        if ret == 0
        {
            while ffmpeglib::avcodec_receive_packet(pCodecCtx, pkt) == 0 {
                ret = ffmpeglib::av_write_frame(pFormatCtx, pkt);
            }
        }

        ffmpeglib::av_packet_unref(pkt);

        //Write Trailer
        ffmpeglib::av_write_trailer(pFormatCtx);

        ffmpeglib::avio_close((*pFormatCtx).pb);
        ffmpeglib::avformat_free_context(pFormatCtx);
        ffmpeglib::avcodec_close(pCodecCtx);
        ffmpeglib::avcodec_free_context(&mut pCodecCtx);
    }
    return 0;
}