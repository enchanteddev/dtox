use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::Layout,
    style::{Color, Style},
    symbols,
    text::Text,
    widgets::LineGauge,
};

fn render_cpu_usage(cpu_id: u8, cpu_usage: f64, frame: &mut ratatui::Frame, base: ratatui::layout::Rect) {
    let layout = Layout::default()
        .direction(ratatui::layout::Direction::Horizontal)
        .constraints(
            [
                ratatui::layout::Constraint::Percentage(93),
                ratatui::layout::Constraint::Percentage(2),
                ratatui::layout::Constraint::Percentage(5),
            ]
            .as_ref(),
        )
        .split(base);
    frame.render_widget(
        LineGauge::default()
            .label(format!("CPU{cpu_id} Usage"))
            .ratio(cpu_usage / 100.)
            .filled_style(Style::default().fg(Color::Red))
            .line_set(symbols::line::THICK),
        layout[0],
    );
    frame.render_widget(Text::raw(format!("{cpu_usage:.2}%")), layout[2]);
}

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let mut sys = sysinfo::System::new_all();
    loop {
        let cpus = sys.cpus();
        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints(
                    vec![
                        ratatui::layout::Constraint::Percentage(100 / cpus.len() as u16); cpus.len()
                    ],
                )
                .split(frame.area());
            for (id, cpu) in cpus.iter().enumerate() {
                let cpu_usage = cpu.cpu_usage();
                render_cpu_usage(id as u8, cpu_usage as f64, frame, layout[id]);
            }
        })?;


        match event::poll(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL) {
            Ok(found) if found => {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
            _ => {}
        }
        sys.refresh_all();
    }
    ratatui::restore();
    Ok(())
}
