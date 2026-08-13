use std::f64::consts::PI;


pub struct PulseTrainGenerator {
    frequency: f64,
    sample_rate: u32,

    phase: f64
}
impl PulseTrainGenerator {
    pub fn new(frequency: f64, sample_rate: u32) -> Self {
        let mut vel = Self{
            frequency: 0.0,
            sample_rate: 0,

            phase: 0.0
        };
        vel.set(frequency, sample_rate);
        vel
    }
    pub fn set(&mut self, frequency: f64, sample_rate: u32) {
        self.frequency = frequency;
        self.sample_rate = sample_rate;
    }
    pub fn next(&mut self) -> f64 {
        self.phase += self.frequency / (self.sample_rate as f64);

        if self.phase >= 1.0 {
            self.phase -= 1.0;
            1.0
        }
        else {
            0.0
        }
    }
}
pub struct NoiseGenerator {
    state: u32,
}
impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            state: seed
        }
    }
    pub fn next(&mut self) -> f64 {
        let mut x = self.state;

        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;

        self.state = x;

        let normalized = x as f64 / u32::MAX as f64;

        normalized * 2.0 - 1.0 //-1.0 to 1.0
    }
    pub fn next_gaussian(&mut self) -> f64 {
        let mut sum = 0.0;

        for _ in 0..16 {
            sum += self.next();
        }

        sum / (16.0_f64 / 3.0).sqrt()
    }
}
pub struct NoiseIntegrator {
    previous: f64,
}
impl NoiseIntegrator {
    pub fn new() -> Self {
        Self {
            previous: 0.0,
        }
    }
    pub fn process(&mut self, x: f64) -> f64 {
        let y = x + self.previous;
        self.previous = y;
        y
    }
}












pub struct Resonator {
    a: f64,
    b: f64,
    c: f64,

    y1: f64,
    y2: f64
}
impl Resonator {
    pub fn new(frequency: f64, bandwidth: f64, sample_rate: u32) -> Self {
        let mut vel = Self { 
            a: 0.0, 
            b: 0.0, 
            c: 0.0, 
            y1: 0.0, 
            y2: 0.0
        };
        vel.set(frequency, bandwidth, sample_rate);
        vel
    }
    pub fn set(&mut self, frequency: f64, bandwidth: f64, sample_rate: u32) {
        let r: f64 = (-PI * bandwidth / (sample_rate as f64)).exp();
        self.c = -(r * r);
        self.b = 2.0 * r * (2.0 * PI * (frequency / (sample_rate as f64))).cos();
        self.a = 1.0 - self.b - self.c;
    }
    pub fn reset(&mut self) {
        self.y1 = 0.0;
        self.y2 = 0.0;
    }
    pub fn process(&mut self, x: f64) -> f64 {
        let y = self.a * x + self.b * self.y1 + self.c * self.y2;
        self.y2 = self.y1;
        self.y1 = y;
        y
    }
}
pub struct AntiResonator {
    a: f64,
    b: f64,
    c: f64,

    x1: f64,
    x2: f64,
}
impl AntiResonator {
    pub fn new(frequency: f64, bandwidth: f64, sample_rate: u32) -> Self {
        let mut vel = Self { 
            a: 0.0, 
            b: 0.0, 
            c: 0.0, 
            x1: 0.0, 
            x2: 0.0
        };
        vel.set(frequency, bandwidth, sample_rate);
        vel
    }
    pub fn set(&mut self, frequency: f64, bandwidth: f64, sample_rate: u32) {
        let r: f64 = (-PI * bandwidth / (sample_rate as f64)).exp();
        let c = -(r * r);
        let b = 2.0 * r * (2.0 * PI * (frequency / (sample_rate as f64))).cos();
        let a = 1.0 - b - c;
        self.a = 1.0 / a;
        self.b = -b / a;
        self.c = -c / a;
    }
    pub fn process(&mut self, x: f64) -> f64 {
        let y = self.a * x + self.b * self.x1 + self.c * self.x2;
        self.x2 = self.x1;
        self.x1 = x;
        y
    }
}



