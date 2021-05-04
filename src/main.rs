use anyhow::Result;
use obws::{Client, requests::CreateSource};
use serde_json::json;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "add2obs", about = "Add file as source to OBS")]
struct Opt {
    #[structopt(short = "h", long = "host", default_value = "localhost")]
    hostname: String,

    #[structopt(short = "p", long = "port", default_value = "4444")]
    port: u16,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let opt = Opt::from_args();
    let client = Client::connect(opt.hostname, opt.port).await?;
    let path = Path::new(&opt.input);
    let current_scene = client.scenes().get_current_scene().await?.name;

    client.sources().create_source(CreateSource{
        source_name: path.file_name().unwrap().to_str().unwrap(),
        source_kind: "image_source",
        scene_name: &current_scene,
        set_visible: Some(true),
        source_settings: Some(&json!({
            "file": path.to_str().unwrap(),
        })),
    }).await?;

    Ok(())
}
