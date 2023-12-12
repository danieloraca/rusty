use aws_sdk_dynamodb::{
    operation::{
        delete_item::DeleteItemOutput
    },
    types::{
        AttributeValue,
    },
    Client,
    Error,
};

use aws_config::BehaviorVersion;

use uuid::Uuid;

pub async fn create_new(age: &str ) -> Result<(), Error> {
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

pub async fn delete_item(id: &str) -> Result<(), Error> {
    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&shared_config);

    //write item to DynamoDan table
    let request = client
        .delete_item()
        .table_name("DynamoDan")
        .key(
            "id",
            AttributeValue::S(String::from(
                id,
            )),
        );
    request.send().await?;

    Ok(())
}
