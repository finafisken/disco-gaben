use rusoto_dynamodb::DynamoDbClient;
use typemap::Key;

pub struct DbClient;

impl Key for DbClient {
    type Value = DynamoDbClient;
}
