//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::path::PathBuf;

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};
use std::collections::HashMap;

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../finschia-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const FINSCHIA_SDK_DIR: &str = "../../dependencies/finschia-sdk/";
/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../../dependencies/wasmd/";
/// Directory where the ostracon submodule is located
const OSTRACON_DIR: &str = "../../dependencies/ostracon/";
/// Directory where the ostracon submodule is located
const TENDERMINT_DIR: &str = "../../dependencies/tendermint/";
/// Directory where the ibc-go submodule is located
const IBC_GO_DIR: &str = "../../dependencies/ibc-go/";
/// Directory where the ics23 submodule is located
const ICS23_DIR: &str = "../../dependencies/ics23/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate(version_tags: &HashMap<String, String>) {
    let finschia_sdk_version = version_tags
        .get("FINSCHIA_SDK_VERSION")
        .expect("FINSCHIA_SDK_VERSION is not set");
    let wasmd_version = version_tags
        .get("WASMD_VERSION")
        .expect("WASMD_VERSION is not set");
    let ostracon_version = version_tags
        .get("OSTRACON_VERSION")
        .expect("TENDERMINT_VERSION is not set");
    let tendermint_version = version_tags
        .get("TENDERMINT_VERSION")
        .expect("TENDERMINT_VERSION is not set");
    let ibc_go_version = version_tags
        .get("IBC_GO_VERSION")
        .expect("IBC_GO_VERSION is not set");

    // cosmos/ics23 have not supported yet in Finschia. So it is fixed tag as like osmosis.
    let ics23_version = "rust/v0.10.0";
    git::checkout_submodule(FINSCHIA_SDK_DIR, finschia_sdk_version);
    git::checkout_submodule(WASMD_DIR, wasmd_version);
    git::checkout_submodule(OSTRACON_DIR, ostracon_version);
    git::checkout_submodule(TENDERMINT_DIR, tendermint_version);
    git::checkout_submodule(IBC_GO_DIR, ibc_go_version);
    git::checkout_submodule(ICS23_DIR, ics23_version);

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: wasmd_version.to_string(),
        project_dir: WASMD_DIR.to_string(),
        exclude_mods: vec![],
    };
    let ostracon_project = CosmosProject {
        name: "ostracon".to_string(),
        version: ostracon_version.to_string(),
        project_dir: OSTRACON_DIR.to_string(),
        exclude_mods: vec![],
    };

    let tendermint_project = CosmosProject {
        name: "tendermint".to_string(),
        version: tendermint_version.to_string(),
        project_dir: TENDERMINT_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ibc_project = CosmosProject {
        name: "ibc".to_string(),
        version: ibc_go_version.to_string(),
        project_dir: IBC_GO_DIR.to_string(),
        exclude_mods: vec![],
    };

    let cosmos_project = CosmosProject {
        name: "finschia".to_string(),
        version: finschia_sdk_version.to_string(),
        project_dir: FINSCHIA_SDK_DIR.to_string(),
        exclude_mods: vec![],
    };

    let ics23_project = CosmosProject {
        name: "ics23".to_string(),
        version: ics23_version.to_string(),
        project_dir: ICS23_DIR.to_string(),
        exclude_mods: vec![],
    };

    let finschia_code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        cosmos_project,
        vec![
            ibc_project,
            ostracon_project,
            tendermint_project,
            wasmd_project,
            ics23_project,
        ],
    );

    finschia_code_generator.generate();
}

fn main() {
    let iter = dotenvy::from_filename_iter("env")
        .expect("The env file in which tag is defined was not found.");
    let mut version_tags: HashMap<String, String> = HashMap::new();

    for item in iter {
        let (key, val) = item.unwrap();
        version_tags.insert(key, val);
    }

    pretty_env_logger::init();
    generate(&version_tags);
}
