use std::process::{Command, Stdio};

pub fn run_tunnel(
    local_port: u16,
    host: String,
    host_port: u16,
    tunnel_username: String,
    tunnel_password: Option<String>,
    tunnel_key: Option<String>,
) {
    if tunnel_password.is_none() && tunnel_key.is_none()
    {
        println!("You must provide all parameters to run the tunnel");
        return;
    }

    if tunnel_password.is_some() {

        let pwsd = tunnel_password.unwrap();

        let output = Command::new("sshpass")
        .arg("-p")
        .arg(pwsd)
        .arg("ssh")
        .arg("-tt") 
        .arg("-o StrictHostKeyChecking=no")
        .arg("-R")
        .arg(format!("{}:localhost:{}", host_port, local_port))
        .arg(format!("{}@{}", tunnel_username, host))
        .stdin(Stdio::null()) 
        .stderr(Stdio::piped())
        .output()
        .expect("Failed to run ssh command");

        println!("Process exited with status: {}", output.status);
    }


    if let Some(key) = &tunnel_key {
        let output = Command::new("ssh")
            .arg("-tt")
            .arg("-o StrictHostKeyChecking=no")
            .arg("-R")
            .arg(format!("{}:localhost:{}", host_port, local_port))
            .arg(format!("{}@{}", tunnel_username, host))
            .arg("-i")
            .arg(key)
            .stdin(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run ssh command");

        println!("Process exited with status: {}", output.status);
    }
    
}