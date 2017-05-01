extern crate analytics_proto as ap;
extern crate chrono;

use ap::*;
use chrono::UTC;

fn main() {
    extern crate serde_json;

    fn make_event(message: Message) -> Event {
        Event {
            received_time: UTC::now(),
            serviced_time: UTC::now(),
            success: true,
            message: message,
        }
    }

    println!("All Channels: {}",
             serde_json::to_string(&make_event(Message::AllChannels { num_channels: 5 })).unwrap());

    println!("Create Channel: {}",
             serde_json::to_string(&make_event(Message::CreateChannel { channel: "boo!".into() }))
                 .unwrap());

    println!("Get Channel: {}",
             serde_json::to_string(&make_event(Message::GetChannel {
                                                   channel: "boo!".into(),
                                                   number_served: 42,
                                               }))
                     .unwrap());

    println!("Delete Channel: {}",
             serde_json::to_string(&make_event(Message::DelChannel { channel: "wah!".into() }))
                 .unwrap());

    println!("Send Message: {}",
             serde_json::to_string(&make_event(Message::SendMessage {
                                                   channel: "boo!".into(),
                                                   message: "woah, you scared me!".into(),
                                               }))
                     .unwrap());

    println!("Get Message: {}",
             serde_json::to_string(&make_event(Message::GetMessage)).unwrap());

    println!("Update Message: {}",
             serde_json::to_string(&make_event(Message::UpdateMessage {
                                                   channel: "boo!".into(),
                                                   old_message: "woah, you scared me!".into(),
                                                   new_message: "woah, the channel name scared me!"
                                                       .into(),
                                               }))
                     .unwrap());

    println!("Delete Message: {}",
             serde_json::to_string(&make_event(Message::DeleteMessage {
                                                   channel: "boo!".into(),
                                                   message: "woah, the channel name scared me!"
                                                       .into(),
                                               }))
                     .unwrap());
}

