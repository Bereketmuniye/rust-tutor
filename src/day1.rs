use std::process::Command;


pub fn main() {
    let rust_version_output = Command::new("rustc").arg("--version").output().expect("faild to load value");
    let rust_version = String::from_utf8(rust_version_output.stdout).expect("Invalid UTF-8");
    println!("Rust version: {}", rust_version.trim());

    let cargo_versionoutput = Command::new("cargo").arg("--version").output().expect("faild to load value");
    let cargo_version = String::from_utf8(cargo_versionoutput.stdout).expect("Invalid UTF-8");
    println!("Cargo version: {}", cargo_version.trim());

    let git_version_output = Command::new("git").arg("--version").output().expect("faild to load value");
    let git_version = String::from_utf8(git_version_output.stdout).expect("Invalid UTF-8");
    println!("Git version: {}", git_version.trim());

    let node_version_output = Command::new("node").arg("--version").output().expect("faild to load value");
    let node_version = String::from_utf8(node_version_output.stdout).expect("Invalid UTF-8");
    println!("Node.js version: {}", node_version.trim());

    let npm_version_output = Command::new("npm").arg("--version").output().expect("faild to load value");
    let npm_version = String::from_utf8(npm_version_output.stdout).expect("Invalid UTF-8");
    println!("npm version: {}", npm_version.trim());

    let oprating_system_version_output = Command::new("uname").arg("-a").output().expect("faild to load value");
    let operating_system_version = String::from_utf8(oprating_system_version_output.stdout).expect("Invalid UTF-8");
    println!("Operating System version: {}", operating_system_version.trim());

    let system_architecture_output = Command::new("uname").arg("-m").output().expect("faild to load value");
    let system_architecture = String::from_utf8(system_architecture_output.stdout).expect("Invalid UTF-8");
    println!("System Architecture: {}", system_architecture.trim());

    let python_version_output = Command::new("python3").arg("--version").output().expect("faild to load value");
    let python_version = String::from_utf8(python_version_output.stdout).expect("Invalid UTF-8");
    println!("Python version: {}", python_version.trim());

    let ubuntu_version_output = Command::new("lsb_release").arg("-a").output().expect("faild to load value");
    let ubuntu_version = String::from_utf8(ubuntu_version_output.stdout).expect("Invalid UTF-8");
    println!("Ubuntu version: {}", ubuntu_version.trim());

    let docker_version_output = Command::new("docker").arg("--version").output().expect("faild to load value");
    let docker_version = String::from_utf8(docker_version_output.stdout).expect("Invalid UTF-8");
    println!("Docker version: {}", docker_version.trim());

    
}
