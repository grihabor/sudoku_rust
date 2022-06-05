use std::path::Path;
extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    let mut bundler: Bundler =
        Bundler::new(Path::new("bin/main.rs"), Path::new("bin/bundle.rs"));
    bundler.crate_name("sudoku_rust");
    bundler.run();
}
