use aws_sdk_kinesis::{Client, Error};
use aws_sdk_kinesis::output::GetRecordsOutput;

pub async fn execute(client: &Client, shard_it: &str) -> Result<GetRecordsOutput, Error>{
    let resp = client.get_records().shard_iterator(shard_it).send().await?;

    Ok(resp)
}