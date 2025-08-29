use apix_core::db::Db;
use smol::lock::OnceCell;
use std::{path::Path, sync::Arc};
use thiserror::Error;

lazy_static::lazy_static! {
    static ref DB_INSTANCE: OnceCell<Arc<Db>> = OnceCell::new();
}

#[derive(Debug, Error)]
pub enum DbConnectError {
    #[error("Failed to connect to DB: {0}")]
    ConnectError(String),
    #[error("DB is already connected")]
    AlreadyConnected,
}

pub fn connect_db(db_path: &str) -> Result<(), DbConnectError> {
    let path = Path::new(db_path);

    let db = if !path.exists() {
        Db::create_db_and_migrate(path).map_err(|e| {
            DbConnectError::ConnectError(format!("create_db_and_migrate failed: {:?}", e))
        })?
    } else {
        Db::new(db_path)
            .map_err(|e| DbConnectError::ConnectError(format!("Db::new failed: {:?}", e)))?
    };

    smol::block_on(async {
        DB_INSTANCE
            .set(Arc::new(db))
            .await
            .map_err(|_| DbConnectError::AlreadyConnected)
            .expect("Failed to set DB instance");
    });

    Ok(())
}

pub fn get_db() -> Arc<Db> {
    smol::block_on(async { DB_INSTANCE.get().cloned().expect("DB not initialized") })
}
