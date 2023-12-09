mod dynamo_db;

use clap::{App, Arg};

#[tokio::main]
async fn main() {
    let matches = App::new("DynamoDB")
        .version("1.0")
        .author("Daniel Oraca")
        .about("DynamoDB CLI program")
        .arg(
            Arg::with_name("age")
                .short("a")
                .long("age")
                .value_name("AGE")
                .help("Sets the age")
                .takes_value(true),
        )
        .get_matches();

    let arg_age = matches.value_of("age").unwrap_or("0");

    dynamo_db::perform_dynamodb_operations(arg_age).await.unwrap();
}
