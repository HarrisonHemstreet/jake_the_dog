    use tokio_postgres::{NoTls, types::ToSql};

    pub struct QueryBuilder<'a> {
        query_str: &'a str,
        // query params are not always needed. Sometimes you just want to do a simple select * from
        // x table.
        query_params: Option<&'a [&'a (dyn ToSql + Sync)]>
    }

    impl<'a> QueryBuilder<'a> {
        pub fn new(query_str: &'a str, query_params: Option<&'a [&'a (dyn ToSql + Sync)]>) -> Self {
            Self {
                query_str,
                query_params
            }
        }
    }

    pub async fn query(query: QueryBuilder<'_>) -> Vec<tokio_postgres::Row> {

        let query_params = match query.query_params {
            Some(x) => x,
            None => &[]
        };

        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        let rows = client
            .query(query.query_str, query_params)
            .await.unwrap();

        rows
    }
