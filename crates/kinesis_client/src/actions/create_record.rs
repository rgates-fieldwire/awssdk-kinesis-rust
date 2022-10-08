use aws_sdk_kinesis::{Client};
use aws_sdk_kinesis::output::PutRecordOutput;
use aws_sdk_kinesis::types::Blob;
use std::error::Error as OtherError;

pub async fn execute(client: &Client, stream: &str, key: &str, data: &str) -> Result<PutRecordOutput, Box<dyn OtherError>>{
    let blob = Blob::new(data);

    let resp = client
        .put_record()
        .data(blob)
        .partition_key(key)
        .stream_name(stream)
        .send()
        .await?;

    Ok(resp)
}