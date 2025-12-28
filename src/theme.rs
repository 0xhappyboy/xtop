use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    // Background colors
    pub bg_dark: Color,
    pub bg_normal: Color,
    pub bg_light: Color,
    pub bg_lighter: Color,
    // Borders
    pub border: Color,
    pub border_light: Color,
    // Text
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_dim: Color,
    pub text_bright: Color,
    // Status colors
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
    // Special
    pub cpu_colors: [Color; 8],
    pub mem_colors: [Color; 3],
    pub net_colors: [Color; 2],
    pub disk_colors: [Color; 4],
    // Chart
    pub chart_gradient: [Color; 5],
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            // btop-style dark theme
            bg_dark: Color::Rgb(24, 24, 37),           // #181825
            bg_normal: Color::Rgb(30, 31, 47),         // #1e1f2f
            bg_light: Color::Rgb(38, 39, 58),          // #26273a
            bg_lighter: Color::Rgb(49, 50, 73),        // #313249
            border: Color::Rgb(54, 56, 82),            // #363852
            border_light: Color::Rgb(73, 75, 105),     // #494b69
            text_primary: Color::Rgb(205, 214, 244),   // #cdd6f4
            text_secondary: Color::Rgb(166, 173, 200), // #a6adc8
            text_dim: Color::Rgb(127, 132, 156),       // #7f849c
            text_bright: Color::Rgb(230, 235, 255),    // #e6ebff
            success: Color::Rgb(166, 227, 161),        // #a6e3a1
            warning: Color::Rgb(249, 226, 175),        // #f9dfaf
            danger: Color::Rgb(243, 139, 168),         // #f38ba8
            info: Color::Rgb(137, 180, 250),           // #89b4fa
            // CPU core colors
            cpu_colors: [
                Color::Rgb(137, 180, 250), // Blue
                Color::Rgb(245, 194, 231), // Pink
                Color::Rgb(166, 227, 161), // Green
                Color::Rgb(250, 179, 135), // Orange
                Color::Rgb(243, 139, 168), // Red
                Color::Rgb(203, 166, 247), // Purple
                Color::Rgb(249, 226, 175), // Yellow
                Color::Rgb(148, 226, 213), // Cyan
            ],
            // Memory gradient
            mem_colors: [
                Color::Rgb(166, 227, 161), // Green (0-70%)
                Color::Rgb(249, 226, 175), // Yellow (70-90%)
                Color::Rgb(243, 139, 168), // Red (90-100%)
            ],
            // Network colors
            net_colors: [
                Color::Rgb(166, 227, 161), // Download (green)
                Color::Rgb(137, 180, 250), // Upload (blue)
            ],
            // Disk colors
            disk_colors: [
                Color::Rgb(137, 180, 250), // Read
                Color::Rgb(243, 139, 168), // Write
                Color::Rgb(166, 227, 161), // Usage
                Color::Rgb(249, 226, 175), // Available
            ],
            // Chart gradient
            chart_gradient: [
                Color::Rgb(30, 102, 245),  // Dark blue
                Color::Rgb(0, 184, 217),   // Cyan
                Color::Rgb(0, 223, 162),   // Green cyan
                Color::Rgb(255, 184, 108), // Orange
                Color::Rgb(255, 119, 119), // Red
            ],
        }
    }
}

impl Theme {
    pub fn get_cpu_color(&self, index: usize) -> Color {
        self.cpu_colors[index % self.cpu_colors.len()]
    }

    pub fn get_mem_color(&self, percentage: u64) -> Color {
        match percentage {
            0..=70 => self.mem_colors[0],
            71..=90 => self.mem_colors[1],
            _ => self.mem_colors[2],
        }
    }

    pub fn get_usage_color(&self, percentage: u64) -> Color {
        match percentage {
            0..=70 => self.success,
            71..=85 => self.warning,
            _ => self.danger,
        }
    }
}
