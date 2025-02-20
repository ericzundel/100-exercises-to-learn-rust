use tokio::sync::mpsc;

/// TODO: the code below will deadlock because it's using std's channels,
///  which are not async-aware.
///  Rewrite it to use `tokio`'s channels primitive (you'll have to touch
///  the testing code too, yes).
///
/// Can you understand the sequence of events that can lead to a deadlock?

pub struct Message {
    payload: String,
    response_channel: mpsc::UnboundedSender<Message>,
}

/// Replies with `pong` to any message it receives, setting up a new
/// channel to continue communicating with the caller.
pub async fn pong(mut receiver: mpsc::UnboundedReceiver<Message>) {
    loop {
        if let msg= receiver.recv().await.unwrap() {
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::unbounded_channel();
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .unwrap();
            receiver = new_receiver;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{pong, Message};
    use std::sync::mpsc;

    #[tokio::test]
    async fn ping() {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let (response_sender, mut response_receiver) = tokio::sync::mpsc::unbounded_channel();
        sender
            .send(Message {
                payload: "pong".into(),
                response_channel: response_sender,
            })
            .unwrap();

        tokio::spawn(pong(receiver));

        let answer = response_receiver.recv().await.unwrap().payload;
        assert_eq!(answer, "pong");
    }
}
