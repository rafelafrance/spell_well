use clap::Parser;
use spell_well::{SpellWell, WordFreqs};
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(about = "A very simple spell checker.")]
struct Cli {
    ///Read the vocabulary with word frequencies from this CSV file
    #[clap(short, long, value_parser, value_name = "FILE")]
    vocab_csv: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    let mut word_freqs = WordFreqs::new();
    _ = word_freqs.read_csv(&args.vocab_csv);

    SpellWell::new().add_word_list(&word_freqs, 2);

    Ok(())
}
