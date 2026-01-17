// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

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
    log: Log,
}

impl Default for LogList {
    fn default() -> Self {
        Self {
            log: Log {
                status: "".to_string(),
                message: "".to_string(),
            },
        }
    }
}

impl LogList {
    pub async fn print_mes(&mut self, status: LogType, message: &str) {
        let matching = match status {
            LogType::Info => "[Info]",
            LogType::Error => "[Error]",
            LogType::Warning => "[Warning]",
        };
        self.log = Log {
            status: matching.to_string(),
            message: message.to_string(),
        };
    }

    pub fn to_display_string(&self) -> String {
        if self.log.status.is_empty() && self.log.message.is_empty() {
            String::new()
        } else {
            format!("{} {}", self.log.status, self.log.message)
        }
    }
}
