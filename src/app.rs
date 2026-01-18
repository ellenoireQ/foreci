// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

use ratatui::widgets::ListState;
use serde::Deserialize;
use std::process::Stdio;
use tokio::process::Command;

use crate::log::log::{LogList, LogType};

#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Containers,
    Images,
    Deployments,
    Logs,
    Settings,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MenuAction {
    Start,
    Stop,
    Delete,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DockerCompose {
    pub name: String,
    pub service: String,
    pub image: String,
    pub ports: String,
    pub container_name: String,
    pub hostname: String,
    pub build_context: String,
    pub dockerfile: String,
    pub environment: Vec<String>,
    pub volumes: Vec<String>,
    pub networks: Vec<String>,
    pub restart: String,
}

#[derive(Debug, Clone)]
pub struct DockerImage {
    pub repository: String,
    pub tag: String,
    pub image_id: String,
    pub created: String,
    pub size: String,
}

#[derive(Deserialize)]
struct ImageJson {
    repository: String,
    tag: String,
    image_id: String,
    created: String,
    size: String,
}

#[derive(Deserialize)]
pub struct FilePath {
    filepath: String,
}

pub struct App {
    pub current_tab: Tab,
    pub containers: Vec<DockerCompose>,
    pub container_state: ListState,
    pub container_idx: Option<usize>,
    pub loading: bool,
    pub log: LogList,
    pub expanded_index: Option<usize>,
    pub menu_selection: usize,
    pub details_state: bool,
    pub images: Vec<DockerImage>,
    pub image_state: ListState,
    pub image_idx: Option<usize>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_tab: Tab::Containers,
            containers: Vec::new(),
            container_state: ListState::default(),
            container_idx: Some(0),
            loading: false,
            log: LogList::default(),
            expanded_index: None,
            menu_selection: 0,
            details_state: false,
            images: Vec::new(),
            image_state: ListState::default(),
            image_idx: None,
        }
    }
}

impl App {
    pub async fn fetch_containers(&mut self) {
        let mut results: Vec<FilePath> = vec![];
        self.loading = true;
        self.containers.clear();
        if let Ok(output) = Command::new("./bin/runner")
            .args(["search", "docker-compose.yml"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Ok(parse) = serde_json::from_str::<FilePath>(line) {
                    results.push(parse);
                }
            }
        }

        for file_path in &results {
            let full_path = format!("{}/docker-compose.yml", file_path.filepath);
            if let Ok(output) = Command::new("./bin/runner")
                .args(["read", "compose", &full_path])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output()
                .await
            {
                let stdout = String::from_utf8_lossy(&output.stdout);
                for line in stdout.lines() {
                    self.log.print_mes(LogType::Info, line);
                    if let Ok(parsed) = serde_json::from_str::<DockerCompose>(line) {
                        self.containers.push(parsed);
                    }
                }
            }
        }

        self.loading = false;
    }

    pub async fn fetch_images(&mut self) {
        self.loading = true;
        self.images.clear();

        if let Ok(output) = Command::new("./bin/runner")
            .args(["images"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            let stdout = String::from_utf8_lossy(&output.stdout);
            for line in stdout.lines() {
                if let Ok(parsed) = serde_json::from_str::<ImageJson>(line) {
                    let image = DockerImage {
                        repository: parsed.repository,
                        tag: parsed.tag,
                        image_id: parsed.image_id,
                        created: parsed.created,
                        size: parsed.size,
                    };
                    self.images.push(image);
                }
            }
        }

        if self.images.is_empty() {
            self.log.print_mes(LogType::Info, "No Docker images found");
        } else {
            self.log.print_mes(
                LogType::Info,
                &format!("Found {} images", self.images.len()),
            );
            self.image_state.select(Some(0));
            self.image_idx = Some(0);
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
        self.container_idx = Some(i);
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
        self.container_idx = Some(i);
    }

    pub fn select_next_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        let i = match self.image_state.selected() {
            Some(i) => {
                if i >= self.images.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.image_state.select(Some(i));
        self.image_idx = Some(i);
    }

    pub fn select_prev_image(&mut self) {
        if self.images.is_empty() {
            return;
        }
        let i = match self.image_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.images.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.images.len() - 1,
        };
        self.image_state.select(Some(i));
        self.image_idx = Some(i);
    }

    pub fn toggle_expand(&mut self) {
        if let Some(selected) = self.container_state.selected() {
            if self.expanded_index == Some(selected) {
                self.expanded_index = None;
                self.menu_selection = 0;
            } else {
                self.expanded_index = Some(selected);
                self.menu_selection = 0;
            }
        }
    }

    pub fn menu_next(&mut self) {
        if self.expanded_index.is_some() {
            self.menu_selection = (self.menu_selection + 1) % 3;
        }
    }

    pub fn menu_prev(&mut self) {
        if self.expanded_index.is_some() {
            self.menu_selection = if self.menu_selection == 0 {
                2
            } else {
                self.menu_selection - 1
            };
        }
    }

    pub fn get_menu_action(&self) -> Option<MenuAction> {
        if self.expanded_index.is_some() {
            Some(match self.menu_selection {
                0 => MenuAction::Start,
                1 => MenuAction::Stop,
                _ => MenuAction::Delete,
            })
        } else {
            None
        }
    }

    pub async fn execute_menu_action(&mut self) {
        if let Some(idx) = self.expanded_index {
            if idx < self.containers.len() {
                let action = self.get_menu_action();
                let container_name = self.containers[idx].name.clone();

                match action {
                    Some(MenuAction::Start) => {
                        self.log
                            .print_mes(LogType::Info, &format!("Starting: {}", container_name));
                    }
                    Some(MenuAction::Stop) => {
                        self.log
                            .print_mes(LogType::Info, &format!("Stopping: {}", container_name));
                    }
                    Some(MenuAction::Delete) => {
                        self.log
                            .print_mes(LogType::Info, &format!("Deleting: {}", container_name));
                        self.containers.remove(idx);
                        if self.containers.is_empty() {
                            self.container_state.select(None);
                        } else if idx >= self.containers.len() {
                            self.container_state.select(Some(self.containers.len() - 1));
                        }
                    }
                    None => {}
                }
                self.expanded_index = None;
                self.menu_selection = 0;
            }
        }
    }

    pub fn toggle_details(&mut self) {
        self.details_state = true;
        let i = format!("Boolean state: {}", self.details_state);
        self.log.print_mes(LogType::Info, i.as_str());
    }
}
