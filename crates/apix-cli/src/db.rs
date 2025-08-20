use std::sync::Arc;
use apix_core::Db;
use smol::lock::OnceCell;
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
    let db_result = Db::new(db_path);
    let db = match db_result {
        Ok(db) => db,
        Err(e) => return Err(DbConnectError::ConnectError(format!("{:?}", e))),
    };

    smol::block_on(async {
        DB_INSTANCE.set(Arc::new(db))
            .await
            .map_err(|_| DbConnectError::AlreadyConnected).expect("Failed to set DB instance"); 
    });

    Ok(())
}

pub fn get_db() -> Arc<Db> {
    smol::block_on(async {
        DB_INSTANCE.get()
            .cloned()
            .expect("DB not initialized")
    })
}
