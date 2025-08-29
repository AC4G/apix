use std::{fs, io, path::Path};

pub fn copy_dir_recursive(src_dir: &Path, dst_dir: &Path) -> io::Result<()> {
    if !dst_dir.exists() {
        fs::create_dir_all(dst_dir)?;
    }

    if !src_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Source directory {:?} does not exist", src_dir),
        ));
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dst_dir.join(entry.file_name());

        if entry_path.is_dir() {
            copy_dir_recursive(&entry_path, &dest_path)?;
        } else {
            fs::copy(&entry_path, &dest_path)?;
        }
    }

    Ok(())
}

pub fn ensure_dir_and_copy_files(
    dest_dir: &Path,
    src_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    if !dest_dir.exists() {
        fs::create_dir_all(dest_dir)?;
    }

    if !src_dir.exists() {
        return Err(format!(
            "Source directory {} does not exist",
            src_dir.to_str().unwrap()
        )
        .into());
    }

    for entry in fs::read_dir(src_dir)? {
        let entry = entry?;
        let entry_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());

        if entry_path.is_dir() {
            ensure_dir_and_copy_files(&dest_path, &entry_path)?;
        } else {
            if !dest_path.exists() {
                fs::copy(entry_path, dest_path)?;
            }
        }
    }

    Ok(())
}

pub fn create_tmp_folder() -> Result<String, Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let tmp_path = tmp_dir.path().to_str().unwrap().to_string();
    Ok(tmp_path)
}
