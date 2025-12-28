use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Style, Stylize},
    text::Span,
    widgets::{Paragraph, Widget},
};

use crate::{
    app::{App, View},
    components,
    sys_info::DiskInfo,
    theme::Theme,
};

pub fn ui(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.size();
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(size);
    let content_area = main_layout[0];
    let footer_area = main_layout[1];
    let content_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(content_area);
    let top_area = content_layout[0];
    render_top_area(f, top_area, app, &theme);
    let bottom_area = content_layout[1];
    render_bottom_area(f, bottom_area, app, &theme);
    let footer = components::render_footer(
        footer_area,
        &theme,
        &view_to_str(app.current_view),
        app.show_help,
    );
    f.render_widget(footer, footer_area);
    if app.show_help {
        render_help_overlay(f, size, &theme);
    }
}

fn render_top_area(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let top_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);
    render_cpu_chart(f, top_layout[0], app, theme);
    render_cpu_info(f, top_layout[1], app, theme);
}

fn render_cpu_chart(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let cpu_block = ratatui::widgets::Block::default()
        .title(Span::styled(
            " CPU Usage History ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let cpu_area = cpu_block.inner(area);
    let cpu_data: Vec<(f64, f64)> = app
        .metrics
        .cpu_history
        .iter()
        .enumerate()
        .map(|(i, &usage)| (i as f64, usage as f64))
        .collect();
    let cpu_data: &'static [(f64, f64)] = Box::leak(cpu_data.into_boxed_slice());
    let cpu_chart = ratatui::widgets::Chart::new(vec![
        ratatui::widgets::Dataset::default()
            .name("CPU Usage")
            .marker(ratatui::symbols::Marker::Braille)
            .graph_type(ratatui::widgets::GraphType::Line)
            .style(Style::default().fg(theme.cpu_colors[0]))
            .data(cpu_data),
    ])
    .x_axis(
        ratatui::widgets::Axis::default()
            .style(Style::default().fg(theme.text_dim))
            .bounds([0.0, cpu_data.len() as f64 - 1.0])
            .labels(vec![
                Span::styled("-60s", Style::default().fg(theme.text_dim)),
                Span::styled("now", Style::default().fg(theme.text_dim)),
            ]),
    )
    .y_axis(
        ratatui::widgets::Axis::default()
            .style(Style::default().fg(theme.text_dim))
            .bounds([0.0, 100.0])
            .labels(vec![
                Span::styled("0%", Style::default().fg(theme.text_dim)),
                Span::styled("50%", Style::default().fg(theme.text_dim)),
                Span::styled("100%", Style::default().fg(theme.text_dim)),
            ]),
    );
    f.render_widget(cpu_block, area);
    f.render_widget(cpu_chart, cpu_area);
}

fn render_cpu_info(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let info_block = ratatui::widgets::Block::default()
        .title(Span::styled(
            " CPU Info ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let info_area = info_block.inner(area);
    let temp_color = if app.metrics.cpu_temperature > 80.0 {
        theme.danger
    } else if app.metrics.cpu_temperature > 70.0 {
        theme.warning
    } else {
        theme.success
    };
    let cpu_usage_color = theme.get_usage_color(app.metrics.cpu_total_usage);
    let temp_bar = create_thermal_bar(app.metrics.cpu_temperature, theme);
    let usage_bar = create_usage_bar(app.metrics.cpu_total_usage, theme);
    let info_text = vec![
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Model: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                &app.metrics.cpu_model,
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Cores: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{}", app.metrics.cpu_count),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Freq: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{} MHz", app.metrics.cpu_frequency),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Temp: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{:.1}°C", app.metrics.cpu_temperature),
                Style::default()
                    .fg(temp_color)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled(temp_bar, Style::default().fg(temp_color)),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Usage: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{}%", app.metrics.cpu_total_usage),
                Style::default()
                    .fg(cpu_usage_color)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled(usage_bar, Style::default().fg(cpu_usage_color)),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Load: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{:.2}", app.metrics.load_average.one),
                Style::default().fg(
                    if app.metrics.load_average.one > (app.metrics.cpu_count as f32).into() {
                        theme.danger
                    } else {
                        theme.success
                    },
                ),
            ),
        ]),
    ];
    let info_para = Paragraph::new(info_text).block(ratatui::widgets::Block::default());
    f.render_widget(info_block, area);
    f.render_widget(info_para, info_area);
}

