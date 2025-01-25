use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct License {
    pub license: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Library {
    pub package_name: String,
    pub package_version: String,
    pub license: String,
    pub licenses: Vec<License>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ThirdParty {
    pub root_name: String,
    pub third_party_libraries: Vec<Library>,
}

pub fn read_from_file(file_name: &str) -> ThirdParty {
    let file = fs::File::open(file_name).expect(&format!("Unable to read file: {}", file_name));
    let deserialized: ThirdParty = serde_yml::from_reader(file).expect(&format!(
        "Unable to deserialize file contents from {}",
        file_name
    ));
    deserialized
}

#[cfg(test)]
pub mod tests {
    use super::{Library, License, ThirdParty};

    pub fn third_party_data() -> ThirdParty {
        ThirdParty {
            root_name: "my-project".to_owned(),
            third_party_libraries: vec![
                Library {
                    package_name: "my_dependency".to_owned(),
                    package_version: "0.1.1".to_owned(),
                    license: "MIT".to_owned(),
                    licenses: vec![
                        License {
                            license: "MIT".to_owned(),
                            text: "Lots of text...".to_owned(),
                        },
                        License {
                            license: "Apache-2.0".to_owned(),
                            text: "Lots of text 2...".to_owned(),
                        },
                    ],
                },
                Library {
                    package_name: "my_other_dependency".to_owned(),
                    package_version: "1.0.0".to_owned(),
                    license: "Apache-2.0".to_owned(),
                    licenses: vec![License {
                        license: "Apache-2.0".to_owned(),
                        text: "Lots of text 2...".to_owned(),
                    }],
                },
            ],
        }
    }
}
