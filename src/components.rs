use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    symbols,
    text::{Line, Span},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, GraphType, Paragraph, Row,
        Table, Widget,
    },
};

use crate::{sys_info::SystemInfo, theme::Theme};

pub fn render_header<'a>(area: Rect, theme: &'a Theme, metrics: &'a SystemInfo) -> Paragraph<'a> {
    let uptime = format_duration(metrics.uptime);
    let time = chrono::Local::now().format("%H:%M:%S").to_string();
    let header_text = format!(
        " {}@{} | {} | Up: {} | Load: {:.2} {:.2} {:.2} | Processes: {} | Threads: {} ",
        whoami::username(),
        metrics.hostname,
        time,
        uptime,
        metrics.load_average.one,
        metrics.load_average.five,
        metrics.load_average.fifteen,
        metrics.process_count,
        metrics.thread_count,
    );
    Paragraph::new(header_text)
        .style(Style::default().fg(theme.text_bright).bg(theme.bg_dark))
        .alignment(ratatui::layout::Alignment::Center)
}

pub fn render_footer<'a>(
    area: Rect,
    theme: &'a Theme,
    current_view: &'a str,
    show_help: bool,
) -> Paragraph<'a> {
    let footer_text = if show_help {
        "[q]uit [1-6]views [↑↓]scroll [←→]sort [F1]help [F5]tree [F6]aggregate [space]pause [r]eset"
    } else {
        match current_view {
            "System" => "[F1]Help [1]System [2]Process [3]Resources [4]Network [5]Disks [6]Options",
            "Process" => {
                "[F1]Help [↑↓]Select [Enter]Details [c]CPU [m]Memory [p]PID [n]Name [f]FullCmd"
            }
            _ => "[F1]Help [Tab]NextView [q]Quit [space]Pause [+-]Speed",
        }
    };
    Paragraph::new(footer_text)
        .style(Style::default().fg(theme.text_dim).bg(theme.bg_dark))
        .alignment(ratatui::layout::Alignment::Center)
}

