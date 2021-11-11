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
        (*p_format_ctx).oformat = ffmpeglib::av_guess_format(
            CString::new("mjpeg").unwrap().into_raw(),
            null_mut(),
            null_mut(),
        );
        println!("codeid is {}", (*(*p_format_ctx).oformat).video_codec);
        let write_res = ffmpeglib::avio_open(
            &mut (*p_format_ctx).pb,
            c_filepath,
            ffmpeglib::AVIO_FLAG_READ_WRITE as i32,
        );
        if write_res < 0 {
            println!("Couldn't open output file");
            return;
        }

        let codec: *const ffmpeglib::AVCodec = ffmpeglib::avcodec_find_decoder((*(*p_format_ctx).oformat).video_codec);
        let p_avstream = ffmpeglib::avformat_new_stream(p_format_ctx, codec);
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
        let pkt: *mut ffmpeglib::AVPacket = ffmpeglib::av_packet_alloc();
        ffmpeglib::av_new_packet(pkt, y_size * 3);

        let mut got_picture = 0;
        let pic_decode_res = ffmpeglib::avcodec_encode_video2(p_codectx, pkt, frame, &mut got_picture);
        if pic_decode_res < 0 {
            println!("Encode Error");
            return;
        }
        if got_picture > 0 {
            ffmpeglib::av_write_frame(p_format_ctx, pkt);
        }
        ffmpeglib::av_free_packet(pkt);
        ffmpeglib::av_write_trailer(p_format_ctx);
        println!("Encode Successful");

        if p_avstream != null_mut() {
            ffmpeglib::avcodec_close((*p_avstream).codec);
        }
        ffmpeglib::avio_close((*p_format_ctx).pb);
        ffmpeglib::avformat_free_context(p_format_ctx);

        return;
    }
}