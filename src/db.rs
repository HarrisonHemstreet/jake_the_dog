pub mod db {
    use tokio_postgres::{NoTls, types::ToSql};
    use uuid::Uuid;

    pub async fn execute_db_stuff() {

        // Connect to the database.
        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        // Now we can execute a simple statement that just returns its parameter.
        let rows = client
            .query("SELECT * FROM product;", &[])
            .await.unwrap();
        for row in &rows {
            let id: Uuid  = row.get(0);
            println!("id: {:?}", &id);
        }
    }

    pub async fn connect_to_db() -> tokio_postgres::Client {

        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        client
    }

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
}
