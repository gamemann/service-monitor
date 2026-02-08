use std::sync::{Arc, Mutex};

use crate::config::Config;
use crate::service::Service;

use anyhow::Result;

use std::io::{self, Write};

#[derive(Debug)]
pub enum InputType {
    ServiceList,
    Exit,
}

#[derive(Debug, Clone)]
pub struct UserInput {
    pub cfg: Config,
    pub services: Arc<Mutex<Vec<Service>>>,

    pub input: String,
    pub input_last: Option<String>,
}

static CMD_LIST: &[(&str, InputType)] = &[
    ("list", InputType::ServiceList),
    ("exit", InputType::Exit),
    ("quit", InputType::Exit),
    ("q", InputType::Exit),
];

impl UserInput {
    pub fn new(cfg: Config, services: Arc<Mutex<Vec<Service>>>) -> Self {
        UserInput {
            cfg,
            services,

            input: String::new(),
            input_last: None,
        }
    }

    pub async fn parse(&mut self) -> Result<bool> {
        println!();
        print!("Cmd: ");
        io::stdout().flush()?;

        // We must retrieve stdin FD.
        let stdin = io::stdin();

        self.input.clear();

        let _ = stdin.read_line(&mut self.input)?;

        let input = self.input.trim().to_lowercase();

        let input_type = CMD_LIST.iter().find(|(cmd, _)| cmd == &input);

        println!();

        match input_type {
            Some((_, input_type)) => match input_type {
                InputType::ServiceList => self.list_services().await,
                InputType::Exit => return Ok(false),
            },
            None => println!("Invalid command: {}", input),
        }

        Ok(true)
    }
}
