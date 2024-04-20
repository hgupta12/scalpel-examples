use std::{ffi::OsStr, path::Path};

use actix_web::{error, post, web, App, Error, HttpServer, Responder};

use actix_files::Files;
use actix_multipart::form::{
    tempfile::{TempFile, TempFileConfig},
    MultipartForm,
};
use pcap_file::{pcap::PcapReader, pcapng::{Block, PcapNgReader}};
use serde::{Deserialize, Serialize};
use serde_json::json;
extern crate hex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::fs::create_dir_all("./tmp")?;

    HttpServer::new(|| {
        App::new()
            .app_data(TempFileConfig::default().directory("./tmp"))
            .service(save_files)
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(rename = "file")]
    files: Vec<TempFile>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Packet {
    data: String,
    timestamp: u64,
    len: u32,
}

#[post("/")]
async fn save_files(
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, Error> {
    let mut packets = Vec::new();
    for f in form.files {
        let filename = f.file_name.unwrap();
        let extension = Path::new(&filename)
            .extension()
            .and_then(OsStr::to_str)
            .unwrap_or("");

        match extension {
            "pcap" => {
                let mut pcap_reader = match PcapReader::new(f.file).map_err(|e| {
                    error::ErrorBadRequest(json!({
                        "error": format!("Error reading pcap file: {}", e)
                    }))
                }) {
                    Ok(reader) => reader,
                    Err(err) => return Err(err),
                };
                while let Some(pkt) = pcap_reader.next_packet() {
                    let pkt = pkt.unwrap();
                    let data_vec = pkt.data.to_vec();
                    let p = Packet {
                        data: hex::encode(&data_vec),
                        timestamp: pkt.timestamp.as_secs(),
                        len: pkt.orig_len,
                    };
                    packets.push(p);
                }
            }
            "pcapng" => {
                let mut pcapng_reader = match PcapNgReader::new(f.file).map_err(|e| {
                    error::ErrorBadRequest(json!({
                        "error": format!("Error reading pcapng file: {}", e)
                    }))
                }) {
                    Ok(reader) => reader,
                    Err(err) => return Err(err.into()),
                };
                while let Some(block) = pcapng_reader.next_block() {
                    let block = block.unwrap();
                    match block {
                        Block::EnhancedPacket(packet) => {
                            let data_vec = packet.data.to_vec();
                            let p = Packet {
                                data: hex::encode(&data_vec),
                                timestamp: packet.timestamp.as_secs(),
                                len: packet.original_len,
                            };
                            packets.push(p);
                        }
                        _ => {}
                    }
                }
            }
            _ => {
                return Err(error::ErrorBadRequest(json!({
                    "error": format!("Unsupported file format: {}", extension)
                })));
            }
        };
    }

    Ok(web::Json(packets))
}