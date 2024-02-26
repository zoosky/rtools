use std::fs::{self, File};
use std::io::{self, copy};
use std::path::Path;
use zip::ZipArchive;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let mut archive = ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = file
            .enclosed_name()
            .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "Invalid file path"))?
            .to_owned(); // Clone the path here

        if file.name().ends_with('/') {
            println!("File {} extracted to \"{}\"", i, outpath.display());
            fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            copy(&mut file, &mut outfile)?; // Now `file` is only borrowed here

            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.display(),
                file.size()
            );
        }

        #[cfg(unix)]
        set_permissions(&outpath, file.unix_mode())?;
    }

    Ok(())
}

#[cfg(unix)]
fn set_permissions(outpath: &Path, mode: Option<u32>) -> Result<(), std::io::Error> {
    use std::os::unix::fs::PermissionsExt;
    if let Some(mode) = mode {
        fs::set_permissions(outpath, fs::Permissions::from_mode(mode))?;
    }
    Ok(())
}
