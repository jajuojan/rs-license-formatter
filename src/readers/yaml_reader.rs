use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct License {
    pub license: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
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
    let file = fs::File::open(file_name).expect("Unable to read file");
    let deserialized: ThirdParty =
        serde_yaml::from_reader(file).expect("Unable to deserialize file contents");
    deserialized
}
