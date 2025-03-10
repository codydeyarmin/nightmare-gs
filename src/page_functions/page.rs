use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::pages::{Config, ConfigFnOptions, ControlResult, Page, Window};

pub fn list_pages() -> (Window, Option<ControlResult>) {
    let pages: Vec<Config> = Page::iter()
    .map(|page| 
        Config::new(page.to_string())
        .with_on_select(
            ConfigFnOptions::ConfigToNone(select_this_controller))
    )  
    .collect();

    (Window::new("Availabe Pages".to_string())
    .with_configs(pages),
    None)
}

pub fn select_this_controller(config: &Config) -> Option<ControlResult>{
    Some(ControlResult::ChangePage(
        Page::from_str(config.get_short_text()).unwrap()
    )
    )
}