fn render_bottom_area(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let bottom_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    render_memory_disk_info(f, bottom_layout[0], app, theme);
    render_process_table(f, bottom_layout[1], app, theme);
}

fn render_memory_disk_info(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let left_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    render_memory_info(f, left_layout[0], app, theme);
    render_disk_info(f, left_layout[1], app, theme);
}

fn render_memory_info(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let mem_block = ratatui::widgets::Block::default()
        .title(Span::styled(
            " Memory Usage ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let mem_area = mem_block.inner(area);
    let mem_percent =
        (app.metrics.memory_used as f64 / app.metrics.memory_total as f64 * 100.0) as u64;
    let mem_color = theme.get_mem_color(mem_percent);
    let mem_bar_width: usize = 20;
    let mem_filled = (mem_percent as f64 * mem_bar_width as f64 / 100.0).round() as usize;
    let mem_bar = format!(
        "[{}{}]",
        "█".repeat(mem_filled),
        "░".repeat(mem_bar_width.saturating_sub(mem_filled))
    );
    let swap_percent = if app.metrics.swap_total > 0 {
        (app.metrics.swap_used as f64 / app.metrics.swap_total as f64 * 100.0) as u64
    } else {
        0
    };
    let mem_text = vec![
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Total: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{:.1} GB", app.metrics.memory_total as f64 / 1024.0),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Used: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{:.1} GB", app.metrics.memory_used as f64 / 1024.0),
                Style::default()
                    .fg(mem_color)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
            ratatui::text::Span::raw(" "),
            ratatui::text::Span::styled(
                format!("({}%)", mem_percent),
                Style::default().fg(mem_color),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled(mem_bar, Style::default().fg(mem_color)),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Available: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{:.1} GB", app.metrics.memory_available as f64 / 1024.0),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Swap: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!(
                    "{}/{} GB",
                    app.metrics.swap_used / 1024,
                    app.metrics.swap_total / 1024
                ),
                Style::default().fg(if swap_percent > 50 {
                    theme.danger
                } else {
                    theme.text_primary
                }),
            ),
            ratatui::text::Span::raw(" "),
            ratatui::text::Span::styled(
                format!("({}%)", swap_percent),
                Style::default().fg(if swap_percent > 50 {
                    theme.danger
                } else {
                    theme.warning
                }),
            ),
        ]),
    ];
    let mem_para = Paragraph::new(mem_text).block(ratatui::widgets::Block::default());
    f.render_widget(mem_block, area);
    f.render_widget(mem_para, mem_area);
}

