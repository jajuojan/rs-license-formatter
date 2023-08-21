use clap::Parser;
use converter::PackageCollection;
use readers::yaml_reader;
use writers::md_writer::MdWriter;
mod converter;
mod readers;
mod writers;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(help = "Location of yaml license-file")]
    input_file: String,

    #[clap(
        short = 'f',
        long = "fail-on-missing",
        help = "Fail on missing license-texts"
    )]
    fail_on_missing_licenses: bool,
}

fn main() {
    let args = Args::parse();

    let deserialized = yaml_reader::read_from_file(&args.input_file);
    let packages = PackageCollection::from_third_party(&deserialized);

    if args.fail_on_missing_licenses && packages.has_missing_license_texts() {
        eprint!("Missing license-texts found");
        std::process::exit(1);
    }

    //let config = MdConfig {};
    let writer = MdWriter::new(&packages /* , config*/);
    let toc = writer.create_toc();
    let licenses = writer.create_license_texts_list();

    print!("{}\n{}", toc, licenses);
}
