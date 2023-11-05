use std::fs::read_to_string;
use std::net::SocketAddr;
use std::process::exit;

use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use walkdir::WalkDir;

use crate::definition::Definition;

mod definition;
mod policy;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();
    let definitions = parse_definitions("definitions");
    let _ = serve_definitions(3000, definitions).await.map_err(|err| {
        log::error!("{err}");
    });
}

async fn serve_definitions(
    port: u16,
    _definitions: Vec<Definition>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let listener = TcpListener::bind(addr).await?;

    log::info!("Server running at {}:{}", addr.ip(), addr.port());

    loop {
        let (stream, _) = listener.accept().await?;

        let _ = TokioIo::new(stream);

        tokio::task::spawn(async move {});
    }
}

fn parse_definitions(directory: &str) -> Vec<Definition> {
    log::info!("Parsing definitions in {directory}");
    let mut definitions = vec![];
    for entry in WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        match entry.path().to_str() {
            Some(path) => {
                log::info!("Parsing {path}");
                let contents = read_to_string(path);
                match contents {
                    Ok(content) => {
                        let definition = serde_yaml::from_str(content.as_str());
                        match definition {
                            Ok(definition) => {
                                log::info!("Parsed {path}");
                                definitions.push(definition);
                            }
                            Err(error) => {
                                log::error!("Could not parse {path}: {error}");
                                exit(1);
                            }
                        }
                    }
                    Err(_) => {
                        log::error!("Malformed path.");
                        exit(1);
                    }
                }
            }
            None => {
                eprintln!("Could not process one of the files in {directory}");
                exit(1);
            }
        }
    }
    log::info!("Parsed definitions in {directory}");
    return definitions;
}
