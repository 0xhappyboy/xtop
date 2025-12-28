use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct SystemInfo {
    // System Information
    pub hostname: String,
    pub kernel_version: String,
    pub os_name: String,
    pub uptime: Duration,
    // CPU Information
    pub cpu_count: usize,
    pub cpu_usage_per_core: Vec<u64>,
    pub cpu_total_usage: u64,
    pub cpu_frequency: u64, // MHz
    pub cpu_temperature: f32,
    pub cpu_model: String,
    // Memory Information
    pub memory_total: u64,     // MB
    pub memory_used: u64,      // MB
    pub memory_free: u64,      // MB
    pub memory_available: u64, // MB
    pub memory_cached: u64,    // MB
    pub memory_buffers: u64,   // MB
    pub swap_total: u64,       // MB
    pub swap_used: u64,        // MB
    pub swap_free: u64,        // MB
    // Disk Information
    pub disks: Vec<DiskInfo>,
    // Network Information
    pub network_interfaces: Vec<NetworkInterface>,
    pub total_rx: u64, // KB/s
    pub total_tx: u64, // KB/s
    // Process Information
    pub processes: Vec<ProcessInfo>,
    pub process_count: usize,
    pub thread_count: usize,
    // Historical Data
    pub cpu_history: Vec<u64>,
    pub memory_history: Vec<u64>,
    pub net_rx_history: Vec<u64>,
    pub net_tx_history: Vec<u64>,
    // Load
    pub load_average: LoadAverage,
    // Update Timestamp
    pub last_update: Instant,
}

#[derive(Debug, Clone)]
pub struct DiskInfo {
    pub name: String,
    pub mount_point: String,
    pub total: u64,       // GB
    pub used: u64,        // GB
    pub free: u64,        // GB
    pub usage: u64,       // Percentage
    pub read_speed: u64,  // MB/s
    pub write_speed: u64, // MB/s
    pub device_type: String,
}

