#[derive(Debug, Clone, PartialEq)]
pub enum Tab {
    Containers,
    Images,
    Deployments,
    Logs,
    Settings,
}

pub struct App {
    pub current_tab: Tab,
}

impl App {
    pub fn new() -> Self {
        Self {
            current_tab: Tab::Containers,
        }
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
