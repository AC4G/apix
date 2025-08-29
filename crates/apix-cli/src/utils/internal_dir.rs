use std::sync::Arc;

use apix_core::utils::internal_dir::InternalDir;
use smol::lock::OnceCell;

lazy_static::lazy_static! {
    static ref INTERNAL_DIR: OnceCell<Arc<InternalDir>> = OnceCell::new();
}

pub fn init_internal_dir() -> Result<(), Box<dyn std::error::Error>> {
    let internal_dir = match InternalDir::init_internal_dir() {
        Ok(internal_dir) => internal_dir,
        Err(_e) => return Err(Box::from("Failed to initialize internal directory")),
    };

    smol::block_on(async {
        INTERNAL_DIR.set(Arc::new(internal_dir)).await.unwrap();
    });

    Ok(())
}

pub fn get_internal_dir() -> Arc<InternalDir> {
    match INTERNAL_DIR.get() {
        Some(internal_dir) => internal_dir.clone(),
        None => panic!("Internal directory not initialized"),
    }
}
