/// A four stage rate/level envelope
///
/// # Resources
/// * [Discussion about levels and timing](https://groups.google.com/g/music-synthesizer-for-android/c/QD2KGEj7QIk?pli=1)
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Envelope {
    pub rates: [u8; Envelope::SEGMENT_COUNT],
    pub levels: [u8; Envelope::SEGMENT_COUNT],
}

impl Envelope {
    const SEGMENT_COUNT: usize = 4;

    /// Create a new envelope where each segment has the same rate and level.
    pub(crate) fn from_rate_and_level(rate: u8, level: u8) -> Self {
        Self {
            rates: [rate; Envelope::SEGMENT_COUNT],
            levels: [level; Envelope::SEGMENT_COUNT],
        }
    }

    fn from_rates_and_levels(
        rates: [u8; Envelope::SEGMENT_COUNT],
        levels: [u8; Envelope::SEGMENT_COUNT],
    ) -> Self {
        Self { rates, levels }
    }

    /// Arguments must be exactly [`Envelope::SEGMENT_COUNT`] long.
    pub(crate) fn try_from_rates_and_levels(rates: &[u8], levels: &[u8]) -> Option<Self> {
        (rates.len() == Envelope::SEGMENT_COUNT && levels.len() == Envelope::SEGMENT_COUNT).then(
            || {
                let mut fixed_rates = [0_u8; Envelope::SEGMENT_COUNT];
                fixed_rates.copy_from_slice(rates);
                let mut fixed_levels = [0_u8; Envelope::SEGMENT_COUNT];
                fixed_levels.copy_from_slice(levels);
                Self::from_rates_and_levels(fixed_rates, fixed_levels)
            },
        )
    }

    /// Clamp all parameters to valid ranges.
    pub(crate) fn normalize(&self) -> Self {
        Self {
            rates: self.rates.map(|rate| rate.clamp(0, 99)),
            levels: self.levels.map(|level| level.clamp(0, 99)),
        }
    }
}

impl Default for Envelope {
    fn default() -> Self {
        Self {
            rates: [99, 99, 99, 99],
            levels: [99, 99, 99, 99],
        }
    }
}
