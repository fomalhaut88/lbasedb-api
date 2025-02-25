use tokio::io::Result as TokioResult;
use lbasedb::Conn;


pub struct AppData {
    pub db: Conn,
}


impl AppData {
    pub async fn new(data_path: &str) -> TokioResult<Self> {
        let db = Conn::new(data_path).await?;
        Ok(Self { db })
    }
}
