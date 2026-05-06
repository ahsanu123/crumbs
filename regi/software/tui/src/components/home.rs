use super::Component;
use crate::{action::Action, config::Config, extensions::as_key_press_event_trait::EventExt as _};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

pub fn render_table(frame: &mut Frame, area: Rect, table_state: &mut TableState) {
    let header = Row::new(["Ingredient", "Quantity", "Macros"])
        .style(Style::new().bold())
        .bottom_margin(1);

    let rows = [
        Row::new(["Eggplant", "1 medium", "25 kcal, 6g carbs, 1g protein"]),
        Row::new(["Tomato", "2 large", "44 kcal, 10g carbs, 2g protein"]),
        Row::new(["Zucchini", "1 medium", "33 kcal, 7g carbs, 2g protein"]),
        Row::new(["Bell Pepper", "1 medium", "24 kcal, 6g carbs, 1g protein"]),
        Row::new(["Garlic", "2 cloves", "9 kcal, 2g carbs, 0.4g protein"]),
    ];
    let footer = Row::new([
        "Ratatouille Recipe",
        "",
        "135 kcal, 31g carbs, 6.4g protein",
    ]);
    let widths = [
        Constraint::Percentage(30),
        Constraint::Percentage(20),
        Constraint::Percentage(50),
    ];
    let table = Table::new(rows, widths)
        .header(header)
        .footer(footer.italic())
        .column_spacing(1)
        .style(Color::White)
        .row_highlight_style(Style::new().on_black().bold())
        .column_highlight_style(Color::Gray)
        .cell_highlight_style(Style::new().reversed().yellow())
        .highlight_symbol("🍴 ");

    frame.render_stateful_widget(table, area, table_state);
}

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    config: Config,
    table_state: TableState,
}

impl Home {
    pub fn new() -> Self {
        let mut instance = Self::default();

        instance.table_state.select_first();
        instance.table_state.select_first_column();

        instance
    }
}

impl Component for Home {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> color_eyre::Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_config_handler(&mut self, config: Config) -> color_eyre::Result<()> {
        self.config = config;
        Ok(())
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {
                // add any logic here that should run on every render
                if let Some(key) = event::read()?.as_key_press_event() {
                    match key.code {
                        KeyCode::Char('q') => {
                            if let Some(tx) = &self.command_tx {
                                tx.send(Action::Quit).unwrap();
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            self.table_state.select_next_column()
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            self.table_state.select_previous_column()
                        }
                        KeyCode::Char('g') => self.table_state.select_first(),
                        KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
                        KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
                        KeyCode::Char('G') => self.table_state.select_last(),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = frame.area().layout(&layout);

        let title = Line::from_iter([
            Span::from("Table Widget").bold(),
            Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
        ]);

        frame.render_widget(title.centered(), top);

        render_table(frame, main, &mut self.table_state);
        Ok(())
    }
}
