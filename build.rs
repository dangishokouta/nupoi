use clap_complete::Shell;
use clap::{IntoApp, Command};
use std::path::Path;
use std::fs::File;

include!("src/cli.rs");

fn generate(s: Shell, app: &mut Command, outdir: &Path, file: &str) {
    let destfile = outdir.join(file);
    println!("dest: {}", destfile.display());
    std::fs::create_dir_all(destfile.parent().unwrap()).unwrap();
    let mut dest = File::create(destfile).unwrap();

    clap_complete::generate(s, app, "nupoi", &mut dest);
}

fn main() {
    let mut app = Options::command();
    app.set_bin_name("nupoi");

    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("target/completions/");

    generate(Shell::Bash, &mut app, &outdir, "bash/nupoi");
    generate(Shell::Elvish, &mut app, &outdir, "elvish/nupoi");
    generate(Shell::Fish, &mut app, &outdir, "fish/nupoi");
    generate(Shell::PowerShell, &mut app, &outdir, "powershell/nupoi");
    generate(Shell::Zsh, &mut app, &outdir, "zsh/_nupoi");
}
