use std::path::Path;
use std::collections::HashMap;
use chrono::{Utc, Duration, DateTime, NaiveTime};
use chrono_tz::Asia::Tokyo;
use serde_json::{Value, json};
use config;

use lambda_runtime::{error::HandlerError, lambda, Context};
use rusoto_core::{Region, HttpClient};
use rusoto_sns::{SnsClient, PublishInput, Sns};
use rusoto_credential::ProfileProvider;

fn main() {
    lambda!(handler)
}

fn handler(
    event: Value,
    _: Context,
) -> Result<Value, HandlerError> {
    let mut settings = config::Config::default();
    settings.merge(config::File::with_name("/opt/Settings.toml")).unwrap();
    let settings_map = settings.try_into::<HashMap<String, String>>().unwrap();

    let start_time: u32 = settings_map["start_time"].parse().unwrap();
    let end_time: u32 = settings_map["end_time"].parse().unwrap();
    let mut last_time = &event["last_time"].to_string().replace("\"", "");
    let duration_minutes: i64 = settings_map["duration_minutes"].parse().unwrap();
    let (is_send, last_time_c) = is_send_message(
        start_time,
        end_time,
        last_time,
        duration_minutes,
        false,
    );

    if is_send {
        send_message(settings_map["message"].to_string(),settings_map["subject"].to_string(), settings_map["topic_arn"].to_string());
        last_time = &last_time_c;
    }

    let return_value = json!({
        "is_send": is_send,
        "last_time": last_time
    });
    Ok(return_value)
}

fn is_send_message(start_time: u32, end_time: u32, last_time: &str, duration_minutes: i64, force: bool) -> (bool, String) {
    let now = Utc::now().with_timezone(&Tokyo);
    let start_time = NaiveTime::from_hms(start_time, 0, 0);
    let end_time = NaiveTime::from_hms(end_time, 0, 0);

    let last_time = DateTime::parse_from_rfc3339(&last_time).unwrap().with_timezone(&Tokyo);
    let next_time = last_time + Duration::minutes(duration_minutes);

    if force || ((now.time() > start_time || now.time() < end_time) && next_time < now) {
        (true, now.to_rfc3339())
    } else {
        (false, last_time.to_rfc3339())
    }
}

fn send_message(message: String, subject: String, topic_arn: String) {
    let path = Path::new("/opt/credentials");
    let profile = ProfileProvider::with_default_configuration(path);

    let client = SnsClient::new_with(HttpClient::new().unwrap(), profile, Region::ApNortheast1);
    let publishinput = PublishInput {
        message: message,
        message_attributes: None,
        message_structure: None,
        phone_number: None,
        subject: Some(subject),
        target_arn: None,
        topic_arn: Some(topic_arn),
    };
    let _result = client.publish(publishinput).sync().unwrap();
}
