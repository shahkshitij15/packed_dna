//! A general-purpose genomics crate for dealing with DNA.

#![warn(missing_docs)]

use std::{convert::TryFrom, fmt::Display, str::FromStr};

// TODO: add a packed module with the PackedDna struct
//
// this struct must have the following:
// 1. A representation that is more memory efficient that simply storing a vector of `Nuc`
// 2. A FromStr implementation (should be case insensitive like the `Nuc` impl)
// 3. A `FromIterator` implementation to construct it from an iterator over `Nuc`s
// 4. A `fn get(&self, idx: usize) -> Nuc` getter for a particular nucleotide
//
// Make sure to unit test and document all elements
// Also, the internal representation of the PackedDna struct should be privately scoped

/// A nucleotide
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Nuc {
    /// Adenine
    A,
    /// Cytosine
    C,
    /// Guanine
    G,
    /// Thymine
    T,
}

/// An error that can occur when parsing a nucleotide.
#[derive(Debug, thiserror::Error)]
#[error("failed to parse nucleotide from {0}")]
pub struct ParseNucError<T: Display>(T);

impl TryFrom<char> for Nuc {
    type Error = ParseNucError<char>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value.to_ascii_uppercase() {
            'A' => Ok(Self::A),
            'C' => Ok(Self::C),
            'G' => Ok(Self::G),
            'T' => Ok(Self::T),
            _ => Err(ParseNucError(value)),
        }
    }
}

impl FromStr for Nuc {
    type Err = ParseNucError<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let upper = s.to_ascii_uppercase();
        match upper.as_str() {
            "A" => Ok(Self::A),
            "C" => Ok(Self::C),
            "G" => Ok(Self::G),
            "T" => Ok(Self::T),
            _ => Err(ParseNucError(upper)),
        }
    }
}

/// Module for packed DNA 
pub mod packed {
    use super::Nuc;
    
    
    /// A packed DNA sequence (Vector of Nuc)
    pub struct PackedDna(Vec<Nuc>);
    
    impl PackedDna {
        /// New Packed DNA
        pub fn from_iterator<I>(iterator: I) -> Self
        where
            I: IntoIterator<Item = Nuc>,
        {
            Self(iterator.into_iter().collect())
        }
    
        /// Get the Nuc at a particular index
        pub fn get(&self, idx: usize) -> Nuc {
            self.0[idx]
        }
    }

    impl IntoIterator for PackedDna {
        type Item = Nuc;
        type IntoIter = std::vec::IntoIter<Nuc>;
    
        fn into_iter(self) -> Self::IntoIter {
            self.0.into_iter()
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::{Nuc, PackedDna};
    use std::convert::TryFrom;
    
    #[test]
    fn packed_dna_from_iterator() {
        let nucs = vec![Nuc::A, Nuc::C, Nuc::G, Nuc::T];
        let packed_dna = PackedDna::from_iterator(nucs.iter().cloned());
        assert_eq!(packed_dna.get(0), Nuc::A);
        assert_eq!(packed_dna.get(1), Nuc::C);
        assert_eq!(packed_dna.get(2), Nuc::G);
        assert_eq!(packed_dna.get(3), Nuc::T);
    }

    #[test]
    fn tryfrom_char() {
        let c: char = 'A';
        let nuc = Nuc::try_from(c).unwrap();
        assert_eq!(nuc, Nuc::A);
    }

    #[test]
    fn fromstr() {
        let s: String = String::from_str("A");
        let nuc = Nuc::from_str(s).unwrap();
        assert_eq!(nuc, Nuc::A)
    }
}
