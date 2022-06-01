use crate::{router, server};

pub async fn run() {
    let router = router::create();
    server::create(router).await.unwrap();
}
