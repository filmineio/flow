use chrono::Local;
use either::{Either, Left, Right};
use log::Record;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;

#[derive(Serialize, Debug)]
pub struct LogMessage<'a> {
    pub timestamp: String,
    pub level: String,
    pub msg: fmt::Arguments<'a>,
    pub logger: String,
    pub line: String,
}

pub trait MessageOutput {
    fn json(&self) -> Value;
    fn log(&self) -> String;
    fn print(&self, format: LoggerFormat) -> Either<Value, String>;
}

pub trait From<'a> {
    fn from_record(record: &Record<'a>) -> Self;
}

impl<'a> From<'a> for LogMessage<'a> {
    fn from_record(record: &Record<'a>) -> Self {
        LogMessage {
            timestamp: Local::now().format("%Y-%m-%dT%H:%M:%S").to_string(),
            level: record.level().to_string(),
            msg: *record.args(),
            logger: record.target().to_string(),
            line: record.line().unwrap_or(0).to_string(),
        }
    }
}

impl<'a> MessageOutput for LogMessage<'a> {
    fn json(&self) -> Value {
        serde_json::json!(self)
    }

    fn log(&self) -> String {
        format!(
            "{} [{}] - {}(line:{}) - {}",
            self.timestamp, self.level, self.logger, self.line, self.msg
        )
    }

    fn print(&self, format: LoggerFormat) -> Either<Value, String> {
        match format {
            LoggerFormat::JSON => Left(self.json()),
            LoggerFormat::Standard => Right(self.log()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum LoggerFormat {
    JSON,
    Standard,
}
