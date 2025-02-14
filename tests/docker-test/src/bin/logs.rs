use bollard::container::LogsOptions;
use bollard::Docker;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() {
    let docker = match Docker::connect_with_unix_defaults() {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to Docker: {}", e);
            return;
        }
    };

    let container_name = "gitlab";
    let logs_options = LogsOptions::<String> {
        stdout: true,
        stderr: true,
        follow: true,            // Keep streaming logs
        tail: "100".to_string(), // Show last 100 lines before live streaming
        ..Default::default()
    };

    let mut logs_stream = docker.logs(container_name, Some(logs_options));

    while let Some(log_result) = logs_stream.next().await {
        match log_result {
            Ok(log_output) => {
                println!("{:?}", log_output);
            }
            Err(e) => eprintln!("Error reading logs: {}", e),
        }
    }
}
