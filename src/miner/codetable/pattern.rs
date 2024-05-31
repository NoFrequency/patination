/// A pattern is a bitsequence with a length.
/// Left aligned.
pub struct Pattern {
    bits: Vec<u8>,
    length: usize,
}

impl Pattern {
    /// Manually construct a new pattern
    pub fn new(bits: Vec<u8>, length: usize) -> Self {
        Self { bits, length }
    }

    /// Construct a 0-bit
    pub fn zero() -> Self {
        Self {
            bits: vec![0],
            length: 1,
        }
    }
    /// Construct a 1-bit
    pub fn one() -> Self {
        Self {
            bits: vec![128],
            length: 1,
        }
    }

    /// Concatenate two patterns.
    pub fn concat(&self, other: &Pattern) -> Pattern {
        let mut bits = self.bits.clone();
        // Append other bits to end of self.
        Self {
            bits,
            length: self.length + other.length,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pattern_construction_test() {
        let zero = Pattern::zero();
        assert_eq!(zero.bits[0], 0);
        let one = Pattern::one();
        assert_eq!(one.bits[0] >> 7, 1);

        let other = Pattern::new(vec![255, 0, 77], 24);
        assert_eq!(other.bits[0], 255);
        assert_eq!(other.bits[1], 0);
        assert_eq!(other.bits[2], 77);
    }
}
