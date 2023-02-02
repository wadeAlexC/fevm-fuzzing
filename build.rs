use std::env;
use std::process::{Command};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut command = Command::new("python3");
    command
        .arg("gobuild.py")
        .arg(format!("-p={}", out_dir))
        .arg("-n=libcalculate.a");

    
    match command.output() {
        Ok(out) => {
            println!("python output: {:?}", out);
        }
        Err(e) => {
            panic!("error: {:?}", e)
        }
    };
   
    sleep(Duration::from_secs(1));
    println!("cargo:rustc-link-search=native={}", out_dir);
}
