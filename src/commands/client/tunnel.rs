use std::process::{Command, Stdio};

pub fn run_tunnel(
    local_port: Option<u16>,
    host: Option<String>,
    host_port: Option<u16>,
    tunnel_username: Option<String>,
    tunnel_password: Option<String>,
) {
    if local_port.is_none()
        || host.is_none()
        || host_port.is_none()
        || tunnel_username.is_none()
        || tunnel_password.is_none()
    {
        println!("You must provide all parameters to run the tunnel");
        return;
    }

    let local_port = local_port.unwrap();
    let host = host.unwrap();
    let host_port = host_port.unwrap();
    let tunnel_username = tunnel_username.unwrap();
    let tunnel_password = tunnel_password.unwrap();

    let output = Command::new("sshpass")
    .arg("-p")
    .arg(tunnel_password)
    .arg("ssh")
    .arg("-tt") 
    .arg("-o StrictHostKeyChecking=no")
    .arg("-R")
    .arg(format!("{}:localhost:{}", host_port, local_port))
    .arg(format!("{}@{}", tunnel_username, host))
    .stdin(Stdio::null())  // você provavelmente não precisa do stdin piped aqui
    .stderr(Stdio::piped())
    .output()
    .expect("Failed to run ssh command");

    println!("Process exited with status: {}", output.status);
}