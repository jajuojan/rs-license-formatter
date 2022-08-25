use clap::Parser;
use converter::PackageCollection;
use readers::yaml_reader;
use writers::md_writer::{MdConfig, MdWriter};
mod converter;
mod readers;
mod writers;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Location of yaml license-file")]
    input_file: String,
}

fn main() {
    let args = Args::parse();
    // TODO: Fill from args
    let config = MdConfig {
        fail_on_missing_licenses: false,
        join_similar_licenses: false,
    };

    let deserialized = yaml_reader::read_from_file(&args.input_file);
    //println!("{:?}", deserialized);
    let packages = PackageCollection::from_third_party(&deserialized);
    let writer = MdWriter::new(&packages, config);

    let toc = writer.create_toc();
    let licenses = writer.create_license_texts_list();
    print!("{}\n{}", toc, licenses);
}
