use ratatui::widgets::ListState;
use serde::Deserialize;
use std::process::Stdio;
use tokio::process::Command;

use crate::log::log::{Log, LogList, LogType};

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
    pub container_state: ListState,
    pub loading: bool,
    pub log: LogList,
}

pub struct ImageList {
    pub items: Vec<Container>,
    pub state: ListState,
}
impl Default for App {
    fn default() -> Self {
        Self {
            current_tab: Tab::Containers,
            containers: Vec::new(),
            container_state: ListState::default(),
            loading: false,
            log: LogList::default(),
        }
    }
}

impl App {
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

    pub async fn delete(&mut self) {
        self.containers.pop();
    }

    pub fn select_next_container(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.container_state.selected() {
            Some(i) => {
                if i >= self.containers.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.container_state.select(Some(i));
    }

    pub fn select_prev_container(&mut self) {
        if self.containers.is_empty() {
            return;
        }
        let i = match self.container_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.containers.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.containers.len() - 1,
        };
        self.container_state.select(Some(i));
    }
}
