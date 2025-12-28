use std::time::{Duration, Instant};

use crate::sys_info::{ProcessSort, SystemInfo};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum View {
    System,
    Process,
    Resources,
    Network,
    Disks,
    Options,
}

pub struct App {
    pub current_view: View,
    pub metrics: SystemInfo,
    pub scroll_offset: usize,
    pub process_scroll_offset: usize,
    pub selected_process: usize,
    pub show_help: bool,
    pub paused: bool,
    pub update_interval: Duration,
    pub last_update: Instant,
    pub process_sort: ProcessSort,
    pub sort_reverse: bool,
    pub show_full_command: bool,
    pub show_tree_view: bool,
    pub show_proc_details: bool,
    pub proc_aggregated: bool,
    pub max_processes: usize,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_view: View::System,
            metrics: SystemInfo::default(),
            scroll_offset: 0,
            process_scroll_offset: 0,
            selected_process: 0,
            show_help: false,
            paused: false,
            update_interval: Duration::from_millis(1000),
            last_update: Instant::now(),
            process_sort: ProcessSort::Cpu,
            sort_reverse: true,
            show_full_command: false,
            show_tree_view: false,
            show_proc_details: false,
            proc_aggregated: false,
            max_processes: 20,
        }
    }
}

impl App {
    pub fn update_metrics(&mut self) {
        if self.paused || Instant::now().duration_since(self.last_update) < self.update_interval {
            return;
        }
        self.last_update = Instant::now();
        for usage in &mut self.metrics.cpu_usage_per_core {
            let change = rand::random::<u64>() % 10;
            let direction = if rand::random::<bool>() { 1 } else { -1 };
            *usage = (*usage as i64 + change as i64 * direction).clamp(0, 100) as u64;
        }
        self.metrics.cpu_total_usage =
            self.metrics.cpu_usage_per_core.iter().sum::<u64>() / self.metrics.cpu_count as u64;
        let mem_change = rand::random::<u64>() % 50;
        let mem_direction = if rand::random::<bool>() { 1 } else { -1 };
        self.metrics.memory_used = (self.metrics.memory_used as i64
            + mem_change as i64 * mem_direction)
            .clamp(0, self.metrics.memory_total as i64) as u64;
        self.metrics.total_rx = (self.metrics.total_rx as i64 + rand::random::<i64>() % 200 - 100)
            .clamp(0, 5000) as u64;
        self.metrics.total_tx =
            (self.metrics.total_tx as i64 + rand::random::<i64>() % 100 - 50).clamp(0, 2500) as u64;
        self.metrics.cpu_history.remove(0);
        self.metrics.cpu_history.push(self.metrics.cpu_total_usage);
        self.metrics.memory_history.remove(0);
        let mem_percent =
            (self.metrics.memory_used as f64 / self.metrics.memory_total as f64 * 100.0) as u64;
        self.metrics.memory_history.push(mem_percent);
        self.metrics.net_rx_history.remove(0);
        self.metrics.net_rx_history.push(self.metrics.total_rx);
        self.metrics.net_tx_history.remove(0);
        self.metrics.net_tx_history.push(self.metrics.total_tx);
        for process in &mut self.metrics.processes {
            let cpu_change = rand::random::<f64>() % 5.0;
            let cpu_direction = if rand::random::<bool>() { 1.0 } else { -1.0 };
            process.cpu_usage = (process.cpu_usage + cpu_change * cpu_direction).clamp(0.0, 100.0);
            let mem_change = rand::random::<u64>() % 10;
            let mem_direction = if rand::random::<bool>() { 1 } else { -1 };
            process.memory_usage = (process.memory_usage as i64 + mem_change as i64 * mem_direction)
                .clamp(0, 2000) as u64;
        }
        self.sort_processes();
    }

    pub fn cycle_view(&mut self) {
        self.current_view = match self.current_view {
            View::System => View::Process,
            View::Process => View::Resources,
            View::Resources => View::Network,
            View::Network => View::Disks,
            View::Disks => View::Options,
            View::Options => View::System,
        };
        self.reset_selection();
    }

    pub fn reset_selection(&mut self) {
        self.selected_process = 0;
        self.process_scroll_offset = 0;
        self.show_proc_details = false;
    }

    pub fn scroll_down(&mut self) {
        match self.current_view {
            View::Process => {
                if self.selected_process < self.metrics.processes.len() - 1 {
                    self.selected_process += 1;
                    let visible_rows = self.max_processes;
                    if self.selected_process >= self.process_scroll_offset + visible_rows {
                        self.process_scroll_offset += 1;
                    }
                }
            }
            _ => {
                self.scroll_offset = self.scroll_offset.saturating_add(1);
            }
        }
    }

