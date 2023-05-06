use tokio_postgres::{NoTls, types::ToSql};
use dotenv::dotenv;
use std::env;

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

fn get_env_var(env_var: &str) -> String {
    match env::var(env_var) {
        Ok(val) => val,
        Err(e) => {
            println!("{}", e);
            String::from("{e}")
        },
    }
}

pub async fn query(query: QueryBuilder<'_>) -> Result<Vec<tokio_postgres::Row>, ()> {

    dotenv().ok();

    let query_params = match query.query_params {
        Some(x) => x,
        None => &[]
    };

    let connection_str = format!(
        "host={} user={} port={} password={} dbname={}",
        get_env_var("DBHOST"),
        get_env_var("DBUSER"),
        get_env_var("DBPORT"),
        get_env_var("DBPASSWORD"),
        get_env_var("DBNAME")
    );

    let (client, connection) =
        tokio_postgres::connect(&connection_str, NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let rows = client
        .query(query.query_str, query_params)
        .await.unwrap();

    Ok(rows)
}
