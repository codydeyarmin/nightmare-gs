use std::task::Poll::{self, Pending, Ready};
use std::future::*;
use futures::{poll, FutureExt};
use gilrs::{Gamepad, Gilrs};

use crate::pages::{
    Config, ConfigFnOptions, ControlResult, Window
};

pub fn list_controllers_window() -> (Window, Option<ControlResult>) {
    get_controllers_to_window(false)
}

pub fn select_controller_window() -> (Window, Option<ControlResult>){
    get_controllers_to_window(true)
}

pub fn get_controllers_to_window(select: bool) -> (Window, Option<ControlResult>) {
    let mut gilrs = Gilrs::new().unwrap();
    let controllers: Vec<String> = gilrs.gamepads().map(|(_, gamepad) |
        gamepad.name().to_string()
    ).collect();
    (Window::new("Controllers".to_string()).with_configs(controllers
        .iter()
        .map(|s| 
            if select{
                Config::new(s.clone()).with_on_select(
                    ConfigFnOptions::ConfigToNone(select_this_controller))
            }else {
                Config::new(s.clone())
            }
            
        )
        .collect()),
    None)
}

pub fn select_this_controller(config: &Config) -> Option<ControlResult>{
    Some(ControlResult::SetController(config.get_short_text().to_string()))
}
