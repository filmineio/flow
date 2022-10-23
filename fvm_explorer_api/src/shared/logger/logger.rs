use crate::shared::logger::types::{From, LogMessage, LoggerFormat, MessageOutput};
use env_logger::Builder;
use std::io::Write;

pub struct Logger {}

pub trait Init {
    fn init(format: LoggerFormat) -> ();
}

impl Init for Logger {
    fn init(format: LoggerFormat) -> () {
        let mut builder = Builder::from_default_env();
        builder
            .format(move |buf, record| {
                let message = LogMessage::from_record(&record);
                writeln!(buf, "{}", message.print(format))
            })
            .init();
    }
}
