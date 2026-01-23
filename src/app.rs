// Copyright 2026 Fitrian Musya
// SPDX-License-Identifier: MIT

use ratatui::widgets::ListState;
use serde::Deserialize;
use std::collections::VecDeque;
use std::process::Stdio;
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, BufReader};
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

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct ImageStatus {
    pub image: String,
    pub status: String,
    pub progress: Option<String>,
    pub error: String,
}

#[derive(Deserialize)]
pub struct FilePath {
    filepath: String,
}

#[derive(Deserialize)]
pub struct CPUUsage {
    pub container_id: String,
    pub cpu_percent: f64,
    pub mem_usage: f64,
    pub mem_limit: u64,
    pub mem_percent: f64,
    pub net_rx: u64,
    pub net_tx: u64,
}

#[derive(Default, Clone)]
pub struct NetData {
    pub net_rx: u64,
    pub net_tx: u64,
}

const MAX_POINT: usize = 120;

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
    pub log_rx: Option<tokio::sync::mpsc::Receiver<String>>,
    pub log_scroll: u16,

    // Analytics
    pub analytics_rx: Option<tokio::sync::mpsc::Receiver<CPUUsage>>,
    pub analytics: CPUUsage,
    pub cpu_data: VecDeque<u64>,
    pub mem_data: VecDeque<u64>,
    pub net_data: Vec<NetData>,
    last_scroll: Instant,
    pub scroll_offset: usize,
    last_net_rx: u64,
    last_net_tx: u64,
    last_heartbeat: Instant,
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
            log_rx: None,
            log_scroll: 0,
            analytics_rx: None,
            cpu_data: VecDeque::with_capacity(MAX_POINT),
            last_scroll: Instant::now(),
            scroll_offset: 0,
            analytics: CPUUsage {
                container_id: "".to_string(),
                cpu_percent: 0.0,
                mem_usage: 0.0,
                mem_limit: 0,
                mem_percent: 0.0,
                net_rx: 0,
                net_tx: 0,
            },
            mem_data: VecDeque::with_capacity(60),
            net_data: vec![],
            last_net_rx: 0,
            last_net_tx: 0,
            last_heartbeat: Instant::now(),
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
                let container = self.containers[idx].clone();

                match action {
                    Some(MenuAction::Start) => {
                        self.log.print_mes(
                            LogType::Info,
                            &format!("Creating container: {}", container.name),
                        );

                        let mut args = vec![
                            "create".to_string(),
                            "--image".to_string(),
                            container.image.clone(),
                            "--start".to_string(),
                        ];

                        if !container.container_name.is_empty() {
                            args.push("--name".to_string());
                            args.push(container.container_name.clone());
                        } else if !container.service.is_empty() {
                            args.push("--name".to_string());
                            args.push(format!("{}-{}", container.name, container.service));
                        }

                        if !container.hostname.is_empty() {
                            args.push("--hostname".to_string());
                            args.push(container.hostname.clone());
                        }

                        if !container.ports.is_empty() {
                            args.push("--ports".to_string());
                            args.push(container.ports.clone());
                        }

                        if !container.environment.is_empty() {
                            args.push("--env".to_string());
                            args.push(container.environment.join(","));
                        }

                        if !container.volumes.is_empty() {
                            args.push("--volumes".to_string());
                            args.push(container.volumes.join(","));
                        }

                        if !container.restart.is_empty() {
                            args.push("--restart".to_string());
                            args.push(container.restart.clone());
                        }

                        let (tx, rx) = tokio::sync::mpsc::channel::<String>(100);
                        self.log_rx = Some(rx);
                        self.loading = true;

                        tokio::spawn(async move {
                            if let Ok(mut child) = Command::new("./bin/runner")
                                .args(&args)
                                .stdout(Stdio::piped())
                                .stderr(Stdio::piped())
                                .spawn()
                            {
                                if let Some(stdout) = child.stdout.take() {
                                    let mut reader = BufReader::new(stdout).lines();

                                    while let Ok(Some(line)) = reader.next_line().await {
                                        let _ = tx.send(line).await;
                                    }
                                }
                                let _ = child.wait().await;
                            }
                            let _ = tx.send("__DONE__".to_string()).await;
                        });
                    }
                    Some(MenuAction::Stop) => {
                        self.log
                            .print_mes(LogType::Info, &format!("Stopping: {}", container.name));
                    }
                    Some(MenuAction::Delete) => {
                        self.log
                            .print_mes(LogType::Info, &format!("Deleting: {}", container.name));
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

    pub fn poll_logs(&mut self) -> bool {
        let mut has_updates = false;

        if let Some(ref mut rx) = self.log_rx {
            while let Ok(line) = rx.try_recv() {
                has_updates = true;

                if line == "__DONE__" {
                    self.loading = false;
                    self.log_rx = None;
                    break;
                }

                if let Ok(status) = serde_json::from_str::<ImageStatus>(&line) {
                    match status.status.as_str() {
                        "pulling" => {
                            self.log.print_mes(
                                LogType::Info,
                                &format!("Pulling image: {}", status.image),
                            );
                        }
                        "downloading" => {
                            if let Some(progress) = &status.progress {
                                self.log.print_mes(LogType::Info, progress);
                            } else {
                                self.log.print_mes(LogType::Info, "Downloading...");
                            }
                        }
                        "completed" => {
                            self.log.print_mes(
                                LogType::Info,
                                &format!("Pull complete: {}", status.image),
                            );
                        }
                        "exists" => {
                            self.log.print_mes(
                                LogType::Info,
                                &format!("Image exists: {}", status.image),
                            );
                        }
                        "error" => {
                            self.log.print_mes(
                                LogType::Error,
                                &format!("Pull failed: {}", status.error),
                            );
                        }
                        "running" | "created" => {
                            self.log
                                .print_mes(LogType::Info, &format!("Container {}", status.status));
                        }
                        _ => {
                            self.log.print_mes(LogType::Info, &line);
                        }
                    }
                } else {
                    self.log.print_mes(LogType::Info, &line);
                }
            }
        }

        has_updates
    }

    pub fn scroll_log_left(&mut self) {
        if self.log_scroll > 0 {
            self.log_scroll = self.log_scroll.saturating_sub(5);
        }
    }

    pub fn scroll_log_right(&mut self) {
        let max_scroll = self.log.to_display_string().len() as u16;
        if self.log_scroll < max_scroll {
            self.log_scroll = self.log_scroll.saturating_add(5);
        }
    }

    /* Part of analytics
     *
     * CPU Usage
     * Get cpu usage from x for reading and also supporting for updating cpu scrolling for scroll
     * effect in user interface
     *
     * @param
     * update_cpu_data(u64) ref: main.rs => update_cpu_data(value);
     * update_cpu_scroll(void) ref: ui.rs => update_cpu_scroll();
     * */
    pub fn cpu_push_data(&mut self, value: u64) {
        if self.cpu_data.len() == MAX_POINT {
            self.cpu_data.pop_front();
        }
        self.cpu_data.push_back(value);
    }
    pub fn cpu_data_as_slice(&self) -> Vec<u64> {
        self.cpu_data.iter().copied().collect()
    }
    pub fn update_cpu_scroll(&mut self) {
        if self.cpu_data.is_empty() {
            return;
        }
        let now = Instant::now();
        if now.duration_since(self.last_scroll) > Duration::from_millis(100) {
            self.scroll_offset = (self.scroll_offset + 1) % self.cpu_data.len();
            self.last_scroll = now;
        }
    }
    pub fn mem_push_data(&mut self, value: u64) {
        if self.mem_data.len() == MAX_POINT {
            self.mem_data.pop_front();
        }
        self.mem_data.push_back(value);
    }
    pub fn mem_data_as_slice(&self) -> Vec<u64> {
        self.mem_data.iter().copied().collect()
    }

    pub fn start_analytics_stream(&mut self, container_id: &str) {
        if self.analytics_rx.is_some() {
            return;
        }

        self.cpu_data.clear();
        self.mem_data.clear();
        self.net_data.clear();
        self.last_net_rx = 0;
        self.last_net_tx = 0;

        // Add initial seed data so graph doesn't start empty
        for _ in 0..20 {
            self.cpu_push_data(5);
            self.mem_push_data(10);
            self.net_data.push(NetData {
                net_rx: 5,
                net_tx: 5,
            });
        }

        let (tx, rx) = tokio::sync::mpsc::channel::<CPUUsage>(100);
        self.analytics_rx = Some(rx);

        let container_id = container_id.to_string();

        tokio::spawn(async move {
            if let Ok(mut child) = Command::new("./bin/runner")
                .args(["stream", &container_id])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
            {
                if let Some(stdout) = child.stdout.take() {
                    let mut reader = BufReader::new(stdout).lines();

                    while let Ok(Some(line)) = reader.next_line().await {
                        if let Ok(parsed) = serde_json::from_str::<CPUUsage>(&line) {
                            if tx.send(parsed).await.is_err() {
                                break; // Receiver dropped
                            }
                        }
                    }
                }
                let _ = child.wait().await;
            }
        });
    }

    pub fn poll_analytics(&mut self) -> bool {
        let mut updates = Vec::new();

        if let Some(ref mut rx) = self.analytics_rx {
            while let Ok(usage) = rx.try_recv() {
                updates.push(usage);
            }
        }

        let has_updates = !updates.is_empty();

        if has_updates {
            for usage in updates {
                let cpu_value = (usage.cpu_percent * 100.0).round() as u64;
                self.cpu_push_data(cpu_value.max(1));
                let mem_value = (usage.mem_percent * 100.0).round() as u64;
                self.mem_push_data(mem_value.max(1));

                let is_first_reading = self.last_net_rx == 0 && self.last_net_tx == 0;

                if !is_first_reading {
                    let rx_delta = if usage.net_rx >= self.last_net_rx {
                        usage.net_rx - self.last_net_rx
                    } else {
                        0
                    };
                    let tx_delta = if usage.net_tx >= self.last_net_tx {
                        usage.net_tx - self.last_net_tx
                    } else {
                        0
                    };

                    let rx_scaled = ((rx_delta / 100) as u64).max(5);
                    let tx_scaled = ((tx_delta / 100) as u64).max(5);

                    self.net_data.push(NetData {
                        net_rx: rx_scaled,
                        net_tx: tx_scaled,
                    });
                }

                self.last_net_rx = usage.net_rx;
                self.last_net_tx = usage.net_tx;

                self.analytics = usage;
            }
            self.last_heartbeat = Instant::now();
        } else if !self.cpu_data.is_empty() {
            let now = Instant::now();
            if now.duration_since(self.last_heartbeat) >= Duration::from_millis(500) {
                let last_cpu = *self.cpu_data_as_slice().last().unwrap_or(&1);
                let last_mem = *self.mem_data_as_slice().last().unwrap_or(&1);

                let variation = (last_cpu as i64 * (rand::random::<i64>() % 21 - 10)) / 100;
                let new_cpu = (last_cpu as i64 + variation).max(1) as u64;

                let mem_variation = (last_mem as i64 * (rand::random::<i64>() % 11 - 5)) / 100;
                let new_mem = (last_mem as i64 + mem_variation).max(1) as u64;

                let last_net = self.net_data.last().cloned().unwrap_or(NetData {
                    net_rx: 5,
                    net_tx: 5,
                });
                let rx_base = last_net.net_rx.max(5);
                let tx_base = last_net.net_tx.max(5);

                let rx_var = (rx_base as i64 * (rand::random::<i64>() % 41 - 20)) / 100; // +/- 20%
                let tx_var = (tx_base as i64 * (rand::random::<i64>() % 41 - 20)) / 100;
                let new_rx = (rx_base as i64 + rx_var).max(5) as u64;
                let new_tx = (tx_base as i64 + tx_var).max(5) as u64;

                self.cpu_push_data(new_cpu);
                self.mem_push_data(new_mem);
                self.net_data.push(NetData {
                    net_rx: new_rx,
                    net_tx: new_tx,
                });

                self.last_heartbeat = now;
            }
        }

        // Limit data size to prevent memory overflow
        const MAX_DATA_POINTS: usize = 500;
        if self.cpu_data.len() > MAX_DATA_POINTS {
            self.cpu_data
                .drain(0..self.cpu_data.len() - MAX_DATA_POINTS);
        }
        if self.mem_data.len() > MAX_DATA_POINTS {
            self.mem_data
                .drain(0..self.mem_data.len() - MAX_DATA_POINTS);
        }
        if self.net_data.len() > MAX_DATA_POINTS {
            self.net_data
                .drain(0..self.net_data.len() - MAX_DATA_POINTS);
        }

        has_updates
    }
}
