use log::{info, warn};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process;

fn run_git(args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<(), String> {
    let stdout = process::Stdio::inherit();

    let exit_status = process::Command::new("git")
        .args(args)
        .stdout(stdout)
        .status()
        .expect("git exit status missing");

    if !exit_status.success() {
        return Err(format!(
            "git exited with error code: {:?}",
            exit_status.code()
        ));
    }

    Ok(())
}

pub fn update_submodule(dir: &str, rev: &str) {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);

    info!("Updating {} submodule...", dir);

    // switch to the given revision
    run_git(["-C", full_path(dir).to_str().unwrap(), "checkout", rev]).expect("failed to checkout");

    // pull the latest changes
    match run_git(["-C", full_path(dir).to_str().unwrap(), "pull"]) {
        Ok(_) => info!("Updated {} submodule to revision {}", dir, rev),
        Err(_) => warn!(
            "Failed to update {} with revision {}. This might be caused by revision is a tag",
            dir, rev
        ),
    };

    // run_git(["submodule", "update", "--init"]).expect("failed to update submodules");
}

pub fn checkout_submodule(dir: &str, rev: &str) {
    let full_path   = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);
    let mut cmd_git = process::Command::new("git");
    let full_path_dir = full_path(dir).to_str().expect("error");
    let err_msg = format!("Checkout Error; Path: {}", full_path_dir);
    cmd_git.arg("-C").arg(full_path_dir).arg("checkout").arg(rev).status().expect(&err_msg);
}
