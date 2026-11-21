use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    UserMessage,
    SystemNotification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: usize,
    pub username: String,
    pub content: String,
    pub message_type: MessageType,
}

impl ChatMessage {
    pub fn to_chat_str(&self) -> String {
        let sent_msg = format!(
            "[{}] {}",
            if self.message_type == MessageType::UserMessage {
                self.username.as_str()
            } else {
                "System"
            },
            self.content
        );

        sent_msg
    }
}

pub struct UserIdTag {
    ptr: *const (),
}

impl UserIdTag {
    pub fn new<T>(item: &T) -> Self {
        Self {
            ptr: item as *const T as *const (),
        }
    }

    pub fn hash(&self) -> usize {
        self.ptr as usize
    }
}
