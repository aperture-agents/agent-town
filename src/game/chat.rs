//! /src/game/chat.rs
//!
//! central chat log for player speech and system events
//!
//! append-only message history w/ subscriber hooks for broadcast, should be future-looking for
//! integration with aperture graph state as a reducible history

use crate::game::state::GamePhase;

/// whether a chat entry is player speech or a system/event line.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageKind {
    /// player speech; used once discussion I/O calls [`Chat::speech`].
    #[allow(dead_code)]
    Speech,
    System,
}

/// who produced a chat message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sender {
    System,
    Player(usize),
}

/// one entry in log
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub id: usize,
    pub kind: MessageKind,
    pub sender: Sender,
    pub text: String,
    /// phase when the message was recorded
    pub phase: GamePhase,
}

type MessageHook = Box<dyn FnMut(&Message)>;

/// append-only chat w/ broadcast to subscribers
pub struct Chat {
    messages: Vec<Message>,
    next_id: usize,
    hooks: Vec<MessageHook>,
}

impl Chat {
    pub fn new() -> Self {
        let mut chat = Self {
            messages: Vec::new(),
            next_id: 0,
            hooks: Vec::new(),
        };
        chat.subscribe(|msg| match msg.sender {
            Sender::Player(id) => println!("[P{id}] {}", msg.text),
            Sender::System => println!("* {}", msg.text),
        });
        chat
    }

    /// player speech, broadcast to all subscribers and append to log.
    #[allow(dead_code)] // discussion I/O not wired yet
    pub fn speech(&mut self, sender: usize, text: String, phase: GamePhase) -> &Message {
        let id = self.alloc_id();
        self.broadcast(Message {
            id,
            kind: MessageKind::Speech,
            sender: Sender::Player(sender),
            text,
            phase,
        })
    }

    /// system/event line, broadcast to all subscribers and append to log.
    pub fn system(&mut self, text: String, phase: GamePhase) -> &Message {
        let id = self.alloc_id();
        self.broadcast(Message {
            id,
            kind: MessageKind::System,
            sender: Sender::System,
            text,
            phase,
        })
    }

    #[allow(dead_code)] // used by agents/UI later; covered by unit test
    pub fn messages(&self) -> &[Message] {
        &self.messages
    }

    /// reg a hook invoked on every new message of any type
    pub fn subscribe(&mut self, hook: impl FnMut(&Message) + 'static) {
        self.hooks.push(Box::new(hook));
    }

    fn alloc_id(&mut self) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    fn broadcast(&mut self, message: Message) -> &Message {
        self.messages.push(message);
        let idx = self.messages.len() - 1;
        // clone for hooks so that we arent holding a messages borrow across &mut hooks.
        let notified = self.messages[idx].clone();
        for hook in &mut self.hooks {
            hook(&notified);
        }
        &self.messages[idx]
    }
}

impl Default for Chat {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn speech_and_system_append_and_notify() {
        let mut chat = Chat {
            messages: Vec::new(),
            next_id: 0,
            hooks: Vec::new(),
        };
        let seen = Rc::new(RefCell::new(Vec::new()));
        chat.subscribe({
            let seen = Rc::clone(&seen);
            move |msg| seen.borrow_mut().push(msg.clone())
        });

        chat.speech(1, "hello".into(), GamePhase::Discussion);
        chat.system("night falls".into(), GamePhase::Night);

        assert_eq!(chat.messages().len(), 2);
        assert_eq!(chat.messages()[0].kind, MessageKind::Speech);
        assert_eq!(chat.messages()[0].sender, Sender::Player(1));
        assert_eq!(chat.messages()[1].kind, MessageKind::System);
        assert_eq!(chat.messages()[1].sender, Sender::System);
        assert_eq!(seen.borrow().len(), 2);
    }
}
