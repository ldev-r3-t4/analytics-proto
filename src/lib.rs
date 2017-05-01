//! 
//! This crate contains the API for the protocol of the analytics server. `Event` is the type which is sent to the
//! analytics server. Here are some examples of `Event` structures using different `Message` variants:
//!
//! All Channels: {"received_time":"2017-05-01T21:21:01.070462025Z","serviced_time":"2017-05-01T21:21:01.070464560Z","success":true,"message":{"AllChannels":{"num_channels":5}}}
//!
//! Create Channel: {"received_time":"2017-05-01T21:21:01.070521981Z","serviced_time":"2017-05-01T21:21:01.070522531Z","success":true,"message":{"CreateChannel":{"channel":"boo!"}}}
//!
//! Get Channel: {"received_time":"2017-05-01T21:21:01.070554580Z","serviced_time":"2017-05-01T21:21:01.070555092Z","success":true,"message":{"GetChannel":{"channel":"boo!","number_served":42}}}
//!
//! Delete Channel: {"received_time":"2017-05-01T21:21:01.070586852Z","serviced_time":"2017-05-01T21:21:01.070587314Z","success":true,"message":{"DelChannel":{"channel":"wah!"}}}
//!
//! Send Message: {"received_time":"2017-05-01T21:21:01.070615964Z","serviced_time":"2017-05-01T21:21:01.070616427Z","success":true,"message":{"SendMessage":{"channel":"boo!","message":"woah, you scared me!"}}}
//!
//! Get Message: {"received_time":"2017-05-01T21:21:01.070649678Z","serviced_time":"2017-05-01T21:21:01.070650200Z","success":true,"message":"GetMessage"}
//!
//! Update Message: {"received_time":"2017-05-01T21:21:01.070674641Z","serviced_time":"2017-05-01T21:21:01.070675175Z","success":true,"message":{"UpdateMessage":{"channel":"boo!","old_message":"woah, you scared me!","new_message":"woah, the channel name scared me!"}}}
//!
//! Delete Message: {"received_time":"2017-05-01T21:21:01.070721230Z","serviced_time":"2017-05-01T21:21:01.070721680Z","success":true,"message":{"DeleteMessage":{"channel":"boo!","message":"woah, the channel name scared me!"}}}

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate chrono;

use chrono::{UTC, DateTime};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Message {
    /// All Channels: {"received_time":"2017-05-01T21:21:01.070462025Z","serviced_time":"2017-05-01T21:21:01.070464560Z","success":true,"message":{"AllChannels":{"num_channels":5}}}
    AllChannels {
        num_channels: usize,
    },
    /// Create Channel: {"received_time":"2017-05-01T21:21:01.070521981Z","serviced_time":"2017-05-01T21:21:01.070522531Z","success":true,"message":{"CreateChannel":{"channel":"boo!"}}}
    CreateChannel {
        channel: String,
    },
    /// Get Channel: {"received_time":"2017-05-01T21:21:01.070554580Z","serviced_time":"2017-05-01T21:21:01.070555092Z","success":true,"message":{"GetChannel":{"channel":"boo!","number_served":42}}}
    GetChannel {
        channel: String,
        number_served: usize,
    },
    /// Delete Channel: {"received_time":"2017-05-01T21:21:01.070586852Z","serviced_time":"2017-05-01T21:21:01.070587314Z","success":true,"message":{"DelChannel":{"channel":"wah!"}}}
    DelChannel {
        channel: String,
    },
    /// Send Message: {"received_time":"2017-05-01T21:21:01.070615964Z","serviced_time":"2017-05-01T21:21:01.070616427Z","success":true,"message":{"SendMessage":{"channel":"boo!","message":"woah, you scared me!"}}}
    SendMessage {
        channel: String,
        message: String,
    },
    /// Get Message: {"received_time":"2017-05-01T21:21:01.070649678Z","serviced_time":"2017-05-01T21:21:01.070650200Z","success":true,"message":"GetMessage"}
    GetMessage,
    /// Update Message: {"received_time":"2017-05-01T21:21:01.070674641Z","serviced_time":"2017-05-01T21:21:01.070675175Z","success":true,"message":{"UpdateMessage":{"channel":"boo!","old_message":"woah, you scared me!","new_message":"woah, the channel name scared me!"}}}
    UpdateMessage {
        channel: String,
        old_message: String,
        new_message: String,
    },
    /// Delete Message: {"received_time":"2017-05-01T21:21:01.070721230Z","serviced_time":"2017-05-01T21:21:01.070721680Z","success":true,"message":{"DeleteMessage":{"channel":"boo!","message":"woah, the channel name scared me!"}}}
    DeleteMessage {
        channel: String,
        message: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub received_time: DateTime<UTC>,
    pub serviced_time: DateTime<UTC>,
    pub success: bool,
    pub message: Message,
}

