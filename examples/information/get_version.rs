//! Retrieve Senzing product version and license information

use sz_rust_sdk::helpers::ExampleEnvironment;
use sz_rust_sdk::prelude::*;

fn main() -> SzResult<()> {
    let env = ExampleEnvironment::initialize("get-version-example")?;

    match env.get_product() {
        Ok(product) => {
            match product.get_version() {
                Ok(version) => println!("Version: {}", version),
                Err(e) => println!("Version unavailable: {}", e),
            }

            match product.get_license() {
                Ok(license) => println!("License: {}", license),
                Err(e) => println!("License unavailable: {}", e),
            }
        }
        Err(e) => println!("Product component unavailable: {}", e),
    }

    ExampleEnvironment::cleanup(env)?;
    Ok(())
}
