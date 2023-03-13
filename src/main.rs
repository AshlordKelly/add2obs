use anyhow::Result;
use obws::{requests::inputs::Create, Client};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "add2obs", about = "Add file as source to OBS")]
struct Opt {
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    hostname: String,

    #[structopt(short = "k", long = "password")]
    password: Option<String>,

    #[structopt(short = "p", long = "port", default_value = "4444")]
    port: u16,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let client = Client::connect(opt.hostname, opt.port, opt.password).await?;
    let path = Path::new(&opt.input);
    let current_scene = client.scenes().current_program_scene().await?;

    client
        .inputs()
        .create(Create {
            scene: current_scene.as_str(),
            kind: "image_source",
            enabled: Some(true),
            input: path.file_name().unwrap().to_str().unwrap(),
            settings: Some(&json!({
                "file": fs::canonicalize(path.to_str().unwrap()).unwrap(),
            })),
        })
        .await?;
    Ok(())
}
