use std::default;

use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    widgets::{Block, BorderType, Paragraph, Widget,},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigOption{
    CheckBox(bool),
    Window(Window),
    Pane(Pane),
}

impl Default for ConfigOption{
    fn default()-> Self {
        ConfigOption::CheckBox(false)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Config {
    text: String,
    option: ConfigOption,
}

impl Config{
    pub fn new(name: String) -> Self {
        Self { text: name, option: ConfigOption::default() }
    }
    pub fn with_configoption(mut self, option: ConfigOption) -> Self {
        self.option = option;
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Window {
    name: String,
    configs: Vec<Config>,
    selected_option: u16,
}

impl Window {
    pub fn new(name: String) -> Self {
        Self{
            name: name,
            configs: Vec::new(),
            selected_option: 0,
        }
    }
    pub fn with_configs(mut self, configs: Vec<Config>) -> Self {
        for i in configs {
            self.configs.push(i);
        }
        self
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer){
        Paragraph::new(format!("This is the test terminal page text"))
            .block(
                Block::bordered()
                    .title("TerminalPage")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded),
            )
            .centered()
            .render(area, buf);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum PaneContent{
    #[default]
    PlaceHolder,
}


#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Pane {
    content: PaneContent,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Terminal {
    description: String,
    main_window: Window,
    selected_pane: u16,
}

impl Terminal {
    pub fn new() -> Self {
        let mut configs: Vec<Config> = Vec::new();
        configs.push(
            Config::new("List Controllers".to_string()).with_configoption(ConfigOption::default())
        );

        let window = Window::new("Main menu\n ------------".to_string()).with_configs(configs);  

        Self {
            description: "Terminal Config!".to_string(),
            main_window: window,
            selected_pane: 0,
        }

    }


    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let split = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ]);
        let [main_menu, pane1, pane2] =  split.areas(area);
        self.main_window.render(main_menu, buf);
    }



}
