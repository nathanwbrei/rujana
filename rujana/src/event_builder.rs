

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use crate::components::{Duration,Timestamp,SensorId};


#[derive(Debug,PartialEq,Eq)]
pub enum MessageTag { Sample, Heartbeat, NewRun, Finished }

#[derive(Debug)]
pub struct Message {
    pub tag : MessageTag,
    pub timestamp : Timestamp,
    pub sensor_id : SensorId,
    pub payload : f32
}
/*
impl Message {
    pub fn new_sample(sensor_id : SensorId) -> Message {

    }
    pub fn new_heartbeat(sensor_id : SensorId) -> Message {

    }
    pub fn new_changerun() -> Message {
    }
    pub fn new_finish() -> Message {
    }
}
*/

impl PartialEq for Message {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp.eq(&other.timestamp)
    }
}

impl Eq for Message {}

impl PartialOrd for Message {
    fn partial_cmp(&self, other: &Message) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Message {
    fn cmp(&self, other: &Message) -> Ordering {
        self.timestamp.cmp(&other.timestamp)
    }
}

fn find_latest_complete_time(detector_time_map : &HashMap<SensorId,Option<Timestamp>>) -> Option<Timestamp> {
    // map empty => return None
    // at least one entry is None => return None
    // otherwise => return Some(min(entries))

    match detector_time_map.values().min() {
        None => return None,
        Some(x) => return x.clone()
    }
}


// Sample { sensor_id : SensorId, timestamp : Timestamp, payload : Payload },
// Heartbeat { sensor_id : SensorId, timestamp : Timestamp },
// NewRun { timestamp : Timestamp },
// Finished { timestamp : Timestamp }

#[derive(Debug)]
pub struct EventBuildingMailbox {
    inbox : BinaryHeap<Message>,
    outbox : Vec<Message>,
    latest_sample_times: HashMap<SensorId, Option<Timestamp>>,
    latest_event_start : Option<Timestamp>,
    latest_complete_time : Option<Timestamp>,
    event_interval : Duration,
}

impl EventBuildingMailbox {


    fn stage_next_event(&mut self) -> bool {
        loop {
            let peek_time = match self.inbox.peek() {
                None => return false,  // Haven't received any data yet
                Some(m) => m.timestamp
            };
            let latest_complete_time = match self.latest_complete_time {
                None => return false,  // Haven't received data from all detectors yet
                Some(t) => t
            };
            let latest_event_start = match self.latest_event_start {
                None => peek_time,  // Just now starting an event
                Some(t) => t  // Event already underway from a previous call to stage()
            };
            if peek_time > latest_complete_time {
                // We've consumed all messages up until the completion point
                return false;
            }
            if latest_event_start + self.event_interval < peek_time {
                // We've found a message which lies outside our current event
                return true;
            }
            let next_message = match self.inbox.pop() {
                None => return false, // Only way to get here is via race-condition
                Some(m) => m
            };
            self.outbox.push(next_message);
            self.latest_event_start = Some(latest_event_start);
        }
    }


    // Assume all Samples are sorted when we receive them
    fn push(&mut self, messages: Vec<Message>) -> () {

        for m in messages {

            let detector_id = m.sensor_id.clone();
            let timestamp = m.timestamp.clone();
            let prev = self.latest_sample_times.insert(detector_id, Some(timestamp));

            match prev {
                None => panic!("Unexpected detector!"),
                Some(Some(t)) if t > timestamp => panic!("Partial ordering violation!"),
                _ => ()
            }

            if m.tag != MessageTag::Heartbeat {
                self.inbox.push(m);
            }
        }
        self.latest_complete_time = find_latest_complete_time(&self.latest_sample_times);
    }



    fn pop(&mut self, dest: &mut Vec<Message>) -> () {
        dest.clear();
        if self.stage_next_event() {
            for s in self.outbox.drain(..) {
                dest.push(s);
            }
        }
    }


    fn new(event_interval : Duration) -> EventBuildingMailbox {

        EventBuildingMailbox {inbox : BinaryHeap::new(),
                              outbox : Vec::new(),
                              latest_sample_times : HashMap::new(),
                              latest_event_start : None,
                              latest_complete_time : None,
                              event_interval
                             }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn nonsense_test() {

        use crate::event_builder::{EventBuildingMailbox, Message, MessageTag};

        let mut mb = EventBuildingMailbox::new(10, ["bcal", "fcal"]);
        let mut batch = Vec::new();

        let sensor_a = String::from("bcal");
        let sensor_b = String::from("fcal");

        batch.push(Message {tag: MessageTag::Sample, sensor_id: sensor_a.clone(), timestamp: 100, payload: 2.2});
        batch.push(Message {tag: MessageTag::Sample, sensor_id: sensor_b.clone(), timestamp: 101, payload: 8.0});
        batch.push(Message {tag: MessageTag::Sample, sensor_id: sensor_a.clone(), timestamp: 103, payload: 2.2});

        println!("{:?}", batch);

        mb.push(batch);
        println!("{:?}", mb);

        assert_eq!(crate::other::add(2,2), 4);
    }

    #[test]
    fn better_test() {

    }
}


