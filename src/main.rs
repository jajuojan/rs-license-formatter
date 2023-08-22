use clap::{Parser, ValueEnum};
use converter::{Package, PackageCollection};
use readers::yaml_reader;
use writers::md_writer::MdWriter;
mod converter;
mod readers;
mod writers;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputChoice {
    All,
    TocOnly,
    LicenseTextsOnly,
}

#[derive(Parser)]
#[clap(
    author,
    version,
    about,
    long_about = "Convert third-party license-information from toml to markdown"
)]
struct Args {
    #[clap(help = "Location of yaml license-file")]
    input_file: String,

    #[clap(
        short = 'f',
        long = "fail-on-missing",
        help = "Fail on missing license-texts"
    )]
    fail_on_missing_licenses: bool,

    #[clap(
        short = 'o',
        long = "output",
        help = "Choose what to output",
        value_enum,
        default_value = "all"
    )]
    choice: Option<OutputChoice>,
}

fn decorate_with_crate_link(p: &mut Package) {
    p.link = Some(format!(
        "https://crates.io/crates/{}/{}/",
        p.name, p.version
    ));
}

fn main() {
    let args = Args::parse();

    let deserialized = yaml_reader::read_from_file(&args.input_file);
    let mut packages = PackageCollection::from_third_party(&deserialized);

    if args.fail_on_missing_licenses && packages.has_missing_license_texts() {
        eprint!("Missing license-texts found");
        std::process::exit(1);
    }

    packages
        .packages
        .iter_mut()
        .for_each(|p| decorate_with_crate_link(p));

    let writer = MdWriter::new(&packages);
    let mut toc: String = String::new();
    let mut licenses: String = String::new();
    if (args.choice == Some(OutputChoice::All)) || (args.choice == Some(OutputChoice::TocOnly)) {
        toc = writer.create_toc();
    }
    if (args.choice == Some(OutputChoice::All))
        || (args.choice == Some(OutputChoice::LicenseTextsOnly))
    {
        licenses = writer.create_license_texts_list();
    }

    print!("{}\n{}", toc, licenses);
}