impl Default for DiskInfo {
    fn default() -> Self {
        Self {
            name: "N/A".to_string(),
            mount_point: "/".to_string(),
            total: 0,
            used: 0,
            free: 0,
            usage: 0,
            read_speed: 0,
            write_speed: 0,
            device_type: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_speed: u64, // KB/s
    pub tx_speed: u64, // KB/s
    pub ip_address: String,
    pub mac_address: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    pub ppid: u32,
    pub name: String,
    pub command: String,
    pub full_command: String,
    pub user: String,
    pub cpu_usage: f64,      // Percentage
    pub memory_usage: u64,   // MB
    pub memory_percent: f64, // Percentage
    pub state: ProcessState,
    pub priority: i32,
    pub nice: i32,
    pub threads: u32,
    pub start_time: String,
    pub uptime: Duration,
    pub read_speed: u64,  // KB/s
    pub write_speed: u64, // KB/s
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessState {
    Running,
    Sleeping,
    Waiting,
    Zombie,
    Stopped,
    Tracing,
    Dead,
    Wakekill,
    Waking,
    Parked,
    Idle,
}

impl std::fmt::Display for ProcessState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessState::Running => write!(f, "R"),
            ProcessState::Sleeping => write!(f, "S"),
            ProcessState::Waiting => write!(f, "D"),
            ProcessState::Zombie => write!(f, "Z"),
            ProcessState::Stopped => write!(f, "T"),
            ProcessState::Tracing => write!(f, "t"),
            ProcessState::Dead => write!(f, "X"),
            ProcessState::Wakekill => write!(f, "K"),
            ProcessState::Waking => write!(f, "W"),
            ProcessState::Parked => write!(f, "P"),
            ProcessState::Idle => write!(f, "I"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProcessSort {
    Pid,
    Name,
    Cpu,
    Memory,
    User,
    Time,
    Threads,
    State,
}

impl Default for SystemInfo {
    fn default() -> Self {
        let now = Instant::now();
        let cpu_count = 8;
        let mut cpu_usage_per_core = Vec::with_capacity(cpu_count);
        for i in 0..cpu_count {
            cpu_usage_per_core.push((20 + i as u64 * 5).min(100));
        }
        let memory_total = 16384; // 16GB
        let memory_used = 8192; // 8GB
        let memory_available = memory_total - memory_used;
        Self {
            hostname: "localhost".to_string(),
            kernel_version: "5.15.0".to_string(),
            os_name: "Linux".to_string(),
            uptime: Duration::from_secs(86400 + 3600), // 1 day 1 hour
            cpu_count,
            cpu_usage_per_core,
            cpu_total_usage: 45,
            cpu_frequency: 3600,
            cpu_temperature: 65.5,
            cpu_model: "Intel Core i7-12700K".to_string(),
            memory_total,
            memory_used,
            memory_free: memory_available / 2,
            memory_available,
            memory_cached: 2048,
            memory_buffers: 512,
            swap_total: 8192,
            swap_used: 1024,
            swap_free: 8192 - 1024,
            disks: vec![
                DiskInfo {
                    name: "nvme0n1".to_string(),
                    mount_point: "/".to_string(),
                    total: 512,
                    used: 256,
                    free: 256,
                    usage: 50,
                    read_speed: 120,
                    write_speed: 45,
                    device_type: "NVMe".to_string(),
                },
                DiskInfo {
                    name: "sda".to_string(),
                    mount_point: "/home".to_string(),
                    total: 1024,
                    used: 512,
                    free: 512,
                    usage: 50,
                    read_speed: 45,
                    write_speed: 23,
                    device_type: "SSD".to_string(),
                },
            ],

            network_interfaces: vec![NetworkInterface {
                name: "eth0".to_string(),
                rx_bytes: 1024 * 1024 * 1024,
                tx_bytes: 512 * 1024 * 1024,
                rx_speed: 1200,
                tx_speed: 450,
                ip_address: "192.168.1.100".to_string(),
                mac_address: "00:11:22:33:44:55".to_string(),
                status: "up".to_string(),
            }],
            total_rx: 1200,
            total_tx: 450,
            processes: generate_sample_processes(),
            process_count: 150,
            thread_count: 1200,
            cpu_history: vec![45, 50, 55, 60, 65, 70, 65, 60, 55, 50, 45, 40],
            memory_history: vec![50, 52, 54, 56, 58, 60, 62, 64, 66, 68, 70, 72],
            net_rx_history: vec![800, 850, 900, 950, 1000, 1050, 1100, 1150, 1200],
            net_tx_history: vec![300, 325, 350, 375, 400, 425, 450, 475, 500],
            load_average: LoadAverage {
                one: 1.25,
                five: 1.85,
                fifteen: 2.15,
            },
            last_update: now,
        }
    }
}

fn generate_sample_processes() -> Vec<ProcessInfo> {
    let mut processes = Vec::new();
    let sample_processes = vec![
        ("systemd", 1, 0, "root", 1.5, 200, ProcessState::Sleeping),
        (
            "NetworkManager",
            1234,
            1,
            "root",
            2.3,
            45,
            ProcessState::Sleeping,
        ),
        (
            "gnome-shell",
            2345,
            1,
            "user",
            12.5,
            356,
            ProcessState::Running,
        ),
        (
            "firefox",
            3456,
            2345,
            "user",
            24.8,
            1240,
            ProcessState::Running,
        ),
        ("code", 4567, 2345, "user", 18.2, 890, ProcessState::Running),
        ("docker", 5678, 1, "root", 3.5, 345, ProcessState::Sleeping),
        (
            "postgres",
            6789,
            1,
            "postgres",
            7.8,
            456,
            ProcessState::Sleeping,
        ),
        (
            "nginx",
            7890,
            1,
            "www-data",
            1.2,
            89,
            ProcessState::Sleeping,
        ),
        ("redis", 8901, 1, "redis", 2.5, 123, ProcessState::Sleeping),
        (
            "python3",
            9012,
            3456,
            "user",
            15.3,
            234,
            ProcessState::Running,
        ),
    ];
    for (i, (name, pid, ppid, user, cpu, memory, state)) in sample_processes.iter().enumerate() {
        processes.push(ProcessInfo {
            pid: *pid,
            ppid: *ppid,
            name: name.to_string(),
            command: format!("/usr/bin/{}", name.to_lowercase()),
            full_command: format!("/usr/bin/{} --some-flag", name.to_lowercase()),
            user: user.to_string(),
            cpu_usage: *cpu,
            memory_usage: *memory,
            memory_percent: (*memory as f64 / 16384.0) * 100.0,
            state: *state,
            priority: 20,
            nice: 0,
            threads: (i as u32 + 1) * 2,
            start_time: "10:30:15".to_string(),
            uptime: Duration::from_secs(3600 * i as u64),
            read_speed: (i as u64 * 10) % 100,
            write_speed: (i as u64 * 5) % 50,
        });
    }
    processes
}
