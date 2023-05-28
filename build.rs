use std::process::Command;

fn main() {
    println!("rm crud");
    Command::new("rm")
        .args(&["-rf", "frontend/dist"])
        .status()
        .unwrap();

    println!("build");
    Command::new("trunk")
        .args(&["build", "frontend/index.html"])
        .status()
        .unwrap();
    
    println!("cargo:rerun-if-changed=frontend/");
}