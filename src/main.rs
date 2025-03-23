extern crate ffmpeg_next as ffmpeg;
use base64::encode;
use ffmpeg::format::input;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::context::Context;
use ffmpeg::software::scaling::flag::Flags;
use ffmpeg::util::frame::video::Video;
use std::io::{self, BufRead, Write};
use std::path::Path;

fn play_video(file_path: &str) -> Result<(), ffmpeg::Error> {
    ffmpeg::init().unwrap();

    let mut ictx = input(&Path::new(file_path))?;
    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();

    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;

    let mut frame_index = 0;

    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;

                // Кодируем кадр в base64
                let frame_data = rgb_frame.data(0).to_vec();
                let frame_base64 = encode(&frame_data);
                println!("{}", frame_base64); // Отправляем кадр в stdout

                frame_index += 1;
            }
        }
    }

    Ok(())
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let file_path = line.unwrap();
        if let Err(e) = play_video(&file_path) {
            eprintln!("Error playing video: {}", e);
        }
    }
}