pub struct RadiationFilter {
    previous: f64,
}
impl RadiationFilter {
    pub fn new() -> Self {
        Self{
            previous: 0.0
        }
    }
    pub fn process(&mut self, x: f64) -> f64 {
        let y = x - self.previous;
        self.previous = x;
        y
    }
}
#[derive(Clone, Copy)]
pub struct Formant {
    pub frequency: f64,
    pub bandwidth: f64
}
#[derive(Clone, Copy)]
pub struct Vowel {
    pub f1: Formant,
    pub f2: Formant,
    pub f3: Formant
}
pub struct VoiceProfile {
    pub a: Vowel,
    pub i: Vowel,
    pub u: Vowel,
    pub e: Vowel,
    pub o: Vowel,
}



//vowel definitions
pub const MALE_A: Vowel = Vowel {
    f1: Formant { frequency: 700.0, bandwidth: 100.0 },
    f2: Formant { frequency: 1100.0, bandwidth: 100.0 },
    f3: Formant { frequency: 2600.0, bandwidth: 180.0 },
};

pub const MALE_I: Vowel = Vowel {
    f1: Formant { frequency: 300.0, bandwidth: 70.0 },
    f2: Formant { frequency: 2200.0, bandwidth: 100.0 },
    f3: Formant { frequency: 3000.0, bandwidth: 150.0 },
};

pub const MALE_U: Vowel = Vowel {
    f1: Formant { frequency: 350.0, bandwidth: 80.0 },
    f2: Formant { frequency: 1300.0, bandwidth: 100.0 },
    f3: Formant { frequency: 2700.0, bandwidth: 160.0 },
};

pub const MALE_E: Vowel = Vowel {
    f1: Formant { frequency: 500.0, bandwidth: 90.0 },
    f2: Formant { frequency: 1800.0, bandwidth: 110.0 },
    f3: Formant { frequency: 2700.0, bandwidth: 170.0 },
};

pub const MALE_O: Vowel = Vowel {
    f1: Formant { frequency: 500.0, bandwidth: 90.0 },
    f2: Formant { frequency: 900.0, bandwidth: 90.0 },
    f3: Formant { frequency: 2600.0, bandwidth: 170.0 },
};



pub const FEMALE_A: Vowel = Vowel {
    f1: Formant { frequency: 850.0, bandwidth: 120.0 },
    f2: Formant { frequency: 1350.0, bandwidth: 120.0 },
    f3: Formant { frequency: 3100.0, bandwidth: 200.0 },
};

pub const FEMALE_I: Vowel = Vowel {
    f1: Formant { frequency: 370.0, bandwidth: 90.0 },
    f2: Formant { frequency: 2700.0, bandwidth: 120.0 },
    f3: Formant { frequency: 3500.0, bandwidth: 180.0 },
};

pub const FEMALE_U: Vowel = Vowel {
    f1: Formant { frequency: 430.0, bandwidth: 100.0 },
    f2: Formant { frequency: 1600.0, bandwidth: 120.0 },
    f3: Formant { frequency: 3200.0, bandwidth: 190.0 },
};

pub const FEMALE_E: Vowel = Vowel {
    f1: Formant { frequency: 610.0, bandwidth: 110.0 },
    f2: Formant { frequency: 2200.0, bandwidth: 130.0 },
    f3: Formant { frequency: 3200.0, bandwidth: 200.0 },
};

pub const FEMALE_O: Vowel = Vowel {
    f1: Formant { frequency: 600.0, bandwidth: 110.0 },
    f2: Formant { frequency: 1100.0, bandwidth: 110.0 },
    f3: Formant { frequency: 3100.0, bandwidth: 200.0 },
};






pub const MALE_VOICE: VoiceProfile = VoiceProfile {
    a: MALE_A,
    e: MALE_E,
    i: MALE_I,
    o: MALE_O,
    u: MALE_U,
};

pub const FEMALE_VOICE: VoiceProfile = VoiceProfile {
    a: FEMALE_A,
    e: FEMALE_E,
    i: FEMALE_I,
    o: FEMALE_O,
    u: FEMALE_U,
};