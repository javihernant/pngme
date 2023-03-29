use crate::args::{EncodeArgs};
use crate::Result as ResultAlias;
use crate::png::Png;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use std::path::PathBuf;
use std::str::FromStr;
use std::fs;

fn loadPng(path:&PathBuf) -> ResultAlias<Png> {
    let bytes = fs::read(path)?;
    Png::try_from(bytes.as_slice())
}

fn chunk_from_strings(chunk_type: &str, data: &str) -> ResultAlias<Chunk> {
    let chunk_type = ChunkType::from_str(chunk_type)?;
    let data: Vec<u8> = data.bytes().collect();

    Ok(Chunk::new(chunk_type, data))
}

fn produce_out(png: Png, file_path:&PathBuf, out_path:&Option<PathBuf>) -> Result<(), std::io::Error> {
    let out_path = match out_path {
        Some(path) => path.to_owned(),
        None => {
            let file_name = file_path.file_stem().unwrap();
            let encoded_path = file_path.parent().unwrap().join(file_name);
            let mut encoded_path_str = encoded_path.into_os_string();
            encoded_path_str.push("_encoded");
            let mut encoded_path = PathBuf::from(encoded_path_str);
            encoded_path.set_extension("png");
            encoded_path
        },
    };
    println!("{:?}", out_path);
    let bytes = png.as_bytes();
    fs::write(out_path, bytes)
}

pub fn encode(args: &EncodeArgs) -> ResultAlias<()> {
    let mut png = loadPng(&args.file).expect("Couldn't load png!");
    let chunk = chunk_from_strings(&args.chunk_type, &args.message).unwrap();
    png.append_chunk(chunk);
    produce_out(png, &args.file, &args.output).unwrap();
    Ok(())
}

pub fn decode(path:&PathBuf, chunk_type: &str) -> ResultAlias<()>{
    let png = loadPng(path)?;
    let chunk = png.chunk_by_type(chunk_type).expect("No chunk was found with that type");

    println!("ENCODED MESSAGE:\n{}",String::from_utf8(chunk.data().to_vec()).unwrap());
    Ok(())
}

pub fn remove(path:&PathBuf, chunk_type: &str) -> ResultAlias<()>{
    let mut png = loadPng(path)?;
    png.remove_chunk(chunk_type)?;
    produce_out(png, path, &Some(path.to_owned())).unwrap();
    Ok(())
}

pub fn print(path:&PathBuf) -> ResultAlias<()> {
    let png = loadPng(path)?;
    let mut i = 0;
    for c in png.chunks() {
        println!("Chunk {}:\n{}", i, c);
        i+=1;
    }
    Ok(())
}