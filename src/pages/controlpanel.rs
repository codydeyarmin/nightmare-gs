use std::default;

use futures::select;
use ratatui::{
    buffer::Buffer, layout::{Alignment, Constraint, Layout, Rect}, style::{Color, Style, Stylize}, text::{Line, Text}, widgets::{Block, BorderType, Paragraph, Widget,}
};

use crate::page_functions::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub enum ConfigOption{
    #[default]
    Text,
    CheckBox(bool),
    Window(Window),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ConfigFnOptions{
    None(fn()),
    NoneToWindow(fn() -> Window),
    WindowToWindow(fn(window: &Window) -> Window),
}

fn hello_world(){println!("Hello World!");}

impl Default for ConfigFnOptions{
    fn default() -> ConfigFnOptions {
        ConfigFnOptions::None(hello_world)
    }
}


#[derive(Debug,  Default, PartialEq, Eq)]
pub struct Config {
    short_text: String,
    full_text: Option<String>, 
    option: ConfigOption,
    on_select: Option<ConfigFnOptions>
}

impl Config{
    pub fn new(name: String) -> Self {
        Self { short_text: name, full_text: None, option: ConfigOption::default(), on_select: None }
    }

    pub fn with_configoption(mut self, option: ConfigOption) -> Self {
        self.option = option;
        self
    }

    pub fn with_fulltext(mut self, full_text: String) -> Self {
        self.full_text = Some(full_text);
        self
    }

    pub fn with_on_select(mut self, on_select:ConfigFnOptions) -> Self {
        self.on_select = Some(on_select);
        self
    }

    pub fn get_short_text(&self) -> &str {
        &self.short_text.as_ref()
    }

    pub fn get_fulltext(&self) -> &str {
        match &self.full_text {
            Some(string) => string.as_ref(),
            None => self.get_short_text(),
        }
    }

    
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Window {
    name: String,
    content: Vec<Config>,
    window_selected: bool,
    highlighted_content: u16,
    selected_content: Option<u16>,
}

impl Window {
    pub fn new(name: String) -> Self {
        Self{
            name,
            content: Vec::new(),
            window_selected: false,
            highlighted_content: 0,
            selected_content: None,
        }
    }

    pub fn with_configs(mut self, configs: Vec<Config>) -> Self {
        self.content = configs;
        self
    }

    pub fn as_selected(mut self) -> Self {
        self.window_selected = true;
        self
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, next_panes: Vec<Rect>){
        let mut style =  Style::default();
        if self.window_selected {
            style = style.light_cyan();
        }
        let frame = Block::bordered()
            .title(self.name.as_ref())
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .border_style(style);
        let inner_area  = frame.inner(area);
        frame.render(area, buf);
        let mut text = Text::default();
        let mut idx: u16 = 0;
        for i in &self.content {
            let mut style = Style::default();
            if idx == self.highlighted_content {
                style = style.light_cyan();
            } else if let Some(selected_content) = self.selected_content {
                if idx == selected_content {
                    style = style.red();
                }
            }
            let line = Line::from(i.get_short_text()).style(style);
            text.push_line(line);

            idx += 1;
        }
        let mut next_panes = next_panes.clone();
        if let Some(pane) = next_panes.pop(){
            if let Some(selected_content) = self.selected_content{
                match &self.content[selected_content as usize].option{
                    ConfigOption::Window(window) => {
                        window.render(pane, buf, next_panes);
                    },
                    _ => { }
                }
            }
        }
        text.render(inner_area, buf);

    }

    pub fn next_item(&mut self) {
        if self.window_selected {
            if self.window_selected {
                self.highlighted_content = (self.highlighted_content + 1) % self.content.len() as u16;
            } 
        } else {
            if let Some(selected_content) = self.selected_content{
                match &mut self.content[selected_content as usize].option{
                    ConfigOption::Window(window) => {
                        window.next_item()
                    },
                    _ => { panic!("Should hit this in window::next_item()")}
                }
            }
            
        }
        
        
    }

    pub fn previous_item(&mut self) {
        if self.window_selected {
            if self.highlighted_content == 0 {
                self.highlighted_content = self.content.len() as u16 - 1;
            }else {
                self.highlighted_content = self.highlighted_content - 1;
            }
        } else {
            if let Some(selected_content) = self.selected_content{
                match &mut self.content[selected_content as usize].option{
                    ConfigOption::Window(window) => {
                        window.previous_item()
                    },
                    _ => { panic!("Should hit this in window::previous_item()")}
                }
            }
        }
        
    }

    pub fn select(&mut self){
        if self.window_selected {
            self.selected_content = Some(self.highlighted_content);
        } else {
            if let Some(selected_content) = self.selected_content{
                match &mut self.content[selected_content as usize].option{
                    ConfigOption::Window(window) => {
                        window.select()
                    },
                    _ => { panic!("Should hit this in window::select()")}
                }
            }
        }
    }

    pub fn select_window(&mut self, selected_window: u16) -> u16{
        if let Some(selected_content) = self.selected_content{
            match &mut self.content[selected_content as usize].option{
                ConfigOption::Window(window) => {
                    if selected_window == 0 {
                        self.window_selected = true;
                        window.unselect_window();
                        return 0

                    } else {
                        self.window_selected = false;
                        window.select_window(selected_window - 1)
                    }               
                },
                _ => {
                    self.window_selected = true;
                    return 0
                },
            }
        } else{
            self.window_selected = true;
            return 0
        }
    }

    pub fn unselect_window(&mut self){
        self.window_selected = false;
    }

}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ControlPanel {
    description: String,
    main_window: Window,
    selected_window: u16,
}

impl ControlPanel {
    pub fn new() -> Self {
        let mut configs: Vec<Config> = Vec::new();
        configs.push({
            let mut config = Config::new("List Controllers".to_string())
                .with_configoption(ConfigOption::default())
                .with_fulltext("List all controllers connected to the machine".to_string());
            config.option = ConfigOption::Window(Window::new("Controller list window".to_string())
                .with_configs(vec![Config::new("Controller A".to_string()),
                Config::new("Controller =B".to_string())]));
            config
        });
        configs.push({
            let mut config = Config::new("Connect Controller".to_string())
                .with_configoption(ConfigOption::default())
                .with_fulltext("List all controllers connected to the machine".to_string());
            config.option = ConfigOption::Window(Window::new("Controller connect window".to_string())
                .with_configs(vec![Config::new("Controller to be connected".to_string())]));
            config
        });

        let window = Window::new("Main menu".to_string()).with_configs(configs).as_selected();

        Self {
            description: "Terminal Config!".to_string(),
            main_window: window,
            selected_window: 0,
        }

    }


    pub fn render(&self, area: Rect, buf: &mut Buffer) {
        let mut info_text: Option<String> = None;
        let split = Layout::vertical([
            Constraint::Percentage(20),
            Constraint::Percentage(80),
        ]);
        let [info_pane, window] = split.areas(area);
        let split = Layout::horizontal([
            Constraint::Percentage(30),
            Constraint::Percentage(30),
            Constraint::Percentage(40),
        ]);
        let [main_menu, pane1, pane2] =  split.areas(window);
        let panes = vec![pane2, pane1];
        self.main_window.render(main_menu, buf, panes);
    }
    
    pub fn next_item(&mut self) {
        self.main_window.next_item();
    }
    
    pub fn previous_item(&mut self){
        self.main_window.previous_item();
    }

    pub fn select(&mut self){
        self.main_window.select();
    }

    pub fn next_window(&mut self){
        self.selected_window = self.main_window.select_window(self.selected_window + 1);
    }

    pub fn prev_window(&mut self){
        self.selected_window = self.main_window.select_window(self.selected_window.saturating_sub(1));
    }

}
