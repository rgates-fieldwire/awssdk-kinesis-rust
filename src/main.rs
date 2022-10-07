use aws_config::meta::region::RegionProviderChain;
use aws_sdk_kinesis::{Client, Error, Region, Endpoint, PKG_VERSION};
use aws_sdk_kinesis::model::{ShardIteratorType, Record};
use aws_sdk_kinesis::output::GetRecordsOutput;
use aws_sdk_kinesis::types::Blob;
use aws_types::credentials::SharedCredentialsProvider;
use aws_types::Credentials;

use time::OffsetDateTime;
use std::time::{Duration, SystemTime};
use structopt::StructOpt;
use http::Uri;

// AWS constants
const DEFAULT_ENDPOINT: &str = "http://localhost:4566";
const DEFAULT_STREAM_NAME: &str = "EventStream";
const DEFAULT_PARTITION_KEY: &str = "Test_part";
const DEFAULT_SHARD_ID: &str = "shardId-000000000000";

// Other Constants
const DATE_FORMAT_STR: &'static str = "%Y-%m-%d][%H:%M:%S";

// Display stream information.
// snippet-start:[kinesis.rust.describe-stream]
async fn show_stream(client: &Client, stream: &str) -> Result<(), Error> {
    let resp = client.describe_stream().stream_name(stream).send().await?;

    let desc = resp.stream_description.unwrap();

    println!("Stream description:");
    println!("  Name:              {}:", desc.stream_name.unwrap());
    println!("  Status:            {:?}", desc.stream_status.unwrap());
    println!("  Open shards:       {:?}", desc.shards.unwrap().len());
    println!(
        "  Retention (hours): {}",
        desc.retention_period_hours.unwrap()
    );
    println!("  Encryption:        {:?}", desc.encryption_type.unwrap());

    Ok(())
}

// Adds a record to a stream.
// snippet-start:[kinesis.rust.put-record]
async fn add_record(client: &Client, stream: &str, key: &str, data: &str) -> Result<(), Error> {
    let blob = Blob::new(data);

    client
        .put_record()
        .data(blob)
        .partition_key(key)
        .stream_name(stream)
        .send()
        .await?;

    println!("Put data into stream.");

    Ok(())
}

// Get Shard Iterator
async fn get_shard_iterator(client: &Client, stream: &str, shard_id: &str) -> Result<String, Error>{
    let resp = client.get_shard_iterator().stream_name(stream).shard_id(shard_id).shard_iterator_type(ShardIteratorType::TrimHorizon).send().await?;

    let shard_it = resp.shard_iterator.unwrap();

    println!("Shard iterator info:");
    println!("              {}:", shard_it);

    Ok(shard_it)
}

async fn get_records(client: &Client, shard_it: &str) -> Result<GetRecordsOutput, Error>{
    let resp = client.get_records().shard_iterator(shard_it).send().await?;

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {

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

    // show_stream(&client, DEFAULT_STREAM_NAME).await.unwrap();

    // let dt: OffsetDateTime = SystemTime::now().into();

    // add_record(&client, DEFAULT_STREAM_NAME, DEFAULT_PARTITION_KEY, "test from rust").await
    
    let iterator = get_shard_iterator(&client, DEFAULT_STREAM_NAME, DEFAULT_SHARD_ID).await.unwrap();

    let resp = get_records(&client, &iterator).await.unwrap();

    match resp.records{
        None => println!("No records"),
        Some(vec) => for record in vec {
            match record.data{
                None => println!("No data in Record"),
                Some(data) => {
                    let ut8Data = data.into_inner();
                    unsafe {
                        println!("Data: {}", std::str::from_utf8_unchecked(&ut8Data));
                    }
                }
            }
        }
    }

    Ok(())
}
