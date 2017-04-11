extern crate analytics_proto as ap;
extern crate chrono;

use ap::*;
use chrono::UTC;

fn main() {
    extern crate serde_json;
    use std::io::Write;

    fn make_event(message: Message) -> Event {
        Event {
            received_time: UTC::now(),
            serviced_time: UTC::now(),
            message: message,
        }
    }

    serde_json::to_writer(&mut std::io::stderr(),
                          &make_event(Message::AllChannels {
                                          total_channels: 5,
                                          success: true,
                                      }))
            .ok();
    write!(&mut std::io::stderr(), "\n").ok();

    serde_json::to_writer(&mut std::io::stderr(),
                          &make_event(Message::MostRecentMessages {
                                          num_messages: 8,
                                          success: true,
                                      }))
            .ok();
    write!(&mut std::io::stderr(), "\n").ok();

    serde_json::to_writer(&mut std::io::stderr(),
                          &make_event(Message::MoreMessages {
                                          num_requested: 5,
                                          num_sent: 0,
                                          success: false,
                                      }))
            .ok();
    write!(&mut std::io::stderr(), "\n").ok();

    serde_json::to_writer(&mut std::io::stderr(),
                          &make_event(Message::SendMessage {
                                          message_length: 87,
                                          success: true,
                                      }))
            .ok();
    write!(&mut std::io::stderr(), "\n").ok();

    serde_json::to_writer(&mut std::io::stderr(),
                          &make_event(Message::CreateChannel {
                                          channel_name_length: 7,
                                          success: true,
                                      }))
            .ok();
    write!(&mut std::io::stderr(), "\n").ok();
}

