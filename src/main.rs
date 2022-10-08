use aws_sdk_kinesis::{Client, Error, Region, Endpoint, PKG_VERSION};
use aws_sdk_kinesis::output::GetRecordsOutput;
use aws_sdk_kinesis::types::Blob;
use aws_types::credentials::SharedCredentialsProvider;
use aws_types::Credentials;

use kinesis_client::actions::{
    fetch_stream_description,
    create_record,
    fetch_shard_iterator,
    fetch_records,
};

use http::Uri;
use color_eyre::eyre::Result;

// AWS constants
const DEFAULT_ENDPOINT: &str = "http://localhost:4566";
const DEFAULT_STREAM_NAME: &str = "EventStream";
const DEFAULT_PARTITION_KEY: &str = "Test_part";
const DEFAULT_SHARD_ID: &str = "shardId-000000000000";

async fn get_records(client: &Client, shard_it: &str) -> Result<GetRecordsOutput, Error>{
    let resp = client.get_records().shard_iterator(shard_it).send().await?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let shared_config = aws_config::from_env().region(Region::new("us-east-1")).endpoint_resolver(Endpoint::immutable(
        Uri::try_from(DEFAULT_ENDPOINT).expect("Passed in AWS endpoint URI is invalid"),
    )).credentials_provider(SharedCredentialsProvider::new(Credentials::new(
        "test",
        "test",
        None,
        None,
        "fw-env-injected",
    ))).load().await;

    println!("Kinesis client version: {}", PKG_VERSION);

    let client = Client::new(&shared_config);

    //let stream_dis = fetch_stream_description::execute(&client, DEFAULT_STREAM_NAME).await.unwrap();

    //dbg!(stream_dis);

    //let current_time = chrono::offset::Local::now().to_string();
    //let result = create_record::execute(&client, DEFAULT_STREAM_NAME, DEFAULT_PARTITION_KEY, &current_time).await.unwrap();

    //dbg!(result);
    
    let iterator = fetch_shard_iterator::execute(&client, DEFAULT_STREAM_NAME, DEFAULT_SHARD_ID).await.unwrap();

    let resp = fetch_records::execute(&client, &iterator).await.unwrap();

    match resp.records{
        None => println!("No records"),
        Some(vec) => for record in vec {
            match record.data{
                None => println!("No data in Record"),
                Some(data) => {
                    let ut8_data = data.into_inner();
                    println!("Data: {}", String::from_utf8_lossy(&ut8_data));
                }
            }
        }
    }

    Ok(())
}
