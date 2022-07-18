use clap::Parser;
use std::path::PathBuf;
use std::fs;

#[derive(Parser)]
#[clap(
    author, version, about
)]
pub struct Options { 

    #[clap(value_name = "File", required = true, help = "対象となるCSVファイルのパス")]
    pub file: PathBuf,
}
