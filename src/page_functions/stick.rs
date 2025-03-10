use std::task::Poll::{self, Pending, Ready};
use std::future::*;
use futures::{poll, FutureExt};
use stick::{Controller, Event, Listener};

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
    let mut controllers: Vec<String> = Vec::new();
    loop {
        let listener = Listener::default();
        if let Some(controller) = listener.now_or_never(){
            if controllers.contains(&controller.name().to_string()){
                break;
            } else {
                controllers.push(controller.name().to_string());
            }
        }else{
            break;
        }
    }
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
