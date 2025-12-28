use std::time::Duration;

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];
    let mut value = bytes as f64;
    let mut unit_index = 0;
    while value >= 1024.0 && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }
    if value < 10.0 {
        format!("{:.2} {}", value, UNITS[unit_index])
    } else if value < 100.0 {
        format!("{:.1} {}", value, UNITS[unit_index])
    } else {
        format!("{:.0} {}", value, UNITS[unit_index])
    }
}

pub fn format_percentage(percentage: f64, warn_threshold: f64, crit_threshold: f64) -> String {
    if percentage >= crit_threshold {
        format!("{:.1}%", percentage)
    } else if percentage >= warn_threshold {
        format!("{:.1}%", percentage)
    } else {
        format!("{:.1}%", percentage)
    }
}

pub fn format_duration_long(duration: Duration) -> String {
    let secs = duration.as_secs();
    if secs == 0 {
        return "0s".to_string();
    }
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    let seconds = secs % 60;
    let mut parts = Vec::new();
    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if seconds > 0 || parts.is_empty() {
        parts.push(format!("{}s", seconds));
    }
    parts.join(" ")
}

pub fn create_progress_bar(percentage: u64, width: usize) -> String {
    let filled = (percentage as f64 * width as f64 / 100.0).round() as usize;
    let empty = width.saturating_sub(filled);
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}

pub fn truncate_with_ellipsis(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        ".".repeat(max_len)
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

pub fn align_text(text: &str, width: usize, alignment: Alignment) -> String {
    let text_len = text.chars().count();
    if text_len >= width {
        return truncate_with_ellipsis(text, width);
    }
    let padding = width - text_len;
    match alignment {
        Alignment::Left => format!("{}{}", text, " ".repeat(padding)),
        Alignment::Right => format!("{}{}", " ".repeat(padding), text),
        Alignment::Center => {
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            format!(
                "{}{}{}",
                " ".repeat(left_padding),
                text,
                " ".repeat(right_padding)
            )
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

pub fn safe_percentage(part: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        (part as f64 / total as f64) * 100.0
    }
}

pub fn color_gradient(start: (u8, u8, u8), end: (u8, u8, u8), steps: usize) -> Vec<(u8, u8, u8)> {
    let mut gradient = Vec::with_capacity(steps);
    for i in 0..steps {
        let t = i as f32 / (steps - 1) as f32;
        let r = lerp(start.0 as f32, end.0 as f32, t).round() as u8;
        let g = lerp(start.1 as f32, end.1 as f32, t).round() as u8;
        let b = lerp(start.2 as f32, end.2 as f32, t).round() as u8;
        gradient.push((r, g, b));
    }
    gradient
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub mod simulator {
    use rand::Rng;
    use std::time::{Duration, Instant};

    pub struct DataSimulator {
        last_update: Instant,
        base_values: Vec<f64>,
        trends: Vec<f64>,
    }

    impl DataSimulator {
        pub fn new(count: usize) -> Self {
            let mut rng = rand::thread_rng();
            Self {
                last_update: Instant::now(),
                base_values: (0..count).map(|_| rng.gen_range(0.0..100.0)).collect(),
                trends: (0..count).map(|_| rng.gen_range(-1.0..1.0)).collect(),
            }
        }

        pub fn update(&mut self) -> Vec<f64> {
            let now = Instant::now();
            let delta = now.duration_since(self.last_update).as_secs_f64();
            self.last_update = now;
            let mut rng = rand::thread_rng();
            for i in 0..self.base_values.len() {
                self.base_values[i] += self.trends[i] * delta * 10.0;
                self.base_values[i] += rng.gen_range(-5.0..5.0);
                self.base_values[i] = self.base_values[i].clamp(0.0, 100.0);
                if rng.r#gen::<f64>() < 0.1 {
                    self.trends[i] = rng.gen_range(-1.0..1.0);
                }
            }
            self.base_values.clone()
        }
    }

    pub fn simulate_network_data() -> (u64, u64) {
        let mut rng = rand::thread_rng();
        let rx = rng.gen_range(100..2000);
        let tx = rng.gen_range(50..1000);
        (rx, tx)
    }

    pub fn simulate_disk_io() -> (u64, u64) {
        let mut rng = rand::thread_rng();
        let read = rng.gen_range(10..200);
        let write = rng.gen_range(5..100);
        (read, write)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(0), "0.00 B");
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(1073741824), "1.00 GB");
    }

    #[test]
    fn test_create_progress_bar() {
        assert_eq!(create_progress_bar(0, 10), "[░░░░░░░░░░]");
        assert_eq!(create_progress_bar(50, 10), "[█████░░░░░]");
        assert_eq!(create_progress_bar(100, 10), "[██████████]");
    }

    #[test]
    fn test_truncate_with_ellipsis() {
        assert_eq!(truncate_with_ellipsis("Hello World", 5), "He...");
        assert_eq!(truncate_with_ellipsis("Hello", 10), "Hello");
        assert_eq!(truncate_with_ellipsis("Hello", 3), "...");
    }

    #[test]
    fn test_align_text() {
        assert_eq!(align_text("Test", 10, Alignment::Left), "Test      ");
        assert_eq!(align_text("Test", 10, Alignment::Right), "      Test");
        assert_eq!(align_text("Test", 10, Alignment::Center), "   Test   ");
    }
}
