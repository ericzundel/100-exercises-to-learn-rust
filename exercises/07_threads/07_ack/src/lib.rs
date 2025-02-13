use std::sync::mpsc::{Receiver, Sender};
use crate::store::TicketStore;

pub mod data;
pub mod store;

// Refer to the tests to understand the expected schema.
pub enum Command {
    Insert { draft : crate::data::TicketDraft, ack_channel: Sender<store::TicketId> },
    Get { id : store::TicketId, ack_channel: Sender<data::Ticket> }
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: handle incoming commands as expected.
pub fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {draft, ack_channel}) => {
                ack_channel.send(store.add_ticket(draft));
            }
            Ok(Command::Get { id, ack_channel}) => {
                let ticket_ref = store.get(id)
                    .expect("TODO: panic message");
                ack_channel.send(ticket_ref.clone());
            }
            Err(_) => {
                // There are no more senders, so we can safely break
                // and shut down the server.
                break
            },
        }
    }
}
