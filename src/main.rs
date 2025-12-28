mod app;
mod components;
mod sys_info;
mod theme;
mod ui;
mod utils;

use std::{
    io::{self, Stdout},
    time::Duration,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};

use app::App;
use ui::ui;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let mut app = App::default();
    let res = run_app(&mut terminal, &mut app);
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    if let Err(err) = res {
        println!("Error: {:?}", err);
    }
    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;
        app.update_metrics();
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('1') => app.current_view = app::View::System,
                        KeyCode::Char('2') => app.current_view = app::View::Process,
                        KeyCode::Char('3') => app.current_view = app::View::Resources,
                        KeyCode::Char('4') => app.current_view = app::View::Network,
                        KeyCode::Char('5') => app.current_view = app::View::Disks,
                        KeyCode::Tab => app.cycle_view(),
                        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                        KeyCode::PageDown | KeyCode::Char('J') => app.scroll_page_down(),
                        KeyCode::PageUp | KeyCode::Char('K') => app.scroll_page_up(),
                        KeyCode::Home => app.scroll_top(),
                        KeyCode::End => app.scroll_bottom(),
                        KeyCode::Char('+') => app.increase_update_delay(),
                        KeyCode::Char('-') => app.decrease_update_delay(),
                        KeyCode::Char(' ') => app.toggle_pause(),
                        KeyCode::Char('r') => app.reset_selection(),
                        KeyCode::Enter => app.toggle_process_details(),
                        KeyCode::Char('f') => app.toggle_full_command(),
                        KeyCode::Char('c') => app.change_sort_column(sys_info::ProcessSort::Cpu),
                        KeyCode::Char('m') => app.change_sort_column(sys_info::ProcessSort::Memory),
                        KeyCode::Char('p') => app.change_sort_column(sys_info::ProcessSort::Pid),
                        KeyCode::Char('n') => app.change_sort_column(sys_info::ProcessSort::Name),
                        KeyCode::F(1) => app.toggle_help(),
                        KeyCode::F(5) => app.toggle_tree_view(),
                        KeyCode::F(6) => app.toggle_proc_aggregation(),
                        _ => {}
                    }
                }
            }
        }
        if app.paused {
            app.last_update = std::time::Instant::now();
        }
    }
}
