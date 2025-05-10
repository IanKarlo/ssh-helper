use std::process::Command;
use std::io::{self, Write};

fn check_if_ssh_is_installed() -> bool {
    let output = Command::new("ssh").arg("-V").output();
    output.is_ok()
}

fn check_if_ssh_pass_is_installed() -> bool {
    let output = Command::new("sshpass").arg("-v").output();
    output.is_ok()
}

pub fn configure_ssh(yes: bool) {
    let ssh_is_installed = check_if_ssh_is_installed();

    let info = os_info::get();
    let os = info.os_type();

    let valid_os = if os == os_info::Type::Ubuntu || os == os_info::Type::Debian {
        true
    } else {
        false
    };

    if !ssh_is_installed {
        print!("SSH is not installed, do you want to install it? (Y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let mut trimmed_input: &str = "n";
        if !yes {
            std::io::stdin().read_line(&mut input).unwrap();
            trimmed_input = input.trim();
        }
        if (yes || trimmed_input == "Y" || trimmed_input == "y" || trimmed_input == "") && valid_os {
            println!("Installing SSH...");
            println!("installing ssh");
            let output = Command::new("apt-get").arg("-y").arg("install").arg("ssh").output();
            if output.is_ok() {
                println!("SSH installed successfully");
            } else {
                println!("Failed to install SSH");
                std::process::exit(1);
            }
        } else {
            println!("SSH is not installed, please install it and try again");
            std::process::exit(1);
        }
    }

    let ssh_pass_is_installed = check_if_ssh_pass_is_installed();

    if !ssh_pass_is_installed {
        print!("SSH-PASS is not installed, do you want to install it? (Y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let mut trimmed_input: &str = "n";
        if !yes {
            std::io::stdin().read_line(&mut input).unwrap();
            trimmed_input = input.trim();
        }
        if (yes ||trimmed_input == "Y" || trimmed_input == "y" || trimmed_input == "") && valid_os {
            println!("Installing SSH-PASS...");
            println!("installing ssh-pass");
            let output = Command::new("apt-get").arg("-y").arg("install").arg("sshpass").output();
            if output.is_ok() {
                println!("SSH-PASS installed successfully");
            } else {
                println!("Failed to install SSH-PASS");
                std::process::exit(1);
            }
        } else {
            println!("SSH-PASS is not installed, please install it and try again");
            std::process::exit(1);
        }
    }
}