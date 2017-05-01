//! This crate contains the API for the protocol of the analytics server. `Event` is the type which is sent to the
//! analytics server. Here are some examples of `Event` structures using different `Message` variants:
//!
//! `{"received_time":"2017-04-11T04:29:16.064185621Z","serviced_time":"2017-04-11T04:29:16.064188591Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
//!
//! `{"received_time":"2017-04-11T04:29:16.064324152Z","serviced_time":"2017-04-11T04:29:16.064324934Z","message":{"MostRecentMessages":{"num_messages":8,"success":true}}}`
//!
//! `{"received_time":"2017-04-11T04:29:16.064422755Z","serviced_time":"2017-04-11T04:29:16.064423786Z","message":{"MoreMessages":{"num_requested":5,"num_sent":0,"success":false}}}`
//!
//! `{"received_time":"2017-04-11T04:29:16.064537018Z","serviced_time":"2017-04-11T04:29:16.064537698Z","message":{"SendMessage":{"message_length":87,"success":true}}}`
//!
//! `{"received_time":"2017-04-11T04:29:16.064613838Z","serviced_time":"2017-04-11T04:29:16.064614475Z","message":{"CreateChannel":{"channel_name_length":7,"success":true}}}`

use chrono::{UTC, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Message {
    /// JSON Example:
    /// `{"received_time":"2017-04-11T04:29:16.064185621Z","serviced_time":"2017-04-11T04:29:16.064188591Z","message":{"AllChannels":{"total_channels":5,"success":true}}}`
    AllChannels {
        total_channels: usize,
        success: bool,
    },
    /// JSON Example:
    /// `{"received_time":"2017-04-11T04:29:16.064324152Z","serviced_time":"2017-04-11T04:29:16.064324934Z","message":{"MostRecentMessages":{"num_messages":8,"success":true}}}`
    MostRecentMessages { num_messages: usize, success: bool },
    /// JSON Example:
    /// `{"received_time":"2017-04-11T04:29:16.064422755Z","serviced_time":"2017-04-11T04:29:16.064423786Z","message":{"MoreMessages":{"num_requested":5,"num_sent":0,"success":false}}}`
    MoreMessages {
        num_requested: usize,
        num_sent: usize,
        success: bool,
    },
    /// JSON Example:
    /// `{"received_time":"2017-04-11T04:29:16.064537018Z","serviced_time":"2017-04-11T04:29:16.064537698Z","message":{"SendMessage":{"message_length":87,"success":true}}}`
    SendMessage {
        message_length: usize,
        success: bool,
    },
    /// JSON Example:
    /// `{"received_time":"2017-04-11T04:29:16.064613838Z","serviced_time":"2017-04-11T04:29:16.064614475Z","message":{"CreateChannel":{"channel_name_length":7,"success":true}}}`
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
