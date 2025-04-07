
//use futures::{FutureExt, StreamExt};
use gilrs::{Gilrs, Event as GamepadEvent};
use tokio::sync::mpsc;
use crate::event::Event;

#[derive(Debug)]
pub struct ControllerTask {
    sender: Option<mpsc::UnboundedSender<Event>>,
    controller: Option<String>,
    task: Option<tokio::task::JoinHandle<()>>,
}

impl ControllerTask {
    pub fn new() -> Self {
        Self {
            sender: None,
            controller: None,
            task: None,
        }
    }

    pub fn add_sender(&mut self, sender: &mpsc::UnboundedSender<Event>){
        self.sender = Some(sender.clone());
    }

    pub fn add_controller(&mut self, controller:String) {
        self.controller = Some(controller);
    }

    pub fn add_task(&mut self){
        let _sender = self.sender.as_mut().unwrap().clone();
        let controller_name = self.controller.as_ref().unwrap().clone();
        let mut gilrs = Gilrs::new().unwrap();
        let _controller_id = gilrs.gamepads().filter(|(_, gamepad)| {
            controller_name.eq(gamepad.name())})
            .map(|(id, _)| id)
            .next().unwrap();
        self.task = Some(
            tokio::spawn(async move {
                loop {
                    while let Some(GamepadEvent { id, event, time, .. }) = gilrs.next_event() {
                        if id == _controller_id {
                            _sender.send(Event::Controller(event));
                        }
                    }
                }
        }));
    }
}