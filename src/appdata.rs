use tokio::io::Result as TokioResult;
use lbase2::Connection;


pub struct AppData {
    pub db: Connection,
}


impl AppData {
    pub async fn new() -> TokioResult<Self> {
        let db = Connection::new("./tmp/db").await?;
        Ok(Self { db })
    }
}
