use bollard::Docker;
use clap::Parser;
use std::fs::File;
use std::os::unix::process::CommandExt;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    container_name: String,

    #[clap(allow_hyphen_values = true)]
    curl_args: Vec<String>,
}

type BoxError = Box<dyn std::error::Error + Sync + Send>;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let args = Args::parse();
    let docker = Docker::connect_with_socket_defaults()
        .map_err(|err| format!("failed to connect to Docker socket: {err}"))?;

    let options = None;
    let inspect = docker
        .inspect_container(&args.container_name, options)
        .await
        .map_err(|err| format!("failed to inspect container: {err}"))?;

    let netns = inspect
        .network_settings
        .ok_or("container is missing network settings")?
        .sandbox_key
        .ok_or("container is missing netns file")?;

    let netns = File::options()
        .read(true)
        .open(&netns)
        .map_err(|err| format!("failed to open netns file {netns}: {err}"))?;

    let nstype = nix::sched::CloneFlags::CLONE_NEWNET;
    nix::sched::setns(netns, nstype).map_err(|err| format!("failed to join net ns: {err}"))?;

    Command::new("curl").args(args.curl_args).exec();
    Ok(())
}
