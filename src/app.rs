use crate::{database, dgraph, router, server};

#[allow(clippy::missing_panics_doc)]
pub async fn run() {
    let client = dgraph::Client::new("http://127.0.0.1:8080");
    let database = database::Database::new(&client);
    database.migrate().await;

    let router = router::create();
    server::create(router).await.unwrap();
}
