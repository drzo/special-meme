use tokio::process::Command;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct VMCommand {
    command: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct VMResponse {
    output: String,
    error: Option<String>,
}

async fn execute_command(command: &str) -> VMResponse {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .await;

    match output {
        Ok(output) => {
            if output.status.success() {
                VMResponse {
                    output: String::from_utf8_lossy(&output.stdout).to_string(),
                    error: None,
                }
            } else {
                VMResponse {
                    output: String::new(),
                    error: Some(String::from_utf8_lossy(&output.stderr).to_string()),
                }
            }
        }
        Err(e) => VMResponse {
            output: String::new(),
            error: Some(e.to_string()),
        },
    }
}

#[tokio::main]
async fn main() {
    println!("Redox VM Integration Simulator");
    println!("Enter commands to simulate Redox OS execution:");

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let response = execute_command(input).await;
        println!("Response: {:?}", response);
    }
}
