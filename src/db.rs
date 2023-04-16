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

    // if I don't need any query params, then I can just pass &[] in place of the `query_params` param.
    pub async fn query(query_str: &str, query_params: &[&(dyn ToSql + Sync)]) -> Vec<tokio_postgres::Row> {
        let (client, connection) =
            tokio_postgres::connect("host=10.0.0.154 user=root port=5440 password=root dbname=root", NoTls).await.unwrap();

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let rows = client
            .query(query_str, query_params)
            .await.unwrap();
        rows
    }
}
