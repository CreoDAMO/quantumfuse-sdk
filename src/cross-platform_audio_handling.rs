#[cfg(feature = "audio")]
#[cfg(target_os = "linux")]
mod audio_linux {
    use alsa::pcm::{PCM, HwParams, Format, Access};
    use std::error::Error;

    pub fn play_audio() -> Result<(), Box<dyn Error>> {
        let pcm = PCM::new("default", alsa::Direction::Playback, false)?;
        let hwp = HwParams::any(&pcm)?;
        hwp.set_format(Format::s16_le)?;
        hwp.set_access(Access::RWInterleaved)?;
        hwp.set_channels(2)?;
        pcm.hw_params(&hwp)?;

        println!("🎵 Playing Audio via ALSA on Linux...");
        Ok(())
    }
}

#[cfg(not(target_os = "linux"))]
mod audio_fallback {
    use rodio::{OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;

    pub fn play_audio() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let file = File::open("sample.mp3").unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.sleep_until_end();
        println!("🎵 Playing Audio via Rodio on non-Linux systems...");
    }
}

pub fn play_music() {
    #[cfg(all(feature = "audio", target_os = "linux"))]
    audio_linux::play_audio().unwrap();

    #[cfg(not(target_os = "linux"))]
    audio_fallback::play_audio();
}
