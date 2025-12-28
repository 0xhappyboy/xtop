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
    theme::Theme,
};

pub fn ui(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.size();
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(size);
    let header = components::render_header(main_layout[0], &theme, &app.metrics);
    f.render_widget(header, main_layout[0]);
    render_content(f, main_layout[1], app, &theme);
    let footer = components::render_footer(
        main_layout[2],
        &theme,
        &view_to_str(app.current_view),
        app.show_help,
    );
    f.render_widget(footer, main_layout[2]);
    if app.show_help {
        render_help_overlay(f, size, &theme);
    }
}

fn render_content(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    match app.current_view {
        View::System => {
            let system_widget = components::render_system_view(area, theme, &app.metrics);
            system_widget(f);
        }
        View::Process => {
            let max_rows = (area.height as usize).saturating_sub(11);
            let process_widget = components::render_process_view(
                area,
                theme,
                &app.metrics,
                app.selected_process,
                app.process_scroll_offset,
                max_rows,
                app.show_full_command,
            );
            process_widget(f);
        }
        View::Resources => {
            let resources_widget = components::render_resources_view(area, theme, &app.metrics);
            resources_widget(f);
        }
        View::Network => {
            let network_widget = components::render_network_view(area, theme, &app.metrics);
            network_widget(f);
        }
        View::Disks => {
            let disks_widget: Box<dyn FnOnce(&mut Frame<'_>)> =
                components::render_disks_view(area, theme, &app.metrics);
            disks_widget(f);
        }
        View::Options => {
            let options_widget = components::render_options_view(area, theme, app);
            options_widget(f);
        }
    }
    if app.show_proc_details && !app.show_help && app.current_view == View::Process {
        render_process_details_overlay(f, area, app, theme);
    }
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

fn render_process_details_overlay(f: &mut Frame, area: Rect, app: &App, theme: &Theme) {
    if area.height < 20 {
        return;
    }
    let detail_height = 12.min(area.height / 2);
    let detail_area = Rect::new(
        area.x,
        area.y + area.height - detail_height,
        area.width,
        detail_height,
    );
    let overlay = Paragraph::new("").style(Style::default().bg(theme.bg_dark));
    f.render_widget(overlay, detail_area);
    if app.selected_process < app.metrics.processes.len() {
        let process = &app.metrics.processes[app.selected_process];
        let detail_block = ratatui::widgets::Block::default()
            .title(Span::styled(
                format!(
                    " Process Details - {} (PID: {}) ",
                    process.name, process.pid
                ),
                Style::default()
                    .fg(theme.text_bright)
                    .add_modifier(ratatui::style::Modifier::BOLD),
            ))
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(Style::default().fg(theme.border_light))
            .style(Style::default().bg(theme.bg_normal));
        let detail_content = ratatui::widgets::Paragraph::new(vec![
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Command: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    &process.full_command,
                    Style::default().fg(theme.text_secondary),
                ),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("User: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(&process.user, Style::default().fg(theme.text_primary)),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("State: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    process.state.to_string(),
                    Style::default().fg(match process.state {
                        crate::sys_info::ProcessState::Running => theme.success,
                        crate::sys_info::ProcessState::Sleeping => theme.info,
                        _ => theme.warning,
                    }),
                ),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("CPU: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    format!("{:.1}%", process.cpu_usage),
                    Style::default()
                        .fg(if process.cpu_usage > 50.0 {
                            theme.danger
                        } else {
                            theme.success
                        })
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("Memory: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    format!(
                        "{} MB ({:.1}%)",
                        process.memory_usage, process.memory_percent
                    ),
                    Style::default()
                        .fg(if process.memory_percent > 10.0 {
                            theme.danger
                        } else {
                            theme.success
                        })
                        .add_modifier(ratatui::style::Modifier::BOLD),
                ),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Threads: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    process.threads.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("Priority: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    process.priority.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("Nice: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    process.nice.to_string(),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("I/O Read: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    format!("{} KB/s", process.read_speed),
                    Style::default().fg(theme.success),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("I/O Write: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    format!("{} KB/s", process.write_speed),
                    Style::default().fg(theme.danger),
                ),
            ]),
            ratatui::text::Line::from(vec![
                ratatui::text::Span::styled("Start Time: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    &process.start_time,
                    Style::default().fg(theme.text_primary),
                ),
                ratatui::text::Span::raw(" | "),
                ratatui::text::Span::styled("Uptime: ", Style::default().fg(theme.text_dim)),
                ratatui::text::Span::styled(
                    format_duration(process.uptime),
                    Style::default().fg(theme.text_primary),
                ),
            ]),
        ])
        .block(ratatui::widgets::Block::default())
        .wrap(ratatui::widgets::Wrap { trim: true });
        let detail_block_clone = detail_block.clone();
        f.render_widget(detail_block, detail_area);
        f.render_widget(detail_content, detail_block_clone.inner(detail_area));
    }
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
