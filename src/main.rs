mod components;
use components::*;

mod misc;
use misc::*;

mod renderer;
use renderer::*;

//makes a list of resonators from a vowel
fn make_formants(vowel: Vowel, sample_rate: u32) -> (Resonator, Resonator, Resonator) {
    (
        Resonator::new(
            vowel.f1.frequency,
            vowel.f1.bandwidth,
            sample_rate,
        ),
        Resonator::new(
            vowel.f2.frequency,
            vowel.f2.bandwidth,
            sample_rate,
        ),
        Resonator::new(
            vowel.f3.frequency,
            vowel.f3.bandwidth,
            sample_rate,
        ),
    )
}

fn main() {
    //parameters
    let filename: &str = "output.wav";

    let f0: f64 = 220.0;
    let sample_rate: u32 = 44100;
    let seconds: f64 = 1.0;

    let target_peak: f32 = 1.0; //target peak controls volume, 0.0 - 1.0

    let voice: VoiceProfile = MALE_VOICE;

    let mut AV: f64;
    let mut AH: f64;

    let mut pulsetrain = PulseTrainGenerator::new(f0, sample_rate);
    let mut noise = NoiseGenerator::new(67);

    let mut glottal_filter = Resonator::new(0.0, 10000.0 / f0, sample_rate);
    let mut glottal_zero = AntiResonator::new(1500.0, 6000.0, sample_rate);
    let mut noise_integrator = NoiseIntegrator::new();

    let vowel = voice.a;

    let (mut f1, mut f2, mut f3) = make_formants(vowel, sample_rate);

    let mut f4: Resonator = Resonator::new(3300.0, 250.0, sample_rate);
    let mut f5: Resonator = Resonator::new(3750.0, 200.0, sample_rate);

    let mut radiation_filter = RadiationFilter::new();

    //generate samples
    let samples_count = (sample_rate as f64 * seconds) as usize;
    let mut samples: Vec<f32> = Vec::with_capacity(samples_count);

    let h_duration = 0.1;
    let a_duration = 0.5;
    let trans_duration = 0.005;

    let trans_samples = (trans_duration * sample_rate as f64) as usize;
    let h_norm_samples = ((h_duration - trans_duration) * sample_rate as f64) as usize;

    let a_samples = (a_duration * sample_rate as f64) as usize;     

    //voicing amplitude
    AV = 0.0;
    //aspiration amplitude
    AH = 60.0;

    /*  H  */
    f1.set(vowel.f1.frequency, 300.0, sample_rate);
    for _ in 0..h_norm_samples {
        let mut vel: f64 = noise.next_gaussian();

        vel = noise_integrator.process(vel);

        vel *= ah_to_amplitude(AH);

        vel = f5.process(vel);
        vel = f4.process(vel);
        vel = f3.process(vel);
        vel = f2.process(vel);
        vel = f1.process(vel);
        
        vel = radiation_filter.process(vel);
        
        samples.push(vel as f32);
    }

    let ah_start_amp = ah_to_amplitude(AH);
    let ah_end_amp = ah_to_amplitude(0.0);
    for i in 0..trans_samples {
        let t = i as f64 / (trans_samples-1) as f64;
        let lerp_amp = lerp(ah_start_amp, ah_end_amp, t);

        let mut vel: f64 = noise.next_gaussian();

        vel = noise_integrator.process(vel);

        vel *= lerp_amp;

        vel = f5.process(vel);
        vel = f4.process(vel);
        vel = f3.process(vel);
        vel = f2.process(vel);
        vel = f1.process(vel);
        
        vel = radiation_filter.process(vel);
        
        samples.push(vel as f32);
    }

    /*  A  */
    //voicing amplitude
    AV = 60.0;
    //aspiration amplitude
    AH = 0.0;
    f1.set(vowel.f1.frequency, vowel.f1.bandwidth, sample_rate);
    for _ in 0..a_samples {
        let mut vel: f64 = pulsetrain.next();

        vel *= av_to_amplitude(AV);
        vel *= f0;

        vel = glottal_filter.process(vel);
        vel *= 34.8207;
        vel = glottal_zero.process(vel);

        vel = f5.process(vel);
        vel = f4.process(vel);
        vel = f3.process(vel);
        vel = f2.process(vel);
        vel = f1.process(vel);
        
        vel = radiation_filter.process(vel);
        
        samples.push(vel as f32);
    }


    //normalize samples
    normalize_samples(&mut samples, target_peak);

    //play samples
    play_samples(&samples, sample_rate);

    //save samples
    save_to_wav(&samples, sample_rate, filename);
}
