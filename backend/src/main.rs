use bollard::Docker;

use rocket::http::Status;
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};

use std::process::Command;

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct DockerRequest {
    container_name: String,
    app_id: String,
    install_dir: String,
}
// Handle the JSON POST request and run the Docker command
#[post("/", data = "<docker_request>")]
fn create_container(
    docker_request: Json<DockerRequest>,
) -> Result<status::Custom<String>, status::Custom<String>> {
    let DockerRequest {
        container_name,
        app_id,
        install_dir,
    } = docker_request.into_inner();
    // Create the docker run command
    let command = format!(
        "docker run --name {} -d -it -v {}:/data steamcmd/steamcmd:debian +login anonymous +app_update {} +quit",
        container_name, install_dir, app_id
    );
    // Execute the command and capture the output
    let output = Command::new("sh").arg("-c").arg(&command).output();
    match output {
        Ok(output) => {
            if output.status.success() {
                Ok(status::Custom(
                    Status::Ok,
                    format!(
                        "Successfully created container '{}', App ID '{}', Installation directory '{}'",
                        container_name, app_id, install_dir
                    ),
                ))
            } else {
                let error_message = String::from_utf8_lossy(&output.stderr);
                Err(status::Custom(
                    Status::InternalServerError,
                    format!(
                        "Failed to create container '{}'. Error: {}",
                        container_name, error_message
                    ),
                ))
            }
        }
        Err(e) => Err(status::Custom(
            Status::InternalServerError,
            format!("Failed to execute command. Error: {}", e),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_container])
}
