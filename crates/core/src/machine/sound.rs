//! The sound generator: 3 square waves, port FB (on, XOR mode), each channel's volume and
//! high-pass filter, resampled to the sound card's rate.

use super::*;

/// Channel volumes (F8 −6 dB, F9 −3 dB, FA 0 dB: resistor mixing traced on the PCB).
const SND_VOLUME: [f32; 3] = [0.5, 0.75, 1.0];
/// First-order high-pass on each channel's output (68 kΩ, 47 kΩ and 33 kΩ with 47 nF;
/// NESdev Wiki, ASIC registers): inaudible on plain tones, audible with XOR modulation.
const SND_HIGHPASS_HZ: [f32; 3] = [50.0, 72.0, 103.0];

/// 3 square-wave channels. The value written to F8–FA sets the period:
/// the wave flips every `63 - n` steps; 63 (or 0xFF, the reset value) = mute.
/// Port FB: bit 1 = sound on (0 mutes all three), bit 0 = XOR mode (channel 1 plays
/// 1 xor 2 and channel 2 plays 2 xor 3; channel 3 is unchanged).
pub(super) struct Sound {
    pub(super) period: [u8; 3],
    pub(super) tone: [bool; 3],
    pub(super) next_step: u64,
    pub(super) next_sample: f64,
    pub(super) sample_step: f64,
    pub(super) out: Vec<f32>,
    /// Channels muted in the debugger (not part of the state).
    pub(super) muted: [bool; 3],
    /// High-pass filters: coefficient, last input and last output of each channel (not
    /// part of the state: after loading one, the filters settle within a few ms).
    pub(super) hp_alpha: [f32; 3],
    pub(super) hp_in: [f32; 3],
    pub(super) hp_out: [f32; 3],
}

impl Sound {
    pub(super) fn new(sample_rate: u32) -> Self {
        let dt = 1.0 / sample_rate as f32;
        let hp_alpha = SND_HIGHPASS_HZ.map(|hz| {
            let rc = 1.0 / (2.0 * std::f32::consts::PI * hz);
            rc / (rc + dt)
        });
        Self { period: [0; 3], tone: [false; 3], next_step: SND_STEP, next_sample: 0.0, sample_step: SND_HZ / sample_rate as f64, out: Vec::new(), muted: [false; 3], hp_alpha, hp_in: [0.0; 3], hp_out: [0.0; 3] }
    }

    /// One output sample: each channel's square wave (0 or its volume) through its
    /// high-pass, then mixed.
    pub(super) fn level(&mut self, ctrl: u8) -> f32 {
        let t = self.tone;
        let out = if ctrl & 2 == 0 {
            [false; 3]
        } else if ctrl & 1 != 0 {
            [t[0] ^ t[1], t[1] ^ t[2], t[2]]
        } else {
            t
        };
        let mut sum = 0.0; // about −1.125..1.125
        for i in 0..3 {
            let x = if out[i] && !self.muted[i] { SND_VOLUME[i] } else { 0.0 };
            self.hp_out[i] = self.hp_alpha[i] * (self.hp_out[i] + x - self.hp_in[i]);
            self.hp_in[i] = x;
            sum += self.hp_out[i];
        }
        sum * 0.253
    }

    /// Advances up to `until` (units), generating the samples for that span.
    pub(super) fn advance(&mut self, until: u64, ch: &[u8; 3], ctrl: u8) {
        loop {
            let step_due = self.next_step <= until && (self.next_step as f64) <= self.next_sample;
            if step_due {
                for (i, &n) in ch.iter().enumerate() {
                    // HDL: compares the old value with (6'h3F - n) - 1 in 32 bits
                    let target = 62 - (n & 0x3F) as i32;
                    if self.period[i] as i32 == target {
                        self.period[i] = 0;
                        self.tone[i] = !self.tone[i];
                    } else {
                        self.period[i] = (self.period[i] + 1) & 0x3F;
                    }
                }
                self.next_step += SND_STEP;
            } else if self.next_sample <= until as f64 {
                let l = self.level(ctrl);
                self.out.push(l);
                self.next_sample += self.sample_step;
            } else {
                break;
            }
        }
    }
}
