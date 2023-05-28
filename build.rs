use std::process::Command;

fn main() {
    println!("rm");
    Command::new("rm")
        .args(&["-rf", "frontend/dist"])
        .status()
        .unwrap();

    Command::new("trunk")
        .args(&["build", "frontend/index.html"])
        .status()
        .unwrap();
    
    println!("cargo:rerun-if-changed=frontend/");
}