use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct EncodeArgs {
   pub file:PathBuf,
   pub chunk_type:String,
   pub message:String,
   pub output:Option<PathBuf>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
   Encode (EncodeArgs),
   Decode {
      file:PathBuf,
      chunk_type:String,
   },
   Remove {
      file:PathBuf,
      chunk_type:String,
   },
   Print {
      file:PathBuf,
   },
}
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Arguments {
   #[command(subcommand)]
   pub command: Command,
}
