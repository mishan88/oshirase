# Oshirase

## about
* Send message by AWS SNS from Lambda

## Requirements
* Serverless Framework
* Serverless Framework Rust Plugin
* Rust 1.40+
* Docker

## Config
* Add your credential `./static/credentials`
* Write your setting `./static/Settings.toml`
    * See example `./static/Settings_template.toml`

## Run
```sh
npx sls invoke local -f oshirase -d '{"last_time": "2020-01-19T12:25:12.773320+09:00"}'
```
