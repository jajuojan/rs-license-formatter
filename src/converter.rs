use std::rc::Rc;

use readers::yaml_reader::ThirdParty;

use crate::readers;

#[derive(Clone, Debug, PartialEq)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub license: String,
    pub licenses: Vec<Rc<License>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct License {
    pub license: String,
    pub text: String,
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
                    text: lic.text.to_owned(),
                });
                package_licenses.push(Rc::clone(&license));
                licenses.push(Rc::clone(&license));
            }

            packages.push(Package {
                name: library.package_name.to_owned(),
                version: library.package_version.to_owned(),
                license: library.license.to_owned(),
                licenses: package_licenses,
            });
        }
        PackageCollection { packages, licenses }
    }
}
