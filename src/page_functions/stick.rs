use std::task::Poll::{self, Pending, Ready};
use std::future::*;
use futures::{poll, FutureExt};
use stick::{Controller, Event, Listener};

use crate::pages::{
    Window,
    Config,
};



pub fn get_controllers_to_window() -> Window {
    let mut controllers: Vec<String> = Vec::new();
    loop {
        let listener = Listener::default();
        if let Some(controller) = listener.now_or_never(){
            controllers.push(controller.name().to_string());
        }else{
            break;
        }
    }
    Window::new("Controllers".to_string()).with_configs(controllers
        .iter()
        .map(|s| Config::new(s.clone()))
        .collect())
}
