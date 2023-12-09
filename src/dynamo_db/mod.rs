use aws_sdk_dynamodb::{
    Client, Error,
};

use aws_sdk_dynamodb::types::{
    AttributeValue,
};

use aws_config::BehaviorVersion;

use uuid::Uuid;

pub async fn perform_dynamodb_operations(age: &str ) -> Result<(), Error> {
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);
    let res = client.list_tables().send().await?;

    println!("Tables: {:?}", res.table_names);

    let id = Uuid::new_v4();
    //write item to DynamoDan table
    let request = client
        .put_item()
        .table_name("DynamoDan")
        .item(
            "id",
            AttributeValue::S(String::from(
                id,
            )),
        )
        .item(
            "ageR",
            AttributeValue::N(String::from(
                age
            )),
        )
        .item("name", AttributeValue::S(String::from("Dan")));
    request.send().await?;

    Ok(())
}
