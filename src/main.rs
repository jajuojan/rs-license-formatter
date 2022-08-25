use writers::MdWriter;

mod reader;
mod writers;
use clap::Parser;

use crate::writers::MdConfig;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Location of yaml license-file")]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    let deserialized = reader::read_from_file(&args.input_file);
    // TODO: Fill from args
    let config = MdConfig {
        fail_on_missing_licenses: false,
        join_similar_licenses: false,
    };
    let writer = MdWriter::new(&deserialized, config);

    let toc = writer.create_toc();
    let licenses = writer.create_license_texts_list();
    print!("{}\n{}", toc, licenses);
}
