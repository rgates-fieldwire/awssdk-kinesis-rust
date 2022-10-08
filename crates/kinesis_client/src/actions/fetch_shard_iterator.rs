use aws_sdk_kinesis::{Client, Error};
use aws_sdk_kinesis::model::{ShardIteratorType};

pub async fn execute(client: &Client, stream: &str, shard_id: &str) -> Result<String, Error>{
    let resp = client.get_shard_iterator().stream_name(stream).shard_id(shard_id).shard_iterator_type(ShardIteratorType::TrimHorizon).send().await?;

    let shard_it = resp.shard_iterator.unwrap();

    Ok(shard_it)
}