fn render_disk_info(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let disk_block = ratatui::widgets::Block::default()
        .title(Span::styled(
            " Disk Usage ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let disk_area = disk_block.inner(area);
    let binding_disk_info = DiskInfo::default();
    let disk = app.metrics.disks.first().unwrap_or(&binding_disk_info);
    let disk_color = theme.get_usage_color(disk.usage);
    let disk_bar_width: usize = 20;
    let disk_filled = (disk.usage as f64 * disk_bar_width as f64 / 100.0).round() as usize;
    let disk_bar = format!(
        "[{}{}]",
        "█".repeat(disk_filled),
        "░".repeat(disk_bar_width.saturating_sub(disk_filled))
    );

    let disk_text = vec![
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Disk: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(&disk.name, Style::default().fg(theme.text_primary)),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Mount: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                &disk.mount_point,
                Style::default().fg(theme.text_secondary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Total: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{} GB", disk.total),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Used: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{} GB", disk.used),
                Style::default()
                    .fg(disk_color)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
            ratatui::text::Span::raw(" "),
            ratatui::text::Span::styled(
                format!("({}%)", disk.usage),
                Style::default().fg(disk_color),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::raw("  "),
            ratatui::text::Span::styled(disk_bar, Style::default().fg(disk_color)),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("Free: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{} GB", disk.free),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        ratatui::text::Line::from(vec![
            ratatui::text::Span::styled("I/O R/W: ", Style::default().fg(theme.text_dim)),
            ratatui::text::Span::styled(
                format!("{}/{} MB/s", disk.read_speed, disk.write_speed),
                Style::default()
                    .fg(theme.disk_colors[0])
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ),
        ]),
    ];
    let disk_para = Paragraph::new(disk_text).block(ratatui::widgets::Block::default());
    f.render_widget(disk_block, area);
    f.render_widget(disk_para, disk_area);
}

fn render_process_table(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    let table_block = ratatui::widgets::Block::default()
        .title(Span::styled(
            " Processes ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ))
        .borders(ratatui::widgets::Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let table_area = table_block.inner(area);
    let visible_rows = (table_area.height as usize).saturating_sub(1);
    let header = ratatui::widgets::Row::new(vec![
        ratatui::widgets::Cell::from("PID").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        ratatui::widgets::Cell::from("Name").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        ratatui::widgets::Cell::from("CPU%").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
        ratatui::widgets::Cell::from("MEM").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(ratatui::style::Modifier::BOLD),
        ),
    ]);
    let start_idx = app.process_scroll_offset;
    let end_idx = (start_idx + visible_rows).min(app.metrics.processes.len());
    let rows: Vec<ratatui::widgets::Row> = app.metrics.processes[start_idx..end_idx]
        .iter()
        .enumerate()
        .map(|(i, process)| {
            let global_idx = start_idx + i;
            let is_selected = global_idx == app.selected_process;
            let cpu_color = if process.cpu_usage > 50.0 {
                theme.danger
            } else if process.cpu_usage > 25.0 {
                theme.warning
            } else {
                theme.success
            };
            let mem_color = if process.memory_percent > 10.0 {
                theme.danger
            } else if process.memory_percent > 5.0 {
                theme.warning
            } else {
                theme.info
            };
            let bg_color = if is_selected {
                theme.bg_lighter
            } else if global_idx % 2 == 0 {
                theme.bg_normal
            } else {
                theme.bg_light
            };
            ratatui::widgets::Row::new(vec![
                ratatui::widgets::Cell::from(process.pid.to_string())
                    .style(Style::default().fg(theme.text_primary)),
                ratatui::widgets::Cell::from(
                    if app.show_full_command && !process.full_command.is_empty() {
                        process.full_command.clone()
                    } else {
                        process.name.clone()
                    },
                )
                .style(Style::default().fg(theme.text_primary)),
                ratatui::widgets::Cell::from(format!("{:.1}", process.cpu_usage)).style(
                    Style::default()
                        .fg(cpu_color)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                ratatui::widgets::Cell::from(format!("{} MB", process.memory_usage)).style(
                    Style::default()
                        .fg(mem_color)
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ])
            .style(Style::default().bg(bg_color))
        })
        .collect();
    let table = ratatui::widgets::Table::new(
        rows,
        vec![
            Constraint::Length(8),
            Constraint::Percentage(50),
            Constraint::Length(8),
            Constraint::Length(10),
        ],
    )
    .header(header)
    .block(ratatui::widgets::Block::default());
    f.render_widget(table_block, area);
    f.render_widget(table, table_area);
}

fn render_help_overlay(f: &mut Frame, area: Rect, theme: &Theme) {
    let overlay = Paragraph::new("").style(Style::default().bg(theme.bg_dark).fg(theme.text_dim));
    f.render_widget(overlay, area);
    let help_width = (area.width as f32 * 0.8) as u16;
    let help_height = (area.height as f32 * 0.8) as u16;
    let help_x = (area.width - help_width) / 2;
    let help_y = (area.height - help_height) / 2;
    let help_area = Rect::new(help_x, help_y, help_width, help_height);
    let help_widget = components::render_help_view(help_area, theme);
    help_widget(f);
}

fn create_thermal_bar(temp: f32, theme: &Theme) -> String {
    let bar_width = 10;
    let normalized_temp = (temp / 100.0).min(1.0);
    let filled = (normalized_temp * bar_width as f32).round() as usize;
    let chars = vec!["░", "▒", "▓", "█"];
    let mut bar = String::new();
    for i in 0..bar_width {
        if i < filled {
            let char_idx = (i * chars.len() / bar_width).min(chars.len() - 1);
            bar.push_str(chars[char_idx]);
        } else {
            bar.push_str("░");
        }
    }
    format!("[{}]", bar)
}

fn create_usage_bar(usage: u64, theme: &Theme) -> String {
    let bar_width: usize = 10;
    let filled = (usage as f64 * bar_width as f64 / 100.0).round() as usize;
    format!(
        "[{}{}]",
        "█".repeat(filled),
        "░".repeat(bar_width.saturating_sub(filled))
    )
}

fn view_to_str(view: View) -> &'static str {
    match view {
        View::System => "System",
        View::Process => "Process",
        View::Resources => "Resources",
        View::Network => "Network",
        View::Disks => "Disks",
        View::Options => "Options",
    }
}
