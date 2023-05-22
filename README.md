# jake_the_dog

## Purpose of this project:
The purpose of this project is to give newcomers to Rust a basic project to learn from, and even use to start their own projects. Starting Rust can be hard. I'm probably a slower learner than most. It has taken me much longer than I would like to admit in order for me to learn everything in these five files. I know that other people are in this position as well. I hope that this repo can serve as a basic example of how these interlocking technologies work together. I know that I probably have not implemented everything according to best practices, but I did the best with what I had. If you would like me to change anything in here, feel free to open a pull request or ask a question.

## Included technologies:
* The Rust Programming Language
* tokio (Rust runtime)
* tokio-postgres (for connecting to PostgreSQL)
* uuid (Rust crate for UUIDs)
* serde (a crate for working with JSON in Rust)
* actix-web (a framework for creating Rust web services)

## How to generate documentation for this project:
1. run `cargo doc --open`. Running this command will cause Cargo to generate all the relevant documentation for this project. You can also google the documentation via going to docs.rs online and searching for the relevant crate you are interested in.

## How to run this project:
1. make sure you have a `.env` file with the following values set: `DBHOST`, `DBUSER`, `DBPORT`, `DBPASSWORD`, `DBNAME`. All these values will correlate to how you set up PostgreSQL.
2. make sure you have `docker compose` installed (the current version as of writing this is the one lacking the hyphen, so not the `docker-compose` version)
3. run `docker compose up -d`
4. you should be able to hit the routes `localhost:8080/hello/harry` and `localhost:8080/db_test` and receive a JSON response.
5. if you would like to look at the GUI for PostgreSQL (PgAdmin4), then you can go here: `http://localhost:16543`. The username is `test@test.com` and the password is `test`
  i. to add the PostgreSQL server in PgAdmin4, after logging in click on "Add New Server". On the 'General' tab, name the server anything you would like. Next, select the 'Connection' tab. In place of the 'Host name/address', run this command: `ifconfig | grep inet`, and input one of the output ip addresses. For the 'port', input `5440`. For 'Maintenance database', input `root`. For 'Username', input `root`. For 'Password', input `root`. Finally, if everything has been input correctly, you should be able to hit the save button on the modal and you should be connected.
6. In the `docker_postgres_init.sql`, you should see an `INSERT` statement at the bottom of the file. If there isn't any data in the `product` table, then feel free to run that query in PgAdmin. If the `localhost:8080/db_test` endpoint was not working before, it may now.

## What you can learn from this project:
1. how Rust modules work
2. how to connect Rust to PostgreSQL
3. how to use Actix-Web
4. Rust Lifetimes
5. a basic way to setup a project

## About Jake the Dog
wiki: https://adventuretime.fandom.com/wiki/Jake

I chose Jake to be the mascot of this project, because Jake is the most dependable friend a person could have. Jake is also very flexible. Flexibility and dependability are two abilities I look for in a webserver.
