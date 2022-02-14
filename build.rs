extern crate winres;
use std::fs;
use std::io::prelude::*;
use zip::write::FileOptions;

fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("resources/icon.ico");
    res.compile().unwrap();
    compress_client().unwrap();
    println!("cargo:rerun-if-changed=resources/client.exe");
}

fn compress_client() -> zip::result::ZipResult<()> {
    let mut zip = zip::ZipWriter::new(std::fs::File::create(std::path::Path::new("src/packed.zip")).unwrap());
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);
    zip.start_file("_", options)?;
    zip.write_all(&fs::read("resources/client.exe").unwrap())?;
    zip.finish()?;
    Ok(())
}
