use rodio::buffer::SamplesBuffer;
use rodio::{DeviceSinkBuilder, Player};
use std::num::NonZero;


//linear interpolating two values, t is in range of 0.0 - 1.0
pub fn lerp(start: f64, end: f64, t: f64) -> f64 {
    start + (end-start) * t
}

//relative db conversion to amplitude
pub fn db_to_amplitude(db: f64) -> f64 {
    10.0_f64.powf(db / 20.0)
}

const AV_CALIBRATION_DB: f64 = -72.0;
const AH_CALIBRATION_DB: f64 = -102.0;
pub fn av_to_amplitude(av_db: f64) -> f64 {
    if av_db <= 0.0 {
        0.0
    }
    else {
        db_to_amplitude(
            av_db + AV_CALIBRATION_DB
        )
    }
}
pub fn ah_to_amplitude(ah_db: f64) -> f64 {
    if ah_db <= 0.0 {
        0.0
    }
    else {
        db_to_amplitude(
            ah_db + AH_CALIBRATION_DB
        )
    }
}


//normalizes samples to a peak volume
pub fn normalize_samples(samples: &mut Vec<f32>, target_peak: f32) {
    //find peak in samples
    let peak = samples
        .iter()
        .map(|x| x.abs())
        .fold(0.0_f32, f32::max);

    //calculate gain for all samples so peak sample matches the desired peak
    let gain = target_peak / peak;
    
    //apply gain to all samples
    for sample in samples {
        *sample *= gain;
    }
}






















//play samples
pub fn play_samples(samples: &[f32], sample_rate: u32) {
    let output = DeviceSinkBuilder::open_default_sink().expect("Could not open audio output");

    let player = Player::connect_new(&output.mixer());

    let source = SamplesBuffer::new(
        NonZero::new(1).unwrap(),           // 1 channel = mono
        NonZero::new(sample_rate).unwrap(),
    samples,
    );

    player.append(source);

    // Keep the program alive until playback finishes.
    player.sleep_until_end();
}

//save samples to wav file
pub fn save_to_wav(samples: &[f32], sample_rate: u32, filename: &str) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(filename, spec).expect("Failed to create wav file");
    for sample in samples {
        let sample = sample.clamp(-1.0, 1.0);
    
        let sample_i16 = (sample * i16::MAX as f32) as i16;
    
        writer.write_sample(sample_i16).expect("Failed to write sample");
    }
    writer.finalize().expect("Failed to finalize WAV file");
}

