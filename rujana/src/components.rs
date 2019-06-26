
use std::sync::Arc;
use crate::blackboard::Blackboard;

// Basic types that I haven't quite figured out yet

pub type Timestamp = u64;
pub type Duration = u64;
pub type SensorId = String;


// Main data containers

pub struct Event {
    run_number : u64,
    event_number : u64,
    start_time : Timestamp,
    end_time : Timestamp,
    payload : Blackboard
}

pub struct Sample<T> {
    pub timestamp : Timestamp,
    pub sensor_id : SensorId,
    pub payload : T
}

pub struct Subevent<T> {
    pub event_number : u64,
    pub subevent_number : u64,
    pub subevent_count : u64,
    pub event : Arc<Event>, // TODO: Rethink this
    pub payload : T,
}


pub enum EventSourceResult { Success, Finished, TryAgainLater }

pub trait EventSource {
    fn open(&self) -> ();
    fn next(&self, event : &Event) -> EventSourceResult;
    fn close(&self) -> ();
}

pub trait Factory<T> {
    fn process(&self, event : &mut Event, output : &mut Vec<T>) -> (); // TODO: Figure out Metadata<T> or T::Metadata
    fn next_run(&self, event: &mut Event) -> ();
}

pub trait SubeventProcessor<T> {
    fn scatter(event : Event) -> Vec<T>;
    fn process(t : &mut T) -> ();
    fn gather(items: Vec<T>, event : Event) -> Event;
}


// TODO: Who owns Event* after split into subevent
// TODO: Remember to handle the case of no subevents. Event must not get lost!