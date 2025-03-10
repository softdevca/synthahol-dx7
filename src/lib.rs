//! The [DX7](https://www.vintagesynth.com/yamaha/dx7.php) is a frequency
//! modulation (FM) hardware synth by Yamaha.
//!
//! A summary of the file format can be found in the
//! [Dexed repository](https://github.com/asb2m10/dexed/blob/master/Documentation/sysex-format.txt)
//!
//! The series
//! [Yamaha DX7 chip reverse-engineering](https://www.righto.com/2021/12/yamaha-dx7-chip-reverse-engineering.html)
//! by Ken Sherriff is a useful reference on the hardware.

use std::fmt::{Display, Formatter};

pub use algorithms::*;
pub use envelope::*;
pub use format::Format;
pub use read::*;

mod algorithms;
mod envelope;
mod format;
mod read;

const SYSEX_HEADER: [u8; 6] = [0xF0, 0x43, 0x00, 0x09, 0x20, 0x00];

pub type OperatorId = u8;

pub struct Hardware;

impl Hardware {
    /// The DX7 had 16 voice polyphony.
    pub const POLYPHONY: u32 = 16;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PresetName(String);

impl PresetName {
    /// A preset name has a fixed length
    pub const MAX_LENGTH: usize = 10;

    /// Normalize and trim the preset name. Unsupported characters are replaced
    /// with a space.
    ///
    /// # Example
    ///
    /// ```
    /// use synthahol_dx7::PresetName;
    /// assert_eq!("", PresetName::from_lossy(&[]).to_string());
    /// assert_eq!(" AbC", PresetName::from_lossy(" AbC ".as_bytes()).to_string());
    /// assert_eq!("! 8X", PresetName::from_lossy("! 8X".as_bytes()).to_string());
    /// assert_eq!("ABC def", PresetName::from_lossy("ABC\x07def".as_bytes()).to_string());
    /// assert_eq!("abcdefghij", PresetName::from_lossy("abcdefghijklmnopqrstuvwxyz".as_bytes()).to_string());
    /// ```
    pub fn from_lossy(data: &[u8]) -> PresetName {
        let ascii = data
            .iter()
            .map(|c| match c & 0x7F {
                c if (0x20..0x7f).contains(&c) => c, // Printable ASCII range
                _ => b' ',
            })
            .take(PresetName::MAX_LENGTH)
            .collect::<Vec<u8>>();
        PresetName(String::from_utf8_lossy(&ascii).trim_end().to_string())
    }
}

impl Default for PresetName {
    fn default() -> Self {
        PresetName("INIT VOICE".to_owned())
    }
}

impl Display for PresetName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Waveform {
    Triangle = 0,
    SawDown = 1,
    SawUp = 2,
    Square = 3,
    Sine = 4,
    SampleAndHold = 5,
}

impl TryFrom<u8> for Waveform {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Waveform::Triangle),
            1 => Ok(Waveform::SawDown),
            2 => Ok(Waveform::SawUp),
            3 => Ok(Waveform::Square),
            4 => Ok(Waveform::Sine),
            5 => Ok(Waveform::SampleAndHold),
            _ => Err("Unknown waveform {value}"),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum OperatorMode {
    Fixed = 0,
    Ratio = 1,
}

impl Display for OperatorMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use OperatorMode::*;
        let txt = match self {
            Fixed => "Fixed",
            Ratio => "Ratio",
        };
        f.write_str(txt)
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Operator {
    // In the DX7 the operator ON/OFF state is not stored in the preset and
    // is only used in parameter change sysex messages while editing a voice.
    pub envelope: Envelope,
    pub scaling_break_point: u8,
    pub scaling_left_depth: u8,
    pub scaling_right_depth: u8,
    pub scaling_left_curve: u8,
    pub scaling_right_curve: u8,

    // -7 to 7. Stored as 0-14 in the preset.
    pub detune: i8,

    pub rate_scaling: u8,
    pub velocity_sensitivity: u8,
    pub modulation_sensitivity: u8,
    pub output_level: u8,
    pub mode: OperatorMode,
    pub frequency_course: u8,
    pub frequency_fine: u8,
}

impl Operator {
    /// Clamp all parameters to valid ranges.
    fn normalize(&self) -> Self {
        Self {
            scaling_break_point: self.scaling_break_point.clamp(0, 99),
            scaling_left_depth: self.scaling_left_depth.clamp(0, 99),
            scaling_right_depth: self.scaling_right_depth.clamp(0, 99),
            scaling_left_curve: self.scaling_left_curve.clamp(0, 3),
            scaling_right_curve: self.scaling_right_curve.clamp(0, 3),
            detune: self.detune.clamp(0, 14),
            rate_scaling: self.rate_scaling.clamp(0, 7),
            velocity_sensitivity: self.velocity_sensitivity.clamp(0, 7),
            modulation_sensitivity: self.modulation_sensitivity.clamp(0, 3),
            output_level: self.output_level.clamp(0, 99),
            frequency_course: self.frequency_course.clamp(0, 31),
            frequency_fine: self.frequency_fine.clamp(0, 99),
            ..*self
        }
    }
}

impl Default for Operator {
    fn default() -> Self {
        // The last envelope generator has a different default level according to
        // the DX7 II manual.
        let mut envelope = Envelope::default();
        envelope.levels[envelope.levels.len() - 1] = 0;

        Operator {
            envelope,
            scaling_break_point: 39,
            scaling_left_depth: 0,
            scaling_right_depth: 0,
            scaling_left_curve: 0,
            scaling_right_curve: 0,
            detune: 0,
            rate_scaling: 0,
            velocity_sensitivity: 0,
            modulation_sensitivity: 0,
            output_level: 0,
            mode: OperatorMode::Fixed,
            frequency_course: 1,
            frequency_fine: 0,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Preset {
    pub name: PresetName,
    pub operators: [Operator; Preset::OPERATOR_COUNT],
    pub pitch_envelope: Envelope,
    pub algorithm_id: AlgorithmId,

    #[doc(alias = "osc phase init")]
    pub oscillator_key_sync: bool,
    pub feedback_level: u8,
    pub lfo_speed: u8,
    pub lfo_delay: u8,
    pub lfo_pitch_mod_depth: u8,
    pub lfo_pitch_mod_sensitivity: u8,
    pub lfo_amplitude_mod_depth: u8,
    pub lfo_waveform: Waveform,
    pub lfo_key_sync: bool,
    pub transpose: u8,
}

impl Preset {
    const OPERATOR_COUNT: usize = 6;

    /// Clamp all parameters to valid ranges.
    fn normalize(&self) -> Self {
        // Normalization is done outside of reading to enable reuse.
        Preset {
            name: self.name.clone(),
            operators: self.operators.map(|operator| operator.normalize()),
            pitch_envelope: self.pitch_envelope.normalize(),
            algorithm_id: self.algorithm_id.clamp(0, 31),
            oscillator_key_sync: self.oscillator_key_sync,
            feedback_level: self.feedback_level.clamp(0, 7),
            lfo_speed: self.lfo_speed.clamp(0, 99),
            lfo_delay: self.lfo_delay.clamp(0, 99),
            lfo_pitch_mod_depth: self.lfo_pitch_mod_depth.clamp(0, 99),
            lfo_pitch_mod_sensitivity: self.lfo_pitch_mod_sensitivity.clamp(0, 99),
            lfo_amplitude_mod_depth: self.lfo_amplitude_mod_depth.clamp(0, 99),
            lfo_waveform: self.lfo_waveform,
            lfo_key_sync: self.lfo_key_sync,
            transpose: self.transpose.clamp(0, 48),
        }
    }
}

impl Default for Preset {
    fn default() -> Self {
        let mut operators = [Operator::default(); Preset::OPERATOR_COUNT];
        operators[0] = Operator {
            output_level: 99,
            ..Default::default()
        };

        let pitch_envelope = Envelope::from_rate_and_level(99, 50);
        Preset {
            name: PresetName::default(),
            operators,
            pitch_envelope,
            algorithm_id: 0,
            oscillator_key_sync: true,
            feedback_level: 0,
            lfo_speed: 35,
            lfo_delay: 0,
            lfo_pitch_mod_depth: 0,
            lfo_pitch_mod_sensitivity: 3,
            lfo_amplitude_mod_depth: 0,
            lfo_waveform: Waveform::Triangle,
            lfo_key_sync: true,
            transpose: 24,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    pub(crate) fn test_data_path(components: &[&str]) -> PathBuf {
        let mut parts = vec!["tests"];
        parts.extend_from_slice(components);
        parts.iter().collect::<PathBuf>()
    }

    #[test]
    fn default() {
        let preset = Preset::default();
        assert_eq!(preset, preset);

        // The defaults must already be in range so normalizing shouldn't have
        // an effect.
        assert_eq!(preset, preset.normalize());

        // Pitch envelope generators.
        assert_eq!(99, preset.pitch_envelope.rates[0]);
        assert_eq!(50, preset.pitch_envelope.levels[0]);

        // General parameters.
        assert_eq!(0, preset.algorithm_id);
        assert!(preset.lfo_key_sync);
        assert!(preset.oscillator_key_sync);
        assert_eq!(35, preset.lfo_speed);
        assert_eq!(3, preset.lfo_pitch_mod_sensitivity);
        assert_eq!(24, preset.transpose);
        assert_eq!("INIT VOICE", preset.name.to_string());

        // Only the first operator has an output level
        assert_eq!(99, preset.operators[0].output_level);
        assert_eq!(0, preset.operators.last().unwrap().output_level);

        // Only the last envelope generator has a level of zero.
        assert_eq!(99, preset.operators[0].envelope.levels[0]);
        assert_eq!(0, *preset.operators[0].envelope.levels.last().unwrap());

        // General operator parameters.
        assert_eq!(0, preset.operators[0].rate_scaling);
        assert_eq!(39, preset.operators[0].scaling_break_point);
        assert_eq!(0, preset.operators[0].detune);
        assert_eq!(OperatorMode::Fixed, preset.operators[0].mode);
        assert_eq!(1, preset.operators[0].frequency_course);
        assert_eq!(0, preset.operators[0].frequency_fine);
    }

    #[test]
    fn normalize() {
        let preset = Preset {
            feedback_level: 123,
            lfo_delay: 100,
            transpose: 200,
            ..Default::default()
        }
        .normalize();
        assert_eq!(7, preset.feedback_level);
        assert_eq!(99, preset.lfo_delay);
        assert_eq!(48, preset.transpose);
    }
}
