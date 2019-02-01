use fs_extra::{copy_items, dir::CopyOptions};
use std::env;

pub fn main() -> Result<(), fs_extra::error::Error> {
    if let Ok(_) = env::var("CI") {
        return Ok(());
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let final_path = &out_dir[..out_dir.find("/build").unwrap()];
    let mut options = CopyOptions::new();
    options.overwrite = true;
    copy_items(&vec!["lib"], final_path, &options)?;
    Ok(())
}
