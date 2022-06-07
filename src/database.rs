use crate::{dgraph::Client, secret::Secret};

pub struct Database {
    client: Client,
}

impl Database {
    pub fn new(client: &Client) -> Database {
        Database {
            client: client.clone(),
        }
    }

    pub async fn destroy(&self) {
        self.client
            .alter(
                r#"
{
    "drop_all": true
}
    "#,
            )
            .await
    }

    pub async fn migrate(&self) {
        self.client
            .alter(
                r#"
account.username: string @index(term) @upsert .
account.password: password .

type Account {
    account.username: ID!
    account.password
}
        "#,
            )
            .await
    }

    pub async fn signup(&self, username: &str, password: &Secret) {
        let mut transaction = self.client.transaction();

        transaction
            .mutate(&format!(
                r#"
{{
    set {{


    }}
}}
        "#,
            ))
            .await;

        transaction.commit().await;
    }
}
