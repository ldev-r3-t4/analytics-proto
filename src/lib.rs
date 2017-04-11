#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use chrono::{UTC, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    AllChannels {
        total_channels: usize,
        success: bool,
    },
    MostRecentMessages { num_messages: usize, success: bool },
    MoreMessages {
        num_requested: usize,
        num_sent: usize,
        success: bool,
    },
    SendMessage {
        message_length: usize,
        success: bool,
    },
    CreateChannel {
        channel_name_length: usize,
        success: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    received_time: DateTime<UTC>,
    serviced_time: DateTime<UTC>,
    message: Message,
}

#[cfg(test)]
mod tests {
    extern crate serde_json;

    use super::*;
    fn make_event(message: Message) -> Event {
        Event {
            received_time: UTC::now(),
            serviced_time: UTC::now(),
            message: message,
        }
    }

    #[test]
    fn test_all_channels() {
        serde_json::to_writer(&mut std::io::stderr(),
                              &make_event(Message::AllChannels {
                                              total_channels: 5,
                                              success: true,
                                          }))
                .ok();
    }
}

