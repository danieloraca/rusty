mod dynamo_db;

#[tokio::main]
async fn main() {
    dynamo_db::perform_dynamodb_operations().await.unwrap();
}
