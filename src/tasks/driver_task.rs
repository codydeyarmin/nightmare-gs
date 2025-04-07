use std::{fmt, io, time::Duration};

use crate::{event::Event, pages::{Config, ConfigFnOptions, ControlResult, Window}};
use serialport::{available_ports, Error as SerialPortError, DataBits, FlowControl, Parity, SerialPortBuilder, StopBits};
use tokio::sync::mpsc::{self, error::TryRecvError};

#[derive(Debug, Copy, Clone)]
pub enum DriverState{
    Active,
    Connected,
    Disabled,
    Enabled,
}

#[derive(Debug, Clone)]
pub enum DriverError {
    InvalidPort,
    NoPortSet,
    FailedLoadingPorts(SerialPortError),   
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DriverError::InvalidPort => write!(f, "Invalid port"),
            DriverError::NoPortSet => write!(f, "No port setup"),
            DriverError::FailedLoadingPorts(e) => write!(f, "Ports couldn't be read: {}", e),
        }
    }
}

impl From<SerialPortError> for DriverError {
    fn from(err: SerialPortError) -> DriverError {
        DriverError::FailedLoadingPorts(err)
    }
}

#[derive(Debug, Clone)]
pub struct DriverPort {
    port: String,
    baud: u32,
    data_bits: DataBits,
    flow_control: FlowControl,
    parity: Parity,
    stop_bits: StopBits
}

impl DriverPort {
    pub fn new(port: String) -> Result<Self, DriverError>  {
        let ports = available_ports()?;
        if let None = ports.iter().find(|info| *info.port_name == port){
            return Err(DriverError::InvalidPort);
        }
        Ok(
            Self { 
                port, 
                baud: 115200,
                data_bits: DataBits::Eight,
                flow_control: FlowControl::None, 
                parity: Parity::None, 
                stop_bits: StopBits::One,
            }
        )
    }
    
    pub fn with_baud(mut self, baud: u32) -> Self {
        self.baud = baud;
        self
    }

    pub fn get_ports() -> Result<Vec<String>, DriverError> {
        let ports = available_ports()?;
        Ok(ports.iter().map( |port| port.port_name.to_string()).collect())
    }
}

#[derive(Debug)]
pub enum DriverEvent{
    StateChangeError(String),
    StateChange(DriverState),
    SetPort(String),
    StateReport(DriverState),
    Error(serialport::Error)
}

#[derive(Debug)]
pub struct Driver{
    state: DriverState,
    port: Option<SerialPortBuilder>,
    receiver:mpsc::UnboundedReceiver<DriverEvent>,
    sender: mpsc::UnboundedSender<Event>
}

impl Driver {
    pub fn new(receiver:mpsc::UnboundedReceiver<DriverEvent>, sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { 
            state: DriverState::Active, 
            port: None,
            receiver, 
            sender}
    }

    pub fn run(&mut self){
        let mut requested_state: Option<DriverState> = None;
        let mut port = self.port.take().unwrap().open();
        if let Err(e) = port {
            self.sender.send(Event::Driver(DriverEvent::Error(e))).unwrap();
            return;
        } 
        if let Err(e) = port.unwrap().set_timeout(Duration::new(0, 1000000)){
            self.sender.send(Event::Driver(DriverEvent::Error(e))).unwrap();
            return;
        }
        loop {
            let sleep_time = Duration::from_millis(1);
            match self.receiver.try_recv() {
                Ok(event) => {
                    match event {
                        DriverEvent::StateChange(state) => requested_state = Some(state),
                        _ => (),
                    }
                },
                Err(err) => {
                    if let TryRecvError::Disconnected = err {
                        panic!("disconnected channel");
                    }
                }
            }
            //todo remove when done debugging control panel.
            if requested_state.is_some() {
                self.state = requested_state.take().unwrap();
            }
            match &self.state {
                DriverState::Active => {
                    
                },
                DriverState::Connected => {

                },
                DriverState::Disabled => {

                },
                DriverState::Enabled => {

                }
            }
            self.sender.send(Event::Driver(DriverEvent::StateReport(self.state))).unwrap();
            std::thread::sleep(sleep_time);
        }
        
    }
}

// #[derive(Debug)]
// pub enum DriverTaskEvent{
//     Connect,
//     SetPort(String)
// }

#[derive(Debug)]
pub struct DriverTask {
    event_sender: Option<mpsc::UnboundedSender<Event>>,
    //driver_event_sender: mpsc::UnboundedSender<DriverEvent>,
    to_driver_sender: mpsc::UnboundedSender<DriverEvent>,
    to_driver_receiver: Option<mpsc::UnboundedReceiver<DriverEvent>>,
    //driver: Driver,
    task: Option<tokio::task::JoinHandle<()>>,
    port: Option<DriverPort>,
}

impl DriverTask{
    pub fn new() -> Self {
        let (to_driver_sender, to_driver_reciever)
            = mpsc::unbounded_channel();
        Self{
            event_sender : None,
            to_driver_sender,
            to_driver_receiver: Some(to_driver_reciever),
            task: None,
            port: None,
        }
    }

    pub fn set_port(&mut self, name: String) -> Result<(), DriverError> {
        let port = DriverPort::new(name)?;
        self.port = Some(port);
        Ok(())
    }

    pub fn request_state(&mut self, state: DriverState) {
        self.to_driver_sender.send(DriverEvent::StateChange(state)).unwrap()
    }

    pub fn set_sender(&mut self, event_sender: mpsc::UnboundedSender<Event>) {
        self.event_sender = Some(event_sender);
    }

    pub fn start_driver(&mut self) -> Result<(), DriverError> {
        if self.port.is_none() || self.event_sender.is_none() {
            return Err(DriverError::NoPortSet);
        }
        if let Some(_) = self.task {
            //TODO decide what to do if driver already started. 
            return Ok(());
        }
        let to_driver_receiver = self.to_driver_receiver.take().unwrap();
        let sender = self.event_sender.take().unwrap();
        tokio::spawn(async move {
            let mut driver = Driver::new(to_driver_receiver, sender);
            loop {
                driver.run();
            }
        });
        Ok(())
    }

    pub fn list_ports() -> (Window, Option<ControlResult>) {
        //todo fix with error handling
        let ports = DriverPort::get_ports().unwrap();
        (Window::new("Available ports".to_string()).with_configs(ports
            .iter()
            .map(|s| 
                Config::new(s.clone())
                .with_on_select(
                    ConfigFnOptions::ConfigToNone(|config| 
                            Some(ControlResult::DriverChange(DriverEvent::SetPort(config.get_short_text().to_string())))
                )
                )).collect()
            ),
        None)
    }

}
