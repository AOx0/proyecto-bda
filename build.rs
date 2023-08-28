use std::process::Command;

fn main() {
    Command::new("tailwindcss")
        .args(["-i", "./templates/input.css", "-o", "./style.css"])
        .output()
        .expect("failed to execute process");
}