    pub fn scroll_up(&mut self) {
        match self.current_view {
            View::Process => {
                if self.selected_process > 0 {
                    self.selected_process -= 1;
                    if self.selected_process < self.process_scroll_offset {
                        self.process_scroll_offset = self.process_scroll_offset.saturating_sub(1);
                    }
                }
            }
            _ => {
                self.scroll_offset = self.scroll_offset.saturating_sub(1);
            }
        }
    }

    pub fn scroll_page_down(&mut self) {
        match self.current_view {
            View::Process => {
                let page_size = self.max_processes;
                self.selected_process =
                    (self.selected_process + page_size).min(self.metrics.processes.len() - 1);
                self.process_scroll_offset = (self.process_scroll_offset + page_size)
                    .min(self.metrics.processes.len().saturating_sub(page_size));
            }
            _ => {
                self.scroll_offset = self.scroll_offset.saturating_add(10);
            }
        }
    }

    pub fn scroll_page_up(&mut self) {
        match self.current_view {
            View::Process => {
                let page_size = self.max_processes;
                self.selected_process = self.selected_process.saturating_sub(page_size);
                self.process_scroll_offset = self.process_scroll_offset.saturating_sub(page_size);
            }
            _ => {
                self.scroll_offset = self.scroll_offset.saturating_sub(10);
            }
        }
    }

    pub fn scroll_top(&mut self) {
        match self.current_view {
            View::Process => {
                self.selected_process = 0;
                self.process_scroll_offset = 0;
            }
            _ => {
                self.scroll_offset = 0;
            }
        }
    }

    pub fn scroll_bottom(&mut self) {
        match self.current_view {
            View::Process => {
                self.selected_process = self.metrics.processes.len() - 1;
                let visible_rows = self.max_processes;
                self.process_scroll_offset =
                    self.metrics.processes.len().saturating_sub(visible_rows);
            }
            _ => {}
        }
    }

    pub fn toggle_pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn toggle_process_details(&mut self) {
        self.show_proc_details = !self.show_proc_details;
    }

    pub fn toggle_full_command(&mut self) {
        self.show_full_command = !self.show_full_command;
    }

    pub fn toggle_tree_view(&mut self) {
        self.show_tree_view = !self.show_tree_view;
    }

    pub fn toggle_proc_aggregation(&mut self) {
        self.proc_aggregated = !self.proc_aggregated;
    }

    pub fn increase_update_delay(&mut self) {
        self.update_interval = (self.update_interval * 2).min(Duration::from_secs(10));
    }

    pub fn decrease_update_delay(&mut self) {
        self.update_interval = (self.update_interval / 2).max(Duration::from_millis(250));
    }

    pub fn change_sort_column(&mut self, sort: ProcessSort) {
        if self.process_sort == sort {
            self.sort_reverse = !self.sort_reverse;
        } else {
            self.process_sort = sort;
            self.sort_reverse = matches!(sort, ProcessSort::Cpu | ProcessSort::Memory);
        }
        self.sort_processes();
        self.reset_selection();
    }

    fn sort_processes(&mut self) {
        match self.process_sort {
            ProcessSort::Pid => {
                self.metrics.processes.sort_by(|a, b| a.pid.cmp(&b.pid));
            }
            ProcessSort::Name => {
                self.metrics.processes.sort_by(|a, b| a.name.cmp(&b.name));
            }
            ProcessSort::Cpu => {
                self.metrics
                    .processes
                    .sort_by(|a, b| b.cpu_usage.partial_cmp(&a.cpu_usage).unwrap());
            }
            ProcessSort::Memory => {
                self.metrics
                    .processes
                    .sort_by(|a, b| b.memory_usage.cmp(&a.memory_usage));
            }
            ProcessSort::User => {
                self.metrics.processes.sort_by(|a, b| a.user.cmp(&b.user));
            }
            ProcessSort::Time => {
                self.metrics
                    .processes
                    .sort_by(|a, b| b.uptime.cmp(&a.uptime));
            }
            ProcessSort::Threads => {
                self.metrics
                    .processes
                    .sort_by(|a, b| b.threads.cmp(&a.threads));
            }
            ProcessSort::State => {
                self.metrics
                    .processes
                    .sort_by(|a, b| a.state.to_string().cmp(&b.state.to_string()));
            }
        }
        if !self.sort_reverse {
            self.metrics.processes.reverse();
        }
    }
}
