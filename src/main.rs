use clap::Parser;

// Parserを継承した構造体はArgの代わりに使用することが可能。
#[derive(Parser)]
#[clap(
    name = "nupoi",
    author = "Kota Dangisho",
    version = "0.1.0",
    about = "Rust CLI nupoi"
)]
struct AppArg {

    /// check for null
    #[clap(short = 'p', long = "point")]
    point: Option<String>,

    /// file path
    path: std::path::PathBuf,
}

fn main() {
    let _arg: AppArg = AppArg::parse();
    
}

fn hello(name: Option<String>) -> String {
    return format!("Hello, {}", if let Some(n) = name {
        n
    } else {
        "World".to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!("Hello, World", hello(None));
        assert_eq!("Hello, Tamada", hello(Some("Tamada".to_string())));
    }
}