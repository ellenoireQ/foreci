use std::ptr::null;

use tokio::time::error::Elapsed;

pub enum LogType {
    Info,
    Error,
    Warning,
}

#[derive(Clone)]
pub struct Log {
    pub status: String,
    pub message: String,
}

pub struct LogList {
    log: Vec<Log>,
}

impl Default for LogList {
    fn default() -> Self {
        Self {
            log: vec![Log {
                status: "".to_string(),
                message: "".to_string(),
            }],
        }
    }
}

impl LogList {
    pub fn print_mes(&mut self, status: LogType, message: &str) {
        let matching = match status {
            LogType::Info => "[Info]",
            LogType::Error => "[Error]",
            LogType::Warning => "[Warning]",
        };
        let ul = Log {
            status: matching.to_string(),
            message: message.to_string(),
        };
        if (self.log.is_empty()) {
            self.log.push(ul);
        } else {
            self.log.pop();
        }
    }

    pub fn get_logs(&self) -> &Vec<Log> {
        &self.log
    }

    pub fn to_display_string(&self) -> String {
        self.log
            .iter()
            .filter(|l| !l.status.is_empty() || !l.message.is_empty())
            .map(|l| format!("{} {}", l.status, l.message))
            .collect::<Vec<_>>()
            .join(" | ")
    }
}
