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

pub async fn get(table: &str, columns: Option<Vec<&str>>, where_columns: Option<Vec<&str>>, where_values: Option<& [& (dyn ToSql + Sync)]>) -> Vec<tokio_postgres::Row> {
    let mut columns_string: String = String::from("");
    let mut values_string: String = String::from("");
    let new_columns = match columns {
        Some(x) => x,
        None => vec!["*"]
    };

    for (index, column) in new_columns.iter().enumerate() {
        columns_string = format!("{columns_string}, {}", column);
        values_string = format!("{values_string}, ${}", index + 1);
    }

    let new_columns_string: String = columns_string.chars().skip(2).collect();
    let mut where_string: String = String::new();

    if let Some(x) = where_columns {
        for (index, column) in x.iter().enumerate() {
            where_string.push_str(&format!("{} = ${}, ", column, index + 1));
        }
    }

    let new_where_string = where_string.get(..where_string.len() - 2).unwrap_or("");
    let query_string = format!("SELECT {} FROM {} WHERE {};", new_columns_string, table, new_where_string);
    query(QueryBuilder::new(&query_string, where_values)).await.unwrap()
}

pub async fn delete(table: &str, where_columns: Option<Vec<&str>>, where_values: Option<& [& (dyn ToSql + Sync)]>) -> Vec<tokio_postgres::Row> {
    let mut where_string: String = String::new();
    if let Some(x) = where_columns {
        for (index, column) in x.iter().enumerate() {
            where_string.push_str(&format!("{} = ${}, ", column, index + 1));
        }
    }
    let mut query_string = format!("DELETE FROM {table} WHERE {}", where_string);
    query_string = query_string.trim_end_matches(", ").to_string();
    query(QueryBuilder::new(&query_string, where_values)).await.unwrap()
}

pub async fn update(
    table: &str,
    set_columns: Option<Vec<&str>>,
    where_columns: Option<Vec<&str>>,
    values: Option<&[& (dyn ToSql + Sync)]>
) -> Vec<tokio_postgres::Row> {
    let mut set_string: String = String::new();

    let mut query_string = format!("UPDATE {}", table);
    let mut counter: i32 = 0;
    if let Some(x) = set_columns {
        // remove this unused variable or remove this warning
        for (index, column) in x.iter().enumerate() {
            set_string.push_str(&format!("{}", column));
            counter += 1;
            set_string.push_str(&format!(" = ${counter}, "))
        }
    }
    set_string = set_string.trim_end_matches(", ").to_string();
    query_string = format!("{query_string} SET {}", set_string);

    let mut where_string: String = String::new();

    if let Some(x) = where_columns {
        // remove this unused variable or remove this warning
        for (index, column) in x.iter().enumerate() {
            where_string.push_str(&format!("{}", column));
            counter += 1;
            where_string.push_str(&format!(" = ${counter}, "))
        }
    }
    where_string = where_string.trim_end_matches(", ").to_string();
    query_string = format!("{query_string} WHERE {}", where_string);

    query(QueryBuilder::new(&query_string, values)).await.unwrap()
}
