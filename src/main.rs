mod music_play;
mod gui;
mod audio_process;

use music_play::music_play::Playback;
use rodio::{Decoder, OutputStream, Sink};
use symphonia::{
    core::{
        codecs::DecoderOptions, formats::FormatOptions, io::MediaSourceStream,
        meta::MetadataOptions, probe::Hint,
    },
    default::get_codecs,
};

fn main() {
    //  读取音频文件
    let path = std::path::Path::new("./wav/Noa_CS_Ins.wav");
    let file = std::fs::File::open(path).unwrap();
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    //  探测音频文件格式
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &format_opts, &metadata_opts)
        .unwrap();
    let mut format = probed.format;

    //  获取编码参数和采样率
    let track = format
        .default_track()
        .ok_or_else(|| symphonia::core::errors::Error::DecodeError("No default track found"))
        .unwrap();
    let codec_params = &track.codec_params;
    let sample_rate = codec_params.sample_rate.unwrap_or(44100);
    let channels = codec_params.channels.unwrap_or_default().count();
    println!("声道数：{}", channels);

    let mut decoder = get_codecs()
        .make(&codec_params, &DecoderOptions::default())
        .unwrap();



    // // 创建音频输出流
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // let file = std::fs::File::open(path).unwrap();
    // let source = Decoder::new(file).unwrap();

    // sink.append(source);
    // sink.set_volume(0.2);

    // sink.sleep_until_end();

    let mut playback = Playback::new(1.0, 0.2);
    playback.list_append(path);
    playback.play();

}
