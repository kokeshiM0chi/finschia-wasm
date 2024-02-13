use std::path::PathBuf;
use std::process;

pub fn checkout_submodule(dir: &str, rev: &str) {
    let full_path = |p: &str| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(p);
    let mut cmd_git = process::Command::new("git");
    let full_path_dir = full_path(dir).to_string_lossy().to_string();
    let err_msg = format!("Checkout Error; Path: {}", &full_path_dir);
    cmd_git
        .arg("-C")
        .arg(&full_path_dir)
        .arg("checkout")
        .arg(rev)
        .status()
        .expect(&err_msg);
}
