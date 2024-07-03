pub enum LoggerLevel {
    #[allow(dead_code)]
    Debug,
    #[allow(dead_code)]
    Info,
    #[allow(dead_code)]
    Warn,
    Error,
}

pub fn logger(level: LoggerLevel, msg: &str) -> String {
    let prefix = match level {
        LoggerLevel::Debug => "debug",
        LoggerLevel::Info => "info",
        LoggerLevel::Warn => "warn",
        LoggerLevel::Error => "error",
    };

    format!("[{}]: {}", prefix, msg)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_logger() {
        assert_eq!("[debug]: debug message", &logger(LoggerLevel::Debug, "debug message"));
        assert_eq!("[info]: info message", &logger(LoggerLevel::Info, "info message"));
        assert_eq!("[warn]: warn message", &logger(LoggerLevel::Warn, "warn message"));
        assert_eq!("[error]: error message", &logger(LoggerLevel::Error, "error message"));
    }
}
