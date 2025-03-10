use std::fs::File;
use std::io::{BufReader, Error, ErrorKind, Read};
use std::path::Path;

use crate::*;

/// Compute a masked 2's complement checksum.
fn checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |sum, c| sum.wrapping_sub(*c)) & 0x7F
}

/// Banks are collection of presets
pub struct Bank;

impl Bank {
    pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<Preset>, Error> {
        let input = File::open(&path)?;
        let mut reader = BufReader::new(input);
        Self::read(
            &mut reader,
            Some(path.as_ref().to_string_lossy().to_string()),
        )
    }

    pub fn read<R: Read>(reader: &mut R, _name: Option<String>) -> Result<Vec<Preset>, Error> {
        // Header
        let mut header = [0; SYSEX_HEADER.len()];
        reader.read_exact(&mut header)?;
        if header != SYSEX_HEADER {
            return Err(Error::new(ErrorKind::InvalidData, "Incorrect header"));
        }

        // Body
        let mut body = [0; 4096]; // Length is hard coded in the header
        reader.read_exact(&mut body)?;

        // Body checksum
        let mut byte_buf = [0; 1];
        reader.read_exact(&mut byte_buf)?;
        let expected_checksum = byte_buf[0];
        let computed_checksum = checksum(&body);
        if computed_checksum != expected_checksum {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!(
                    "Computed checksum {computed_checksum} does not match expected checksum {expected_checksum}"
                ),
            ));
        }

        // Verify the end of SysEx marker
        reader.read_exact(&mut byte_buf)?;
        if byte_buf[0] != 0xF7 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Missing End of SysEx marker",
            ));
        }

        // Presets
        let mut presets = Vec::with_capacity(32);
        for packed_preset in body.chunks(128) {
            // Going directly to a String is unsafe because the name bytes may
            // be garbage.
            let name = PresetName::from_lossy(&packed_preset[118..127]);

            // Operators
            let mut operators = [Operator::default(); Preset::OPERATOR_COUNT];
            for operator_index in 0..operators.len() {
                let packed_operator =
                    &packed_preset[(operator_index * 17)..(operator_index + 1) * 17];

                // Envelope generators
                let rates = &packed_operator[0..4];
                let levels = &packed_operator[rates.len()..(rates.len() + 4)];
                let envelope =
                    Envelope::try_from_rates_and_levels(rates, levels).expect("envelope");

                let scaling_break_point = packed_operator[8];
                let scaling_left_depth = packed_operator[9];
                let scaling_right_depth = packed_operator[10];
                let scaling_left_curve = packed_operator[11] & 0b0011;
                let scaling_right_curve = (packed_operator[11] & 0b1100) >> 2;

                // -7 to 7 stored as 0-14 in the preset
                let detune = 0_i8 - ((packed_operator[12] & 0b1111000) >> 3) as i8;

                let rate_scaling = packed_operator[12] & 0b0000111; // 0-7
                let velocity_sensitivity = (packed_operator[13] & 0b0011100) >> 2; // 0-7
                let modulation_sensitivity = packed_operator[13] & 0b0000011; // 0-3
                let output_level = packed_operator[14]; // 0-99

                let mode = if packed_operator[15] & 0b0000001 == 0 {
                    OperatorMode::Fixed
                } else {
                    OperatorMode::Ratio
                };

                let frequency_course = (packed_operator[15] & 0b0111110) >> 1; // 0-31
                let frequency_fine = packed_operator[16]; // 0-99

                operators[operator_index] = Operator {
                    envelope,
                    scaling_break_point,
                    scaling_left_depth,
                    scaling_right_depth,
                    scaling_left_curve,
                    scaling_right_curve,
                    detune,
                    rate_scaling,
                    velocity_sensitivity,
                    modulation_sensitivity,
                    output_level,
                    mode,
                    frequency_course,
                    frequency_fine,
                }
            }
            operators.reverse(); // Stored last-operator-first in the file

            // Pitch envelope generators
            let pitch_env_rates_base = 102;
            let rates = &packed_preset[pitch_env_rates_base..(pitch_env_rates_base + 4)];
            let pitch_env_levels_base = pitch_env_rates_base + rates.len();
            let levels =
                &packed_preset[pitch_env_levels_base..(pitch_env_levels_base + rates.len())];
            let pitch_envelope =
                Envelope::try_from_rates_and_levels(rates, levels).expect("pitch envelope");

            let algorithm = packed_preset[110] as AlgorithmId;
            let oscillator_key_sync = (packed_preset[111] & 0b0001000) >> 4 == 1;
            let feedback_level = packed_preset[111] & 0b0000111;
            let lfo_speed = packed_preset[112];
            let lfo_delay = packed_preset[113];
            let lfo_pitch_mod_depth = packed_preset[114];
            let lfo_amplitude_mod_depth = packed_preset[115];
            let lfo_pitch_mod_sensitivity = (packed_preset[116] & 0b1110000) >> 4;
            let lfo_waveform = Waveform::try_from((packed_preset[116] & 0b0001110) >> 1)
                .map_err(|msg| Error::new(ErrorKind::InvalidData, msg))?;
            let lfo_key_sync = packed_preset[116] & 0b0000001 == 1;
            let transpose = packed_preset[117];

            let preset = Preset {
                name,
                operators,
                pitch_envelope,
                algorithm_id: algorithm,
                oscillator_key_sync,
                feedback_level,
                lfo_speed,
                lfo_delay,
                lfo_pitch_mod_depth,
                lfo_amplitude_mod_depth,
                lfo_pitch_mod_sensitivity,
                lfo_waveform,
                lfo_key_sync,
                transpose,
            }
            .normalize();
            presets.push(preset);
        }
        Ok(presets)
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::test_data_path;

    use super::*;

    #[test]
    fn checksums() {
        assert_eq!(0, checksum(&[]));
        assert_eq!(0, checksum(&[0,]));
        assert_eq!(86, checksum(&[42,]));
        assert_eq!(113, checksum(&[1, 2, 3, 4, 5,]));
        assert_eq!(94, checksum(&[100, 20, 30, 40, 100,]));
    }

    #[test]
    fn factory_bank() {
        let presets = Bank::read_file(test_data_path(&["rom1a.syx"])).unwrap();
        assert_eq!(presets.len(), 32);

        let preset = presets.first().unwrap();
        assert_eq!("BRASS   1", preset.name.to_string());
        assert_eq!(21, preset.algorithm_id);
        assert_eq!(Waveform::Sine, preset.lfo_waveform);

        let op1 = preset.operators[0];
        assert_eq!(0, op1.detune);

        let op6 = preset.operators[5];
        assert_eq!(0, op6.detune);
    }
}
