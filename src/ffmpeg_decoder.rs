use ffmpeg_next::codec::context::Context;
use ffmpeg_next::format::input;
use ffmpeg_next::media::Type;

fn decode_video(path: &str) {
    ffmpeg_next::init().unwrap();
    let mut ictx = input(&path).unwrap();
    let input = ictx.streams().best(Type::Video).unwrap();
    let context = Context::from_parameters(input.parameters()).unwrap();
    let mut _decoder = context.decoder().video().unwrap();

    // Декодирование и обработка видео
}
