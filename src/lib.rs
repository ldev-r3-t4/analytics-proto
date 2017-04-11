#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use chrono::{UTC, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// JSON Appearance:
    /// `{"received_time":"2017-04-11T01:47:13.591956532Z","serviced_time":"2017-04-11T01:47:13.591959763Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    AllChannels {
        total_channels: usize,
        success: bool,
    },
    /// JSON Appearance:
    /// `{"received_time":"2017-04-11T01:47:13.591956532Z","serviced_time":"2017-04-11T01:47:13.591959763Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    MostRecentMessages { num_messages: usize, success: bool },
    /// JSON Appearance:
    /// `{"received_time":"2017-04-11T01:47:13.591956532Z","serviced_time":"2017-04-11T01:47:13.591959763Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    MoreMessages {
        num_requested: usize,
        num_sent: usize,
        success: bool,
    },
    /// JSON Appearance:
    /// `{"received_time":"2017-04-11T01:47:13.591956532Z","serviced_time":"2017-04-11T01:47:13.591959763Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    SendMessage {
        message_length: usize,
        success: bool,
    },
    /// JSON Appearance:
    /// `{"received_time":"2017-04-11T01:47:13.591956532Z","serviced_time":"2017-04-11T01:47:13.591959763Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    CreateChannel {
        channel_name_length: usize,
        success: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub received_time: DateTime<UTC>,
    pub serviced_time: DateTime<UTC>,
    pub message: Message,
}

