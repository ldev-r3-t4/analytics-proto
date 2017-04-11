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

    #[test]
    fn test_most_recent_messages() {
        serde_json::to_writer(&mut std::io::stderr(),
                              &make_event(Message::MostRecentMessages {
                                              num_messages: 8,
                                              success: true,
                                          }))
                .ok();
    }

    #[test]
    fn test_more_messages() {
        serde_json::to_writer(&mut std::io::stderr(),
                              &make_event(Message::MoreMessages {
                                              num_requested: 5,
                                              num_sent: 0,
                                              success: false,
                                          }))
                .ok();
    }

    #[test]
    fn test_send_message() {
        serde_json::to_writer(&mut std::io::stderr(),
                              &make_event(Message::SendMessage {
                                              message_length: 87,
                                              success: true,
                                          }))
                .ok();
    }

    #[test]
    fn test_create_channel() {
        serde_json::to_writer(&mut std::io::stderr(),
                              &make_event(Message::CreateChannel {
                                              channel_name_length: 7,
                                              success: true,
                                          }))
                .ok();
    }
}

