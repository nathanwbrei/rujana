
use super::event::Event;


pub enum EventSourceError { Success, Finished, TryAgainLater }

trait EventSource {
    fn open(&self) -> ();
    fn next(&self, &mut event : Event) -> EventSourceError;
    fn close(&self) -> ();
}


