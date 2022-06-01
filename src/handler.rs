pub mod accounts {
    use axum::extract::Json;

    use crate::secret::Secret;

    #[derive(Debug, serde::Deserialize)]
    pub struct SignupRequest {
        #[allow(dead_code)]
        username: Secret,

        #[allow(dead_code)]
        password: Secret,
    }

    #[allow(clippy::unused_async)]
    pub async fn signup(Json(payload): Json<SignupRequest>) {
        println!("request: {:?}", payload);
    }
}

pub mod health {
    #[allow(clippy::unused_async)]
    pub async fn liveness() {}

    #[allow(clippy::unused_async)]
    pub async fn readiness() {}
}
