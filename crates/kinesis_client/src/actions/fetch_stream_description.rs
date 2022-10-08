use aws_sdk_kinesis::{Client};
use aws_sdk_kinesis::model::StreamDescription;
use std::error::Error as OtherError;

pub async fn execute(client: &Client, stream: &str) -> Result<StreamDescription, Box<dyn OtherError>>{
    let resp = client.describe_stream().stream_name(stream).send().await?;
    match resp.stream_description{
        Some(sd) => Ok(sd),
        None =>  simple_error::bail!("No discription found")
    }
}