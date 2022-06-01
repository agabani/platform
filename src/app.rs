use crate::{router, server};

#[allow(clippy::missing_panics_doc)]
pub async fn run() {
    let router = router::create();
    server::create(router).await.unwrap();
}
