/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::fmt;

pub use bindgen::cargo::cargo_metadata::Error as CargoMetadataError;
pub use bindgen::cargo::cargo_toml::Error as CargoTomlError;
pub use bindgen::cargo::cargo_expand::Error as CargoExpandError;

#[derive(Debug)]
pub enum Error {
    CargoMetadata(String, CargoMetadataError),
    CargoToml(String, CargoTomlError),
    CargoExpand(String, CargoExpandError),
    ParseSyntaxError{crate_name: String, src_path: String, message: String},
    ParseCannotOpenFile{crate_name: String, src_path: String},
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::CargoMetadata(ref path, ref error) => {
                write!(f, "Couldn't execute `cargo metadata` with manifest {:?}: {:?}", path, error)
            }
            &Error::CargoToml(ref path, ref error) => {
                write!(f, "Couldn't load manifest file {:?}: {:?}", path, error)
            }
            &Error::CargoExpand(ref crate_name, ref error) => {
                write!(f, "Parsing crate `{}`: couldn't run `cargo rustc --pretty=expanded`: {:?}", crate_name, error)
            }
            &Error::ParseSyntaxError{ref crate_name, ref src_path, ref message} => {
                write!(f, "Parsing crate `{}`:`{}`:\n{}", crate_name, src_path, message)
            }
            &Error::ParseCannotOpenFile{ref crate_name, ref src_path} => {
                write!(f, "Parsing crate `{}`: cannot open file `{}`.", crate_name, src_path)
            }
        }
    }
}