pub fn render_system_view<'a>(
    area: Rect,
    theme: &'a Theme,
    metrics: &'a SystemInfo,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),
            Constraint::Length(12),
            Constraint::Min(8),
        ])
        .split(area);
    let cpu_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(layout[0]);
    let cpu_block = Block::default()
        .title(Span::styled(
            " CPU Usage ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let cpu_area = cpu_block.inner(cpu_layout[0]);
    let cpu_info = vec![
        Line::from(vec![
            Span::styled("Model: ", Style::default().fg(theme.text_dim)),
            Span::styled(&metrics.cpu_model, Style::default().fg(theme.text_primary)),
        ]),
        Line::from(vec![
            Span::styled("Cores: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{}", metrics.cpu_count),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Frequency: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} MHz", metrics.cpu_frequency),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Temperature: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.1}°C", metrics.cpu_temperature),
                Style::default().fg(if metrics.cpu_temperature > 80.0 {
                    theme.danger
                } else if metrics.cpu_temperature > 70.0 {
                    theme.warning
                } else {
                    theme.success
                }),
            ),
        ]),
        Line::from(vec![
            Span::styled("Total Usage: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{}%", metrics.cpu_total_usage),
                Style::default()
                    .fg(theme.get_usage_color(metrics.cpu_total_usage))
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let cpu_info_block = Block::default()
        .title(Span::styled(
            " CPU Info ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let cpu_info_para = Paragraph::new(cpu_info).block(Block::default());
    let mem_block = Block::default()
        .title(Span::styled(
            " Memory Usage ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let mem_area = mem_block.inner(layout[1]);
    let mem_percent = (metrics.memory_used as f64 / metrics.memory_total as f64 * 100.0) as u64;
    let swap_percent = if metrics.swap_total > 0 {
        (metrics.swap_used as f64 / metrics.swap_total as f64 * 100.0) as u64
    } else {
        0
    };
    let mem_info = vec![
        Line::from(vec![
            Span::styled("Total: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.1} GB", metrics.memory_total as f64 / 1024.0),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Used: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.1} GB", metrics.memory_used as f64 / 1024.0),
                Style::default()
                    .fg(theme.get_mem_color(mem_percent))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" "),
            Span::styled(
                format!("({}%)", mem_percent),
                Style::default().fg(theme.get_mem_color(mem_percent)),
            ),
        ]),
        Line::from(vec![
            Span::styled("Available: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.1} GB", metrics.memory_available as f64 / 1024.0),
                Style::default().fg(theme.text_primary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Cached: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.1} GB", metrics.memory_cached as f64 / 1024.0),
                Style::default().fg(theme.text_secondary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Swap: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!(
                    "{}/{} GB",
                    metrics.swap_used / 1024,
                    metrics.swap_total / 1024
                ),
                Style::default().fg(if swap_percent > 50 {
                    theme.danger
                } else {
                    theme.text_primary
                }),
            ),
            Span::raw(" "),
            Span::styled(
                format!("({}%)", swap_percent),
                Style::default().fg(if swap_percent > 50 {
                    theme.danger
                } else {
                    theme.warning
                }),
            ),
        ]),
    ];
    let mem_gauge = Gauge::default()
        .block(Block::default())
        .gauge_style(Style::default().fg(theme.get_mem_color(mem_percent)))
        .percent(mem_percent as u16)
        .label(format!("{}%", mem_percent));
    let mem_info_para = Paragraph::new(mem_info).block(Block::default());
    let sys_block = Block::default()
        .title(Span::styled(
            " System Info ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let sys_area = sys_block.inner(layout[2]);
    let sys_info = vec![
        Line::from(vec![
            Span::styled("OS: ", Style::default().fg(theme.text_dim)),
            Span::styled(&metrics.os_name, Style::default().fg(theme.text_primary)),
        ]),
        Line::from(vec![
            Span::styled("Kernel: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                &metrics.kernel_version,
                Style::default().fg(theme.text_primary),
            ),
        ]),
        Line::from(vec![
            Span::styled("Hostname: ", Style::default().fg(theme.text_dim)),
            Span::styled(&metrics.hostname, Style::default().fg(theme.text_primary)),
        ]),
    ];
    let sys_info_para = Paragraph::new(sys_info).block(Block::default());
    let cpu_usage_data = metrics.cpu_usage_per_core.clone();
    Box::new(move |f: &mut ratatui::Frame| {
        let cpu_data: Vec<(&'static str, u64)> = cpu_usage_data
            .iter()
            .enumerate()
            .map(|(i, &usage)| {
                let label = if i < 10 {
                    format!("C{}", i)
                } else {
                    format!("{}", i)
                };
                let leaked_str: &'static str = Box::leak(label.into_boxed_str());
                (leaked_str, usage)
            })
            .collect();
        let cpu_chart = BarChart::default()
            .block(Block::default())
            .bar_width(3)
            .bar_gap(1)
            .bar_style(Style::default().fg(theme.cpu_colors[0]))
            .value_style(Style::default().fg(theme.text_secondary))
            .label_style(Style::default().fg(theme.text_dim))
            .data(&cpu_data);
        let cpu_info_block_clone = cpu_info_block.clone();
        f.render_widget(cpu_block, cpu_layout[0]);
        f.render_widget(cpu_chart, cpu_area);
        f.render_widget(cpu_info_block, cpu_layout[1]);
        f.render_widget(cpu_info_para, cpu_info_block_clone.inner(cpu_layout[1]));
        f.render_widget(mem_block, layout[1]);
        let mem_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(mem_area);
        f.render_widget(mem_info_para, mem_layout[0]);
        f.render_widget(mem_gauge, mem_layout[1]);
        f.render_widget(sys_block, layout[2]);
        f.render_widget(sys_info_para, sys_area);
    })
}

pub fn render_process_view<'a>(
    area: Rect,
    theme: &'a Theme,
    metrics: &'a SystemInfo,
    selected_process: usize,
    scroll_offset: usize,
    max_rows: usize,
    show_full_command: bool,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(8),
        ])
        .split(area);
    let header = Row::new(vec![
        Cell::from("PID").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("Name").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("CPU%").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("MEM").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("User").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("State").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
        Cell::from("Threads").style(
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ),
    ]);
    let start_idx = scroll_offset;
    let end_idx = (scroll_offset + max_rows).min(metrics.processes.len());
    let rows: Vec<Row> = metrics.processes[start_idx..end_idx]
        .iter()
        .enumerate()
        .map(|(i, process)| {
            let global_idx = start_idx + i;
            let is_selected = global_idx == selected_process;

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
            let state_color = match process.state {
                crate::sys_info::ProcessState::Running => theme.success,
                crate::sys_info::ProcessState::Sleeping => theme.info,
                crate::sys_info::ProcessState::Zombie => theme.danger,
                _ => theme.warning,
            };
            let bg_color = if is_selected {
                theme.bg_lighter
            } else if global_idx % 2 == 0 {
                theme.bg_normal
            } else {
                theme.bg_light
            };
            Row::new(vec![
                Cell::from(process.pid.to_string()).style(Style::default().fg(theme.text_primary)),
                Cell::from(if show_full_command && !process.full_command.is_empty() {
                    process.full_command.clone()
                } else {
                    process.name.clone()
                })
                .style(Style::default().fg(theme.text_primary)),
                Cell::from(format!("{:.1}", process.cpu_usage))
                    .style(Style::default().fg(cpu_color).add_modifier(Modifier::BOLD)),
                Cell::from(format!("{} MB", process.memory_usage))
                    .style(Style::default().fg(mem_color).add_modifier(Modifier::BOLD)),
                Cell::from(process.user.clone()).style(Style::default().fg(theme.text_secondary)),
                Cell::from(process.state.to_string()).style(
                    Style::default()
                        .fg(state_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Cell::from(process.threads.to_string())
                    .style(Style::default().fg(theme.text_secondary)),
            ])
            .style(Style::default().bg(bg_color))
        })
        .collect();
    let table = Table::new(
        rows,
        vec![
            Constraint::Length(8),
            Constraint::Percentage(25),
            Constraint::Length(8),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(8),
        ],
    )
    .header(header)
    .block(
        Block::default()
            .title(" Processes ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border)),
    );
    let detail_block = Block::default()
        .title(" Process Details ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border_light));
    let details = if selected_process < metrics.processes.len() {
        let process = &metrics.processes[selected_process];
        vec![
            Line::from(vec![
                Span::styled("PID: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    process.pid.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
                Span::raw(" | "),
                Span::styled("PPID: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    process.ppid.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
            Line::from(vec![
                Span::styled("Command: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    &process.full_command,
                    Style::default().fg(theme.text_secondary),
                ),
            ]),
            Line::from(vec![
                Span::styled("Start Time: ", Style::default().fg(theme.text_dim)),
                Span::styled(&process.start_time, Style::default().fg(theme.text_primary)),
                Span::raw(" | "),
                Span::styled("Uptime: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    format_duration(process.uptime),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
            Line::from(vec![
                Span::styled("Priority: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    process.priority.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
                Span::raw(" | "),
                Span::styled("Nice: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    process.nice.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
            Line::from(vec![
                Span::styled("I/O Read: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    format!("{} KB/s", process.read_speed),
                    Style::default().fg(theme.success),
                ),
                Span::raw(" | "),
                Span::styled("I/O Write: ", Style::default().fg(theme.text_dim)),
                Span::styled(
                    format!("{} KB/s", process.write_speed),
                    Style::default().fg(theme.danger),
                ),
            ]),
        ]
    } else {
        vec![Line::from("No process selected")]
    };
    let detail_para = Paragraph::new(details).block(Block::default());
    let detail_block_clone = detail_block.clone();
    Box::new(move |f: &mut ratatui::Frame| {
        f.render_widget(table, layout[1]);
        f.render_widget(detail_block_clone, layout[2]);
        f.render_widget(detail_para, detail_block.inner(layout[2]));
    })
}

pub fn render_resources_view<'a>(
    area: Rect,
    theme: &'a Theme,
    metrics: &'a SystemInfo,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Min(8),
        ])
        .split(area);
    let cpu_block = Block::default()
        .title(Span::styled(
            " CPU History ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let cpu_area = cpu_block.inner(layout[0]);
    let cpu_data: Vec<(f64, f64)> = metrics
        .cpu_history
        .iter()
        .enumerate()
        .map(|(i, &usage)| (i as f64, usage as f64))
        .collect();
    let cpu_data: &'static [(f64, f64)] = Box::leak(cpu_data.into_boxed_slice());
    let mem_data: Vec<(f64, f64)> = metrics
        .memory_history
        .iter()
        .enumerate()
        .map(|(i, &usage)| (i as f64, usage as f64))
        .collect();
    let mem_data: &'static [(f64, f64)] = Box::leak(mem_data.into_boxed_slice());
    let rx_data: Vec<(f64, f64)> = metrics
        .net_rx_history
        .iter()
        .enumerate()
        .map(|(i, &speed)| (i as f64, speed as f64))
        .collect();
    let rx_data: &'static [(f64, f64)] = Box::leak(rx_data.into_boxed_slice());
    let tx_data: Vec<(f64, f64)> = metrics
        .net_tx_history
        .iter()
        .enumerate()
        .map(|(i, &speed)| (i as f64, speed as f64))
        .collect();
    let tx_data: &'static [(f64, f64)] = Box::leak(tx_data.into_boxed_slice());
    Box::new(move |f: &mut ratatui::Frame| {
        let cpu_chart = Chart::new(vec![
            Dataset::default()
                .name("CPU Usage")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(theme.cpu_colors[0]))
                .data(cpu_data),
        ])
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, cpu_data.len() as f64 - 1.0])
                .labels(vec![
                    Span::styled("-60s", Style::default().fg(theme.text_dim)),
                    Span::styled("now", Style::default().fg(theme.text_dim)),
                ]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::styled("0%", Style::default().fg(theme.text_dim)),
                    Span::styled("50%", Style::default().fg(theme.text_dim)),
                    Span::styled("100%", Style::default().fg(theme.text_dim)),
                ]),
        );
        f.render_widget(cpu_block.clone(), layout[0]);
        f.render_widget(cpu_chart, cpu_area);
        let mem_block = Block::default()
            .title(Span::styled(
                " Memory History ",
                Style::default()
                    .fg(theme.text_bright)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border));
        let mem_area = mem_block.inner(layout[1]);
        let mem_chart = Chart::new(vec![
            Dataset::default()
                .name("Memory Usage")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(theme.mem_colors[0]))
                .data(mem_data),
        ])
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, mem_data.len() as f64 - 1.0])
                .labels(vec![
                    Span::styled("-60s", Style::default().fg(theme.text_dim)),
                    Span::styled("now", Style::default().fg(theme.text_dim)),
                ]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, 100.0])
                .labels(vec![
                    Span::styled("0%", Style::default().fg(theme.text_dim)),
                    Span::styled("50%", Style::default().fg(theme.text_dim)),
                    Span::styled("100%", Style::default().fg(theme.text_dim)),
                ]),
        );
        f.render_widget(mem_block, layout[1]);
        f.render_widget(mem_chart, mem_area);
        let net_block = Block::default()
            .title(Span::styled(
                " Network History ",
                Style::default()
                    .fg(theme.text_bright)
                    .add_modifier(Modifier::BOLD),
            ))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border));
        let net_area = net_block.inner(layout[2]);
        let net_chart = Chart::new(vec![
            Dataset::default()
                .name("Download")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(theme.net_colors[0]))
                .data(rx_data),
            Dataset::default()
                .name("Upload")
                .marker(symbols::Marker::Braille)
                .graph_type(GraphType::Line)
                .style(Style::default().fg(theme.net_colors[1]))
                .data(tx_data),
        ])
        .x_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, rx_data.len() as f64 - 1.0])
                .labels(vec![
                    Span::styled("-45s", Style::default().fg(theme.text_dim)),
                    Span::styled("now", Style::default().fg(theme.text_dim)),
                ]),
        )
        .y_axis(
            Axis::default()
                .style(Style::default().fg(theme.text_dim))
                .bounds([0.0, 2000.0])
                .labels(vec![
                    Span::styled("0 KB/s", Style::default().fg(theme.text_dim)),
                    Span::styled("1 MB/s", Style::default().fg(theme.text_dim)),
                    Span::styled("2 MB/s", Style::default().fg(theme.text_dim)),
                ]),
        );
        f.render_widget(net_block, layout[2]);
        f.render_widget(net_chart, net_area);
    })
}

pub fn render_network_view<'a>(
    area: Rect,
    theme: &'a Theme,
    metrics: &'a SystemInfo,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),
            Constraint::Min(8),
            Constraint::Length(8),
        ])
        .split(area);
    let iface_block = Block::default()
        .title(Span::styled(
            " Network Interfaces ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let iface_area = iface_block.inner(layout[0]);
    let iface_rows: Vec<Row> = metrics
        .network_interfaces
        .iter()
        .map(|iface| {
            Row::new(vec![
                Cell::from(iface.name.clone()).style(Style::default().fg(theme.text_primary)),
                Cell::from(iface.ip_address.clone())
                    .style(Style::default().fg(theme.text_secondary)),
                Cell::from(format!("{:.1} MB/s", iface.rx_speed as f64 / 1024.0)).style(
                    Style::default()
                        .fg(theme.net_colors[0])
                        .add_modifier(Modifier::BOLD),
                ),
                Cell::from(format!("{:.1} MB/s", iface.tx_speed as f64 / 1024.0)).style(
                    Style::default()
                        .fg(theme.net_colors[1])
                        .add_modifier(Modifier::BOLD),
                ),
                Cell::from(iface.status.clone()).style(Style::default().fg(
                    if iface.status == "up" {
                        theme.success
                    } else {
                        theme.danger
                    },
                )),
            ])
        })
        .collect();
    let iface_table = Table::new(
        iface_rows,
        vec![
            Constraint::Length(10),
            Constraint::Length(20),
            Constraint::Length(15),
            Constraint::Length(15),
            Constraint::Length(8),
        ],
    )
    .block(Block::default());
    let conn_block = Block::default()
        .title(Span::styled(
            " Active Connections ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let conn_area = conn_block.inner(layout[1]);
    let connections = vec![
        (
            "TCP",
            "192.168.1.100:443",
            "93.184.216.34:443",
            "ESTABLISHED",
            "firefox",
        ),
        (
            "TCP",
            "192.168.1.100:55555",
            "151.101.1.69:443",
            "ESTABLISHED",
            "curl",
        ),
        (
            "UDP",
            "192.168.1.100:5353",
            "224.0.0.251:5353",
            "LISTEN",
            "systemd",
        ),
        (
            "TCP",
            "192.168.1.100:22",
            "192.168.1.50:65432",
            "ESTABLISHED",
            "sshd",
        ),
        (
            "TCP",
            "127.0.0.1:5432",
            "127.0.0.1:45678",
            "ESTABLISHED",
            "postgres",
        ),
    ];
    let conn_rows: Vec<Row> = connections
        .iter()
        .map(|(proto, local, remote, state, process)| {
            let state_color = match *state {
                "ESTABLISHED" => theme.success,
                "LISTEN" => theme.info,
                "TIME_WAIT" => theme.warning,
                _ => theme.danger,
            };
            Row::new(vec![
                Cell::from(*proto).style(Style::default().fg(theme.text_primary)),
                Cell::from(*local).style(Style::default().fg(theme.text_secondary)),
                Cell::from(*remote).style(Style::default().fg(theme.text_secondary)),
                Cell::from(*state).style(Style::default().fg(state_color)),
                Cell::from(*process).style(Style::default().fg(theme.text_primary)),
            ])
        })
        .collect();
    let conn_table = Table::new(
        conn_rows,
        vec![
            Constraint::Length(8),
            Constraint::Length(25),
            Constraint::Length(25),
            Constraint::Length(15),
            Constraint::Length(15),
        ],
    )
    .block(Block::default());
    let stats_block = Block::default()
        .title(Span::styled(
            " Network Statistics ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let stats_area = stats_block.inner(layout[2]);
    let stats_text = vec![
        Line::from(vec![
            Span::styled("Total RX: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.2} GB", metrics.total_rx as f64 / 1024.0 / 1024.0),
                Style::default()
                    .fg(theme.net_colors[0])
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled("Total TX: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:.2} GB", metrics.total_tx as f64 / 1024.0 / 1024.0),
                Style::default()
                    .fg(theme.net_colors[1])
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Current RX: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} KB/s", metrics.total_rx),
                Style::default()
                    .fg(theme.net_colors[0])
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled("Current TX: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} KB/s", metrics.total_tx),
                Style::default()
                    .fg(theme.net_colors[1])
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    let stats_para = Paragraph::new(stats_text).block(Block::default());
    Box::new(move |f: &mut ratatui::Frame| {
        f.render_widget(iface_block, layout[0]);
        f.render_widget(iface_table, iface_area);
        f.render_widget(conn_block, layout[1]);
        f.render_widget(conn_table, conn_area);
        f.render_widget(stats_block, layout[2]);
        f.render_widget(stats_para, stats_area);
    })
}

pub fn render_disks_view<'a>(
    area: Rect,
    theme: &'a Theme,
    metrics: &'a SystemInfo,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(8),
        ])
        .split(area);
    let disk_block = Block::default()
        .title(Span::styled(
            " Disk Usage ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let disk_area = disk_block.inner(layout[1]);
    let disk_rows: Vec<Row> = metrics
        .disks
        .iter()
        .map(|disk| {
            let usage_color = theme.get_usage_color(disk.usage);
            let bar_width: usize = 20;
            let filled = (disk.usage as f64 * bar_width as f64 / 100.0).round() as usize;
            let bar = format!(
                "[{}{}]",
                "█".repeat(filled),
                "░".repeat(bar_width.saturating_sub(filled))
            );
            Row::new(vec![
                Cell::from(disk.name.clone()).style(Style::default().fg(theme.text_primary)),
                Cell::from(disk.mount_point.clone())
                    .style(Style::default().fg(theme.text_secondary)),
                Cell::from(format!("{} GB", disk.total))
                    .style(Style::default().fg(theme.text_primary)),
                Cell::from(format!("{} GB", disk.used)).style(
                    Style::default()
                        .fg(usage_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Cell::from(format!("{} GB", disk.free))
                    .style(Style::default().fg(theme.text_primary)),
                Cell::from(format!("{}%", disk.usage)).style(
                    Style::default()
                        .fg(usage_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Cell::from(bar).style(Style::default().fg(usage_color)),
            ])
        })
        .collect();
    let disk_table = Table::new(
        disk_rows,
        vec![
            Constraint::Length(12),
            Constraint::Length(12),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(10),
            Constraint::Length(8),
            Constraint::Length(25),
        ],
    )
    .block(Block::default());
    let io_block = Block::default()
        .title(Span::styled(
            " Disk I/O Statistics ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let io_area = io_block.inner(layout[2]);
    let total_read: u64 = metrics.disks.iter().map(|d| d.read_speed).sum();
    let total_write: u64 = metrics.disks.iter().map(|d| d.write_speed).sum();
    let io_text = vec![
        Line::from(vec![
            Span::styled("Total Read Speed: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} MB/s", total_read),
                Style::default()
                    .fg(theme.disk_colors[0])
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled("Total Write Speed: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} MB/s", total_write),
                Style::default()
                    .fg(theme.disk_colors[1])
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("Busiest Disk: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                metrics
                    .disks
                    .iter()
                    .max_by_key(|d| d.read_speed + d.write_speed)
                    .map(|d| d.name.clone())
                    .unwrap_or_else(|| "N/A".to_string()),
                Style::default().fg(theme.text_primary),
            ),
        ]),
    ];
    let io_para = Paragraph::new(io_text).block(Block::default());
    Box::new(move |f: &mut ratatui::Frame| {
        f.render_widget(disk_block, layout[1]);
        f.render_widget(disk_table, disk_area);
        f.render_widget(io_block, layout[2]);
        f.render_widget(io_para, io_area);
    })
}

pub fn render_options_view<'a>(
    area: Rect,
    theme: &'a Theme,
    app: &crate::app::App,
) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(1)])
        .split(area);
    let options_block = Block::default()
        .title(Span::styled(
            " Options ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let options_area = options_block.inner(layout[1]);
    let options_text = vec![
        Line::from(vec![
            Span::styled("Update Interval: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{} ms", app.update_interval.as_millis()),
                Style::default().fg(theme.text_primary),
            ),
            Span::raw(" [+/- to adjust]"),
        ]),
        Line::from(vec![
            Span::styled("Paused: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                if app.paused { "Yes" } else { "No" },
                Style::default().fg(if app.paused {
                    theme.danger
                } else {
                    theme.success
                }),
            ),
            Span::raw(" [Space to toggle]"),
        ]),
        Line::from(vec![
            Span::styled("Show Full Command: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                if app.show_full_command { "Yes" } else { "No" },
                Style::default().fg(if app.show_full_command {
                    theme.success
                } else {
                    theme.info
                }),
            ),
            Span::raw(" [f to toggle]"),
        ]),
        Line::from(vec![
            Span::styled("Tree View: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                if app.show_tree_view { "Yes" } else { "No" },
                Style::default().fg(if app.show_tree_view {
                    theme.success
                } else {
                    theme.info
                }),
            ),
            Span::raw(" [F5 to toggle]"),
        ]),
        Line::from(vec![
            Span::styled("Process Aggregation: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                if app.proc_aggregated { "Yes" } else { "No" },
                Style::default().fg(if app.proc_aggregated {
                    theme.success
                } else {
                    theme.info
                }),
            ),
            Span::raw(" [F6 to toggle]"),
        ]),
        Line::from(vec![
            Span::styled("Sort Column: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                format!("{:?}", app.process_sort),
                Style::default().fg(theme.text_primary),
            ),
            Span::raw(" [c/m/p/n to change]"),
        ]),
        Line::from(vec![
            Span::styled("Sort Reverse: ", Style::default().fg(theme.text_dim)),
            Span::styled(
                if app.sort_reverse { "Yes" } else { "No" },
                Style::default().fg(if app.sort_reverse {
                    theme.success
                } else {
                    theme.info
                }),
            ),
            Span::raw(" [←→ to toggle]"),
        ]),
    ];
    let options_para = Paragraph::new(options_text).block(Block::default());
    Box::new(move |f: &mut ratatui::Frame| {
        f.render_widget(options_block, layout[1]);
        f.render_widget(options_para, options_area);
    })
}

pub fn render_help_view<'a>(area: Rect, theme: &'a Theme) -> Box<dyn FnOnce(&mut Frame) + 'a> {
    let help_block = Block::default()
        .title(Span::styled(
            " Help - Key Bindings ",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border));
    let help_area = help_block.inner(area);
    let help_text = vec![
        Line::from(vec![Span::styled(
            "Navigation:",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  [1-6]    Switch between views")]),
        Line::from(vec![Span::raw("  [Tab]     Cycle through views")]),
        Line::from(vec![Span::raw("  [q/Esc]   Quit the application")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Process View:",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  [↑↓/jk]   Navigate processes")]),
        Line::from(vec![Span::raw("  [Page Up/Down] Scroll page")]),
        Line::from(vec![Span::raw("  [Home/End]    Jump to top/bottom")]),
        Line::from(vec![Span::raw("  [Enter]       Show process details")]),
        Line::from(vec![Span::raw(
            "  [c/m/p/n]     Sort by CPU/Memory/PID/Name",
        )]),
        Line::from(vec![Span::raw("  [←→]          Toggle sort order")]),
        Line::from(vec![Span::raw("  [f]           Toggle full command")]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "General:",
            Style::default()
                .fg(theme.text_bright)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(vec![Span::raw("  [Space]    Pause/Resume updates")]),
        Line::from(vec![Span::raw(
            "  [+/-]      Increase/Decrease update speed",
        )]),
        Line::from(vec![Span::raw("  [r]        Reset selection")]),
        Line::from(vec![Span::raw("  [F1]       Show/hide this help")]),
        Line::from(vec![Span::raw("  [F5]       Toggle tree view")]),
        Line::from(vec![Span::raw("  [F6]       Toggle process aggregation")]),
    ];
    let help_para = Paragraph::new(help_text)
        .block(Block::default())
        .wrap(ratatui::widgets::Wrap { trim: true });
    Box::new(move |f: &mut ratatui::Frame| {
        f.render_widget(help_block, area);
        f.render_widget(help_para, help_area);
    })
}

fn format_duration(duration: std::time::Duration) -> String {
    let secs = duration.as_secs();
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m {}s", secs / 60, secs % 60)
    } else if secs < 86400 {
        format!("{}h {}m", secs / 3600, (secs % 3600) / 60)
    } else {
        format!("{}d {}h", secs / 86400, (secs % 86400) / 3600)
    }
}
