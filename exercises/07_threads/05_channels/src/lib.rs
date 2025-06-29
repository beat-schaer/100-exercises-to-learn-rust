use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// Start the system by spawning the server thread.
// It returns a `Sender` instance which can then be used
// by one or more clients to interact with the server.
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

fn execute_server_command(store: &mut store::TicketStore, cmd: Command) {
    match cmd {
        Command::Insert(ticket) => { 
            let _ = store.add_ticket(ticket); 
        },
    }
}

// TODO: The server task should **never** stop.
//  Enter a loop: wait for a command to show up in
//  the channel, then execute it, then start waiting
//  for the next command.
pub fn server(receiver: Receiver<Command>) {
    let mut store = store::TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(command) => execute_server_command(&mut store, command),
            Err(err) => println!("Failed to read receiver: {}", err),
        }
     }
}
