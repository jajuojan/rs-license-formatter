use std::rc::Rc;

use readers::yaml_reader::ThirdParty;

use crate::readers;

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub link: Option<String>,
    pub license: String,
    pub licenses: Vec<Rc<License>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct License {
    pub license: String,
    pub text: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackageCollection {
    pub packages: Vec<Package>,
    pub licenses: Vec<Rc<License>>,
}

impl PackageCollection {
    pub fn from_third_party(third_party: &ThirdParty) -> PackageCollection {
        let mut packages = Vec::new();
        let mut licenses = Vec::new();

        for library in &third_party.third_party_libraries {
            let mut package_licenses = Vec::new();
            for lic in &library.licenses {
                let license = Rc::new(License {
                    license: lic.license.to_owned(),
                    text: resolve_license_text(&lic.text),
                });
                package_licenses.push(Rc::clone(&license));
                licenses.push(Rc::clone(&license));
            }

            packages.push(Package {
                name: library.package_name.to_owned(),
                version: library.package_version.to_owned(),
                link: None,
                license: library.license.to_owned(),
                licenses: package_licenses,
            });
        }
        PackageCollection { packages, licenses }
    }

    pub(crate) fn has_missing_license_texts(&self) -> bool {
        for package in &self.packages {
            if package.licenses.iter().any(|l| l.text.is_none()) {
                return true;
            }
        }

        false
    }
}

fn resolve_license_text(license_text: &String) -> Option<String> {
    if license_text == "NOT FOUND" {
        None
    } else {
        Some(license_text.to_owned())
    }
}

#[cfg(test)]
pub mod tests {
    use std::rc::Rc;

    use crate::converter::{License, Package, PackageCollection};
    use crate::readers::yaml_reader::tests::third_party_data;

    pub fn package_collection_data() -> PackageCollection {
        PackageCollection {
            packages: vec![
                Package {
                    name: "my_dependency".to_owned(),
                    version: "0.1.1".to_owned(),
                    license: "MIT".to_owned(),
                    link: None,
                    licenses: vec![
                        Rc::new(License {
                            license: "MIT".to_owned(),
                            text: Some("Lots of text...".to_owned()),
                        }),
                        Rc::new(License {
                            license: "Apache-2.0".to_owned(),
                            text: Some("Lots of text 2...".to_owned()),
                        }),
                    ],
                },
                Package {
                    name: "my_other_dependency".to_owned(),
                    version: "1.0.0".to_owned(),
                    license: "Apache-2.0".to_owned(),
                    link: None,
                    licenses: vec![Rc::new(License {
                        license: "Apache-2.0".to_owned(),
                        text: Some("Lots of text 2...".to_owned()),
                    })],
                },
            ],
            licenses: vec![
                Rc::new(License {
                    license: "MIT".to_owned(),
                    text: Some("Lots of text...".to_owned()),
                }),
                Rc::new(License {
                    license: "Apache-2.0".to_owned(),
                    text: Some("Lots of text 2...".to_owned()),
                }),
                Rc::new(License {
                    license: "Apache-2.0".to_owned(),
                    text: Some("Lots of text 2...".to_owned()),
                }),
            ],
        }
    }

    #[test]
    fn test_from_third_party() {
        let input = third_party_data();
        let output = PackageCollection::from_third_party(&input);
        assert_eq!(output, package_collection_data(),);
    }
}
