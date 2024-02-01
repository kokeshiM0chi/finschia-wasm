//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use dotenvy;
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
/// Directory where the cometbft submodule is located
const COMETBFT_DIR: &str = "../../dependencies/cometbft/";
/// Directory where the ibc-go submodule is located
const IBC_GO_DIR: &str = "../../dependencies/ibc-go/";
/// Directory where the ics23 submodule is located
const ICS23_DIR: &str = "../../dependencies/ics23/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate(version_tags: &HashMap<String, String>) {
    let args: Vec<String> = env::args().collect();
    let finschia_sdk_version = version_tags
        .get("FINSCHIA_SDK_VERSION")
        .expect("FINSCHIA_SDK_VERSION is not set");
    let wasmd_version = version_tags
        .get("WASMD_VERSION")
        .expect("WASMD_VERSION is not set");
    let tendermint_version = version_tags
        .get("TENDERMINT_VERSION")
        .expect("TENDERMINT_VERSION is not set");
    let ibc_go_version = version_tags
        .get("IBC_GO_VERSION")
        .expect("IBC_GO_VERSION is not set");
    let ics23_version = version_tags
        .get("ICS23_VERSION")
        .expect("ICS23_VERSION is not set");

    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(FINSCHIA_SDK_DIR, &finschia_sdk_version);
        git::update_submodule(WASMD_DIR, &wasmd_version);
        git::update_submodule(COMETBFT_DIR, &tendermint_version);
        git::update_submodule(IBC_GO_DIR, &ibc_go_version);
        git::update_submodule(ICS23_DIR, &ics23_version);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: wasmd_version.to_string(),
        project_dir: WASMD_DIR.to_string(),
        exclude_mods: vec![],
    };
    let cometbft_project = CosmosProject {
        name: "tendermint".to_string(),
        version: tendermint_version.to_string(),
        project_dir: COMETBFT_DIR.to_string(),
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
        vec![ibc_project, cometbft_project, wasmd_project, ics23_project],
    );

    finschia_code_generator.generate();
}

fn main() {
    let iter = dotenvy::from_filename_iter("env").expect("The env file in which tag is defined was not found.");
    let mut version_tags: HashMap<String, String> = HashMap::new();

    for item in iter {
        let (key, val) = item.unwrap();
        version_tags.insert(key, val);
    }

    pretty_env_logger::init();
    generate(&version_tags);
}
