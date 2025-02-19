use tokio::io::Result as TokioResult;
use lbasedb::Conn;


pub struct AppData {
    pub db: Conn,
}


impl AppData {
    pub async fn new() -> TokioResult<Self> {
        let db = Conn::new("./tmp/db").await?;
        Ok(Self { db })
    }
}
