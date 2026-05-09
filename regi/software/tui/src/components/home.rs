use super::Component;
use crate::{action::Action, config::Config};
use color_eyre::eyre::Ok;
use crossterm::event::{self, KeyCode};
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
        .highlight_symbol("🍴 ")
        .light_blue()
        .rapid_blink();

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

    fn handle_key_event(&mut self, key: event::KeyEvent) -> color_eyre::Result<Option<Action>> {
        match key.code {
            KeyCode::Char('q') => {
                return Ok(Some(Action::Quit));
            }
            KeyCode::Char('l') | KeyCode::Right => self.table_state.select_next_column(),
            KeyCode::Char('h') | KeyCode::Left => self.table_state.select_previous_column(),
            KeyCode::Char('j') | KeyCode::Down => self.table_state.select_next(),
            KeyCode::Char('k') | KeyCode::Up => self.table_state.select_previous(),
            KeyCode::Char('G') => self.table_state.select_last(),
            KeyCode::Char('g') => self.table_state.select_first(),
            _ => {}
        };
        Ok(None)
    }

    fn update(&mut self, action: Action) -> color_eyre::Result<Option<Action>> {
        match action {
            Action::Tick => {}
            Action::Render => {}
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> color_eyre::Result<()> {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = frame.area().layout(&layout);
        let (row, column) = self.table_state.selected_cell().expect("fail to get cell");

        let title = Line::from_iter([
            Span::from("Table Widget").bold(),
            Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
            Span::from(format!(" row: {}, column: {}", row, column)),
        ]);

        frame.render_widget(title.left_aligned(), top);

        render_table(frame, main, &mut self.table_state);
        Ok(())
    }
}
