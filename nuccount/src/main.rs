// TODO: implement a nucleotide counter
//
// command line argument parsing has been provided
// you must use the PackedDna struct you previously implemented
// if there is any functionality you would like to add to PackedDna feel free to do so in the DNA
// crate
//
// If run with `nuccount --dna ACGTTT" it should print the following to stdout:
// ```
// Input: ACGTTT
//
// A: 1
// C: 1
// G: 1
// T: 3
// ```
//
// be sure to exit with informative error messages if the input is invalid

use structopt::StructOpt;
use std::convert::TryFrom;
use dna::packed::PackedDna;
use dna::Nuc;

/// Count the number of occurrences of each nucleotide in the provided DNA.
#[derive(Debug, StructOpt)]
struct Opts {
    /// The DNA sequence for which we should retrieve a nucleotide count.
    ///
    /// It is case insensitive but only nucleotides A, C, G and T are supported.
    #[structopt(short = "d", long, required = true)]
    dna: String
}

fn main() {
    let opts = Opts::from_args();
    let dna = opts.dna;
    println!("Input: {}", &dna);

    let packed_dna = PackedDna::from_iterator(
        dna.chars().map(|c| Nuc::try_from(c).expect("Invalid nucleotide")),
    );

    // Count occurrences of each nucleotide
    let mut counts = [0; 4]; // A, C, G, T

    for nuc in packed_dna {
        match nuc {
            Nuc::A => counts[0] += 1,
            Nuc::C => counts[1] += 1,
            Nuc::G => counts[2] += 1,
            Nuc::T => counts[3] += 1,
        }
    }

    // Print the results
    println!("A: {}", counts[0]);
    println!("C: {}", counts[1]);
    println!("G: {}", counts[2]);
    println!("T: {}", counts[3]);
}
