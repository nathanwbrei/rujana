use super::event::Event;


pub struct Subevent<T> {
    pub parent_id : u64,
    pub subevent_id : u64,
    pub subevent_count : u64,
    pub payload : T
}


pub trait SubeventProcessor<T> {
    fn scatter(event : Event) -> Vec<T>;
    fn process(&mut t : T) -> ();
    fn gather(items: Vec<T>, event : Event) -> Event;
}


// TODO: Who owns Event* after split into subevent
// TODO: Remember to handle the case of no subevents. Event must not get lost!
