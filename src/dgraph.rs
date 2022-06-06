#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Clone)]
struct Api {
    client: hyper::Client<hyper::client::HttpConnector>,
    host: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct ApiRequestCommit {
    keys: Vec<String>,
    preds: Vec<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponse<T> {
    data: Option<T>,
    errors: Option<Vec<ApiResponseError>>,
    extensions: Option<ApiResponseExtensions>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseDataAlter {
    code: String,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseDataCommit {
    code: String,
    message: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseDataMutate {
    code: String,
    message: String,
    queries: Option<()>,
    uids: Option<ApiResponseDataMutateUids>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseDataMutateUids {}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseExtensions {
    server_latency: Option<ApiResponseExtensionsServerLatency>,
    txn: ApiResponseExtensionsTransaction,
    metrics: Option<ApiResponseExtensionsMetrics>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseExtensionsServerLatency {
    parsing_ns: u64,
    processing_ns: u64,
    encoding_ns: Option<u64>,
    assign_timestamp_ns: u64,
    total_ns: u64,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseExtensionsTransaction {
    start_ts: u64,
    commit_ts: Option<u64>,
    keys: Option<Vec<String>>,
    preds: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseExtensionsMetrics {
    num_uids: HashMap<String, u64>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseError {
    message: String,
    extensions: ApiResponseErrorExtensions,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct ApiResponseErrorExtensions {
    code: String,
}

impl Api {
    async fn alter(&self, statement: &str) -> ApiResponse<ApiResponseDataAlter> {
        use hyper::body::Buf as _;

        let request = hyper::Request::builder()
            .uri(format!("{}/alter", self.host))
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/dql")
            .body(hyper::body::Body::from(statement.to_owned()))
            .unwrap();

        let response = self.client.request(request).await.unwrap();

        let buffer = hyper::body::aggregate(response.into_body()).await.unwrap();

        serde_json::from_reader(buffer.reader()).unwrap()
    }

    async fn commit(
        &self,
        start_ts: u64,
        keys: &[String],
        predicates: &[String],
    ) -> ApiResponse<ApiResponseDataCommit> {
        use hyper::body::Buf as _;

        let request = hyper::Request::builder()
            .uri(format!("{}/commit?startTs={}", self.host, start_ts))
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/json")
            .body(hyper::body::Body::from(
                serde_json::to_string(&ApiRequestCommit {
                    keys: keys.to_owned(),
                    preds: predicates.to_owned(),
                })
                .unwrap(),
            ))
            .unwrap();

        let response = self.client.request(request).await.unwrap();

        let buffer = hyper::body::aggregate(response.into_body()).await.unwrap();

        serde_json::from_reader(buffer.reader()).unwrap()
    }

    async fn mutate(
        &self,
        statement: &str,
        commit_now: bool,
    ) -> ApiResponse<ApiResponseDataMutate> {
        use hyper::body::Buf as _;

        let request = hyper::Request::builder()
            .uri(format!("{}/mutate?commitNow={}", self.host, commit_now))
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/rdf")
            .body(hyper::body::Body::from(statement.to_owned()))
            .unwrap();

        let response = self.client.request(request).await.unwrap();

        let buffer = hyper::body::aggregate(response.into_body()).await.unwrap();

        serde_json::from_reader(buffer.reader()).unwrap()
    }

    async fn query<T>(&self, statement: &str) -> ApiResponse<T>
    where
        T: serde::de::DeserializeOwned,
    {
        use hyper::body::Buf as _;

        let request = hyper::Request::builder()
            .uri(format!("{}/query", self.host))
            .method(hyper::Method::POST)
            .header(hyper::header::CONTENT_TYPE, "application/dql")
            .body(hyper::body::Body::from(statement.to_owned()))
            .unwrap();

        let response = self.client.request(request).await.unwrap();

        let buffer = hyper::body::aggregate(response.into_body()).await.unwrap();

        serde_json::from_reader(buffer.reader()).unwrap()
    }
}

pub struct Client {
    api: Api,
}

impl Client {
    pub fn new(host: &str) -> Client {
        Client {
            api: Api {
                client: hyper::Client::new(),
                host: host.to_string(),
            },
        }
    }

    pub async fn alter(&self, statement: &str) {
        let response = self.api.alter(statement).await.data.unwrap();

        if response.code != "Success" {
            todo!("{}", response.code);
        }
    }

    pub async fn mutate(&self, statement: &str) {
        let response = self.api.mutate(statement, true).await.data.unwrap();

        if response.code != "Success" {
            todo!("{}", response.code);
        }
    }

    pub async fn query<T>(&self, statement: &str) -> T
    where
        T: serde::de::DeserializeOwned,
    {
        self.api.query(statement).await.data.unwrap()
    }

    pub fn transaction(&self) -> Transaction {
        Transaction::new(&self.api)
    }
}

pub struct Transaction {
    api: Api,

    start_ts: Option<u64>,
    keys: Vec<String>,
    predicates: Vec<String>,
}

impl Transaction {
    fn new(api: &Api) -> Transaction {
        Transaction {
            api: api.clone(),
            start_ts: None,
            keys: Vec::new(),
            predicates: Vec::new(),
        }
    }

    pub async fn abort(&self) {}

    pub async fn commit(&self) {
        let response = self
            .api
            .commit(self.start_ts.unwrap(), &self.keys, &self.predicates)
            .await;

        let data = response.data.unwrap();
        if data.code != "Success" {
            todo!("{}", data.code);
        }
    }

    pub async fn mutate(&mut self, statement: &str) {
        let response = self.api.mutate(statement, false).await;

        let data = response.data.unwrap();
        if data.code != "Success" {
            todo!("{}", data.code);
        }

        let txn = response.extensions.unwrap().txn;
        self.start_ts = Some(txn.start_ts);
        self.keys.append(&mut txn.keys.unwrap());
        self.predicates.append(&mut txn.preds.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct EmptyData {
        empty: Vec<()>,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct BalancesData {
        balances: Vec<BalanceData>,
    }

    #[derive(Debug, PartialEq, Eq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct BalanceData {
        uid: String,
        balance: String,
    }

    #[tokio::test]
    async fn test() {
        let client = Client::new("http://127.0.0.1:8080");

        // empty read
        let statement = r#"
{
    empty() {}
}"#;

        let result = client.query::<EmptyData>(statement).await;

        assert_eq!(result, EmptyData { empty: Vec::new() });

        // alter
        let statement = r#"
name: string @index(term) .
type Person {
    name
}"#;

        client.alter(statement).await;

        // mutate
        let statement = r#"
{
    set {
        <0x1> <balance> "110" .
        <0x1> <dgraph.type> "Balance" .
        <0x2> <balance> "60" .
        <0x2> <dgraph.type> "Balance" .
    }
}"#;

        client.mutate(statement).await;

        // query
        let statement = r#"
{
    balances(func: type(Balance)) {
        uid
        balance
    }
}"#;

        let result = client.query::<BalancesData>(statement).await;

        assert_eq!(
            result,
            BalancesData {
                balances: vec![
                    BalanceData {
                        uid: "0x1".to_string(),
                        balance: "110".to_string()
                    },
                    BalanceData {
                        uid: "0x2".to_string(),
                        balance: "60".to_string()
                    }
                ]
            }
        );

        // mutate with transaction
        let mut transaction = client.transaction();

        let statement = r#"
{
    set {
        <0x1> <balance> "110" .
        <0x1> <dgraph.type> "Balance" .
        <0x2> <balance> "70" .
        <0x2> <dgraph.type> "Balance" .
    }
}"#;

        transaction.mutate(statement).await;

        // query before commit
        let statement = r#"
{
    balances(func: type(Balance)) {
        uid
        balance
    }
}"#;

        let result = client.query::<BalancesData>(statement).await;

        assert_eq!(
            result,
            BalancesData {
                balances: vec![
                    BalanceData {
                        uid: "0x1".to_string(),
                        balance: "110".to_string()
                    },
                    BalanceData {
                        uid: "0x2".to_string(),
                        balance: "60".to_string()
                    }
                ]
            }
        );

        // commit
        transaction.commit().await;

        // query after commit
        let statement = r#"
{
    balances(func: type(Balance)) {
        uid
        balance
    }
}"#;

        let result = client.query::<BalancesData>(statement).await;

        assert_eq!(
            result,
            BalancesData {
                balances: vec![
                    BalanceData {
                        uid: "0x1".to_string(),
                        balance: "110".to_string()
                    },
                    BalanceData {
                        uid: "0x2".to_string(),
                        balance: "70".to_string()
                    }
                ]
            }
        );
    }
}
