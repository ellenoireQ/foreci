use serde::Deserialize;
use std::process::Stdio;
use tokio::process::Command;

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Containers,
    Images,
    Deployments,
    Logs,
    Settings,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Container {
    pub name: String,
    pub dockerfile: String,
    pub status: String,
}

pub struct App {
    pub current_tab: Tab,
    pub containers: Vec<Container>,
    pub loading: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Containers,
            containers: Vec::new(),
            loading: false,
        }
    }

    pub async fn fetch_containers(&mut self) {
        self.loading = true;
        self.containers.clear();

        if let Ok(output) = Command::new("./bin/runner")
            .args(["run", "test"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Ok(parsed) = serde_json::from_str::<Container>(line) {
                    self.containers.push(parsed);
                }
            }
        }

        self.loading = false;
    }

    pub fn next_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Containers => Tab::Images,
            Tab::Images => Tab::Deployments,
            Tab::Deployments => Tab::Logs,
            Tab::Logs => Tab::Settings,
            Tab::Settings => Tab::Containers,
        };
    }

    pub fn prev_tab(&mut self) {
        self.current_tab = match self.current_tab {
            Tab::Containers => Tab::Settings,
            Tab::Images => Tab::Containers,
            Tab::Deployments => Tab::Images,
            Tab::Logs => Tab::Deployments,
            Tab::Settings => Tab::Logs,
        };
    }
}
