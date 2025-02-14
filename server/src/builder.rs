use std::process::Command;
use std::path::PathBuf;
use std::io::{ Result, Error, ErrorKind};

pub fn build_wasm(path: PathBuf) -> Result<()> {
    let status = Command::new("wasm-pack")
                    .current_dir(path)
                    .args(["build", "--target", "web"])
                    .status()?;
    

    if !status.success() {
        return Err(Error::new(ErrorKind::Other, "WASM build failed"));
    }
    Ok(())
}


