use log::{info, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Config, Logger, Root},
    encode::pattern::PatternEncoder
};

const LOG_PATH: &str = "logs/hta.log";

pub fn setup_log() -> log4rs::Handle {
    //TODO Cleanup needed.

    // Setup
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}"
        )))
        .build();

    let fileout: FileAppender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%H:%M:%S)(local)} - {l}: {m}{n})}"
        )))
        .build(LOG_PATH)
        .unwrap();

    let config: Config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("fileout", Box::new(fileout)))
        .logger(Logger::builder().build("app::backend::db", LevelFilter::Trace))
        .logger(
            Logger::builder()
                .appender("fileout")
                .additive(false)
                .build("app::fileout", LevelFilter::Debug)
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("fileout")
                .build(LevelFilter::Debug)
        )
        .unwrap();

    let handle: log4rs::Handle = log4rs::init_config(config).unwrap();

    return handle;
}
