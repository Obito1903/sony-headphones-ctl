use std::{future, time::Duration};

use bluer::rfcomm::Stream;
use commands::{mdr::ambient_sound::AmbientSound, Command, DataType};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::timeout,
};

pub mod commands;
pub mod common;

pub struct Client {
    stream: Stream,
    current_asm: Option<AmbientSound>,
}

impl Client {
    pub fn new(stream: Stream) -> Self {
        Client {
            stream,
            current_asm: None,
        }
    }

    pub async fn init(&mut self) {
        let mut raw_command = hex::decode("3e0e01000000023600473c").unwrap();
        let command = Command::try_from(&raw_command[..]).unwrap();
        raw_command = command.clone().try_into().unwrap();
        println!("Sending {:?} | raw : {:?}", command, raw_command);
        self.send_command(command).await.unwrap();

        loop {
            match self.read().await {
                Err(e) => {
                    println!("{:?}", e);
                }
                Ok(cmd) => {
                    println!("Received {:?}", cmd);
                }
            };
            self.send_ack().await.unwrap();
            self.flush().await.unwrap();
        }
    }

    pub async fn read(&mut self) -> Result<Command, common::Error> {
        let mut buffer = vec![0; 1024];
        let len = self
            .stream
            .read(&mut buffer)
            .await
            .map_err(|x| common::Error::new(x.to_string()))?;

        // print!("Received {:?} bytes:  | {:?}", len, &buffer[0..len]);
        let command = commands::Command::try_from(&buffer[0..len])?;
        Ok(command)
    }

    pub async fn wait_ack(&mut self) -> Result<(), common::Error> {
        match timeout(Duration::from_secs(1), self.read()).await {
            Ok(res) => {
                let cmd = res?;
                match cmd.data_type {
                    DataType::Ack() => return Ok(()),
                    _ => {
                        todo!("Handle other commands");
                        // return Err(common::Error::new(format!(
                        //     "Invalid Ack received: {:?}",
                        //     cmd
                        // )));
                    }
                }
            }
            Err(_) => Err(common::Error::new("No Ack received".to_string())),
        }
    }

    pub async fn send_command(&mut self, command: Command) -> Result<usize, common::Error> {
        let bytes: Vec<u8> = command.try_into().unwrap();
        self.stream
            .write(&bytes)
            .await
            .map_err(|x| common::Error::new(x.to_string()))
    }

    pub async fn send_ack(&mut self) -> Result<usize, common::Error> {
        let command = Command {
            seq_number: 0x00000001,
            data_type: commands::DataType::Ack(),
        };
        self.send_command(command).await
    }

    pub async fn send_with_ack(&mut self, command: Command) -> Result<(), common::Error> {
        for _ in 0..3 {
            self.send_command(command.clone()).await?;
            match self.wait_ack().await {
                Ok(_) => return Ok(()),
                Err(_) => {}
            }
        }
        Err(common::Error::new("No Ack received".to_string()))
    }

    pub async fn flush(&mut self) -> Result<(), common::Error> {
        self.stream
            .flush()
            .await
            .map_err(|x| common::Error::new(x.to_string()))
    }
}
