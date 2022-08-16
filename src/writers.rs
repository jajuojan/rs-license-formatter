use regex::Regex;

use crate::reader;

#[derive(Clone)]
struct MdLicense {
    license: String,
    text: Option<String>,
    link_anchor: Option<String>,
}
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

pub struct MdWriter {
    md_items: Vec<MdItem>,
    md_config: MdConfig,
}

fn guess_copyright_note(licenses: &Vec<MdLicense>) -> Option<String> {
    let a: Vec<&MdLicense> = licenses.iter().filter(|l| l.license == "MIT").collect();
    if a.len() != 1 || a[0].text.is_none() {
        return None;
    }

    let re = Regex::new(r"Copyright \(c\)").unwrap();
    let license_text = a[0].to_owned().text.unwrap();
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

fn to_md_item(input: &reader::Library) -> MdItem {
    let mut licenses: Vec<MdLicense> = Vec::new();
    for lic in &input.licenses {
        licenses.push(MdLicense {
            license: lic.license.to_owned(),
            link_anchor: Some(format_license_header(&input.package_name, &lic.license)),
            text: if lic.text == "NOT FOUND" {
                None
            } else {
                Some(lic.text.to_owned())
            },
        });
    }

    MdItem {
        package_name: input.package_name.to_owned(),
        copyright_note: guess_copyright_note(&licenses),
        link_to_project: None, // TODO: implement
        licenses: licenses,
    }
}

fn into_md_items(input: &reader::ThirdParty) -> Vec<MdItem> {
    input
        .third_party_libraries
        .iter()
        .map(|library| to_md_item(library))
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
    pub fn new(input: &reader::ThirdParty, md_config: MdConfig) -> Self {
        Self {
            md_items: into_md_items(input),
            md_config,
        }
    }

    pub fn create_toc(&self) -> String {
        let mut output = "| Library Name | License | Authors |\n|-|-|-|\n".to_string();
        for md_item in &self.md_items {
            let name = format_toc_package_name(&md_item);
            let license = format_toc_license_name(&md_item);
            output += &format!(
                "| {} | {} | {} |\n",
                name,
                license,
                md_item.copyright_note.to_owned().unwrap_or("".to_string())
            );
        }
        output
    }

    pub fn create_licenses_list(&self) -> String {
        let mut output = "# Licenses of Third-Party Libraries\n".to_string();
        for md_item in &self.md_items {
            for license in &md_item.licenses {
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
