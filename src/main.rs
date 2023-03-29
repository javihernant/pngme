mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;
use chunk_type::ChunkType;
use clap::Parser;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Arguments::parse();

    match &args.command {
        args::Command::Encode(args) => commands::encode(args)?,
        args::Command::Decode { file, chunk_type } => commands::decode(file, chunk_type)?,
        args::Command::Remove { file, chunk_type } => commands::remove(file, chunk_type)?,
        args::Command::Print { file } => commands::print(file)?,
    }

    Ok(())
}