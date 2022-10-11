use aws_sdk_kinesis::model::record;
use aws_sdk_kinesis::{Client, Error, Region, Endpoint, PKG_VERSION};
use aws_sdk_kinesis::output::GetRecordsOutput;
use aws_sdk_kinesis::types::Blob;
use aws_types::credentials::SharedCredentialsProvider;
use aws_types::Credentials;
use aws_sdk_kinesis::model::Record;

use kinesis_client::Event;
use kinesis_client::actions::{
    fetch_stream_description,
    create_record,
    fetch_shard_iterator,
    fetch_records,
};

use kinesis_client::json::{
    event_wrapper::EventWrapper,
};

use http::Uri;
use color_eyre::eyre::Result;
use std::fmt;
use std::error;

// AWS constants
const DEFAULT_ENDPOINT: &str = "http://localhost:4566";
const DEFAULT_STREAM_NAME: &str = "EventStream";
const DEFAULT_PARTITION_KEY: &str = "Test_part";
const DEFAULT_SHARD_ID: &str = "shardId-000000000001";

type CustomResult<T> = std::result::Result<T, Box<dyn error::Error>>;
#[derive(Debug, Clone)]
struct BasicError;

impl fmt::Display for BasicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oops something went wrong")
    }
}

impl error::Error for BasicError {}

fn display_records(records: Option<&[Record]>){
    match records{
        None => println!("No records"),
        Some(vec) => for record in vec {
            match &record.data{
                None => println!("No data in Record"),
                Some(data) => {
                    let ut8_data = data.clone().into_inner();
                    println!("Data: {}", String::from_utf8_lossy(&ut8_data));
                }
            }
        }
    }
}
fn process_records(records: Option<&[Record]>) -> CustomResult<Vec<Event>>{
    let mut v: Vec<Event> = vec![];
    match records{
        None => println!("No records"),
        Some(rec) => for record in rec {
            let ew = decode_events_from_record(record.clone()).unwrap();
            print!("{} ", ew);
            let e = ew.get_event().unwrap();
            v.push(e)
        }
    }
    Ok(v)
}

fn decode_events_from_record(record: Record) -> CustomResult<EventWrapper> {
    match &record.data{
        None => return Err(BasicError.into()),
        Some(data) => {
            let ut8_data = data.clone().into_inner();
            let result= serde_json::from_str(&String::from_utf8_lossy(&ut8_data));
            match result{
                Ok(value) => return Ok(value),
                Err(e) => return Err(BasicError.into())
            }
        }
    }
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
    
    let mut iterator: String = fetch_shard_iterator::execute(&client, DEFAULT_STREAM_NAME, DEFAULT_SHARD_ID).await.unwrap();

    let mut resp = fetch_records::execute(&client, &iterator).await.unwrap();

    // display_records(resp.records());

    let event = Event{
        project_id: "Test".to_string(),
        account_id: "Test 2".to_string(),
        user_id: 0,
        message: "foobar".to_string()
    };

    // dbg!(&event);

    let serialized = serde_json::to_string(&event).unwrap();

    // dbg!(&serialized);

    // let deserialized: Event = serde_json::from_str(&serialized).unwrap();

    // dbg!(&deserialized);

    let ew: EventWrapper = EventWrapper{
        id: "Test Id".to_string(),
        event_type: 0,
        event_json: serialized,
    };

    print!("{}", ew);
    let sew = serde_json::to_string(&ew).unwrap();

    // dbg!(&sew);

    // let dew: EventWrapper = serde_json::from_str(&sew).unwrap();
    
    // dbg!(dew.event_json);


    let _result = create_record::execute(&client, DEFAULT_STREAM_NAME, DEFAULT_PARTITION_KEY, &sew).await.unwrap();

    dbg!(_result);

    iterator = fetch_shard_iterator::execute(&client, DEFAULT_STREAM_NAME, DEFAULT_SHARD_ID).await.unwrap();
    resp = fetch_records::execute(&client, &iterator).await.unwrap();

     display_records(resp.records());
    
    let _unused = process_records(resp.records());

    Ok(())
}
