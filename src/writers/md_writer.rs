use regex::Regex;

use crate::converter::{Package, PackageCollection};

#[derive(Clone)]
struct MdLicense {
    license: String,
    text: Option<String>,
    link_anchor: Option<String>,
}
#[derive(Clone)]
struct MdItem {
    package_name: String,
    copyright_note: Option<String>,
    link_to_project: Option<String>,
    licenses: Vec<MdLicense>,
}
pub struct MdConfig {
    pub fail_on_missing_licenses: bool,
    pub join_similar_licenses: bool,
}
pub struct MdWriterItem {
    md_item: MdItem,
    license_text_ref: String,
}

pub struct MdWriter {
    writer_items: Vec<MdWriterItem>,
    md_config: MdConfig,
    license_texts: Vec<String>,
}

impl MdWriterItem {
    fn from(md_item: &MdItem) -> Self {
        MdWriterItem {
            md_item: md_item.to_owned(),
            license_text_ref: String::new(),
        }
    }
}

fn guess_copyright_note(licenses: &Vec<MdLicense>) -> Option<String> {
    let mit_licenses: Vec<&MdLicense> = licenses.iter().filter(|l| l.license == "MIT").collect();
    if mit_licenses.len() != 1 || mit_licenses[0].text.is_none() {
        return None;
    }

    // TODO: use lazy_static for RegExp?
    let re = Regex::new(r"Copyright \(c\)").unwrap();
    let license_text = mit_licenses[0].to_owned().text.unwrap();
    let copyright_line: Vec<&str> = license_text
        .split("\n")
        .filter(|t| re.is_match(t))
        .collect();
    if copyright_line.len() == 1 {
        return Some(copyright_line[0].trim().to_string());
    }

    return None;
}

fn format_license_header(package: &str, license: &str) -> String {
    format!("{} ({})", license, package)
}

fn to_md_item(package: &Package) -> MdItem {
    let mut licenses: Vec<MdLicense> = Vec::new();
    for lic in &package.licenses {
        licenses.push(MdLicense {
            license: lic.license.to_owned(),
            link_anchor: Some(format_license_header(&package.name, &lic.license)),
            text: if lic.text == "NOT FOUND" {
                None
            } else {
                Some(lic.text.to_owned())
            },
        });
    }

    MdItem {
        package_name: package.name.to_owned(),
        copyright_note: guess_copyright_note(&licenses),
        link_to_project: None, // TODO: implement
        licenses,
    }
}

fn into_md_items(input: &PackageCollection) -> Vec<MdItem> {
    input
        .packages
        .iter()
        .map(|package| to_md_item(package))
        .collect()
}

fn to_name_or_link(first: &str, second: &Option<String>) -> String {
    if second.is_some() {
        format!("[{}]({})", &first, &second.to_owned().unwrap())
    } else {
        format!("{}", &first)
    }
}

fn format_toc_package_name(i: &MdItem) -> String {
    to_name_or_link(&i.package_name, &i.link_to_project)
}

fn format_toc_license_name(i: &MdItem) -> String {
    let mut license = "".to_string();
    for (pos, lic) in i.licenses.iter().enumerate() {
        if pos > 0 {
            license += "/";
        }
        let mut anchor = lic.link_anchor.to_owned();
        if lic.link_anchor.is_some() {
            anchor = Some(format!(
                "#{}",
                &lic.link_anchor
                    .to_owned()
                    .unwrap()
                    .replace(" ", "-")
                    .replace("(", "")
                    .replace(")", "")
                    .replace(".", "")
            ));
        }
        license += &to_name_or_link(&lic.license, &anchor);
    }
    license
}

impl MdWriter {
    pub fn new(input: &PackageCollection, md_config: MdConfig) -> Self {
        let license_texts: Vec<String> = [].to_vec();
        let writer_items = into_md_items(input)
            .iter()
            .map(|i| MdWriterItem::from(i))
            .collect();
        Self {
            writer_items,
            md_config,
            license_texts,
        }
    }

    pub fn create_toc(&self) -> String {
        let mut output = "| Library Name | License | Authors |\n|-|-|-|\n".to_string();
        for writer_item in &self.writer_items {
            let name = format_toc_package_name(&writer_item.md_item);
            let license = format_toc_license_name(&writer_item.md_item);
            output += &format!(
                "| {} | {} | {} |\n",
                name,
                license,
                writer_item
                    .md_item
                    .copyright_note
                    .to_owned()
                    .unwrap_or("".to_string())
            );
        }
        output
    }

    pub fn create_license_texts_list(&self) -> String {
        let mut output = "# Licenses of Third-Party Libraries\n".to_string();
        for writer_item in &self.writer_items {
            for license in &writer_item.md_item.licenses {
                output += &format!(
                    "\n## {}\n\n```\n{}```\n",
                    license
                        .to_owned()
                        .link_anchor
                        .unwrap_or(license.license.to_owned()),
                    license
                        .text
                        .as_ref()
                        .unwrap_or(&"NOT FOUND\n".to_string())
                        .replace("\\n", "\n")
                );
            }
        }
        output
    }
}

#[cfg(test)]
mod tests {
    use crate::converter::tests::package_collection_data;
    use crate::writers::md_writer::{MdConfig, MdWriter};

    #[test]
    fn test_create_toc() {
        let input = package_collection_data();
        let writer = MdWriter::new(
            &input,
            MdConfig {
                join_similar_licenses: false,
                fail_on_missing_licenses: false,
            },
        );
        let toc = writer.create_toc();
        assert_eq!(
            toc,
            "| Library Name | License | Authors |\n|-|-|-|\n| my_dependency | [MIT](#MIT-my_dependency)/[Apache-2.0](#Apache-20-my_dependency) |  |\n| my_other_dependency | [Apache-2.0](#Apache-20-my_other_dependency) |  |\n"
        );
    }
}
