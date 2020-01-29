fn main() {
    use std::process::Command;
    let output = Command::new("pip")
        .args(&["install", "bs4"])
        .output()
        .expect("failed to execute process");

    println!("status: {}", output.status);
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}