#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use chrono::{UTC, DateTime};

#[derive(Serialize, Deserialize)]
enum Message {
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

#[derive(Serialize, Deserialize)]
struct Event {
    received_time: DateTime<UTC>,
    serviced_time: DateTime<UTC>,
    message: Message,
}

