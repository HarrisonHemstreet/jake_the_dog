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

pub async fn insert(table: &str, columns: Vec<&str>, values: Option<& [& (dyn ToSql + Sync)]>) -> Vec<tokio_postgres::Row> {
    let mut columns_string: String = String::from("");
    let mut values_string: String = String::from("");
    for (index, column) in columns.iter().enumerate() {
        columns_string = format!("{columns_string}, {column}");
        values_string = format!("{values_string}, ${}", index + 1);
    }
    let new_columns_string: String = columns_string.chars().skip(2).collect();
    let new_values_string: String = values_string.chars().skip(2).collect();
    let query_string = format!("INSERT INTO {} ({}) VALUES ({})", table, new_columns_string, new_values_string);
    query(QueryBuilder::new(&query_string, values)).await.unwrap()
}

pub async fn get_all(table: &str) -> Vec<tokio_postgres::Row> {
    let query_string: String = format!("SELECT * FROM {};", table);
    query(QueryBuilder::new(&query_string, None)).await.unwrap()
}
