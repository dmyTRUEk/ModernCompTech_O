//! Generate nanotubes by their chirality numbers and repetion count and save in pdb file.

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Params {
    #[arg(help = "Chirality number `m`")]
    m: u32,
    #[arg(help = "Chirality number `n`")]
    n: u32,
    #[arg(help = "Repetion count")]
    repetions: u32,
}


fn main() {
    let params = Params::parse();

    dbg!(params);
}

