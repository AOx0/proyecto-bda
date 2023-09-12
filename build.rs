use std::process::Command;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    let res = Command::new("tailwindcss")
        .args(["-i", "./templates/input.css", "-o", "./style.css"])
        .output();

    if let Err(err) = res {
        p!("Error executing `tailwindcss` {}. Skipping.", err);
    }
}
