/// A pattern is a bitsequence with a length.
/// Left aligned.
/// u8 structure: [bit | mask | bit | mask | bit | mask | bit | mask]
/// length includes masked out bits.
#[derive(Eq, Hash, PartialEq, Clone, Debug)]
pub struct Pattern {
    masked_bits: Vec<u8>,
    length: usize,
}

impl Pattern {
    /// Manually construct a new pattern
    pub fn new(masked_bits: Vec<u8>, length: usize) -> Self {
        Self {
            masked_bits,
            length,
        }
    }

    /// Construct a 0-bit
    pub fn zero() -> Self {
        Self {
            masked_bits: vec![0b01000000],
            length: 1,
        }
    }
    /// Construct a 1-bit
    pub fn one() -> Self {
        Self {
            masked_bits: vec![0b11000000],
            length: 1,
        }
    }

    pub fn get(&self, index: usize) -> Option<bool> {
        // OOB check?
        if (self.masked_bits[index / 4] & (0b01000000 >> ((index % 4) * 2))) != 0 {
            Some((self.masked_bits[index / 4] & (0b10000000 >> ((index % 4) * 2))) != 0)
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_construction_test() {
        let zero = Pattern::zero();
        assert_eq!(zero.masked_bits[0], 64);
        assert_eq!(zero.length, 1);
        let one = Pattern::one();
        assert_eq!(one.masked_bits[0], 128 + 64);
        assert_eq!(one.length, 1);

        let other = Pattern::new(vec![255, 0, 77], 12);
        assert_eq!(other.masked_bits[0], 255);
        assert_eq!(other.masked_bits[1], 0);
        assert_eq!(other.masked_bits[2], 77);
        assert_eq!(other.length, 12);
    }
    fn get_test() {
        let zero = Pattern::zero();
        assert_eq!(zero.get(0), Some(false));
        let one = Pattern::one();
        assert_eq!(one.get(0), Some(true));

        let other: Pattern = Pattern::new(vec![0b00011011], 4);
        assert_eq!(other.length, 4);
        assert_eq!(other.get(0), None);
        assert_eq!(other.get(1), Some(false));
        assert_eq!(other.get(2), None);
        assert_eq!(other.get(3), Some(true));
    }
}
