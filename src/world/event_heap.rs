use std::collections::BinaryHeap;
use std::cmp::{
    Ord,
    Ordering,
    Eq,
    PartialEq,
};

struct TimedEvent<Event> {
    time: f64,
    event: Event
}

impl<Event> Eq for TimedEvent<Event> {
}

impl<Event> PartialEq for TimedEvent<Event> {
    fn eq(&self, other: &Self) -> bool {
        self.time.eq(&other.time)
    }
}

impl<Event> PartialOrd for TimedEvent<Event> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl<Event> Ord for TimedEvent<Event> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.time < other.time {
            Ordering::Less
        } else if self.time == other.time {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }
}

pub struct EventHeap<Event> {
    time: f64,
    heap: BinaryHeap<TimedEvent<Event>>,
}

impl<Event> EventHeap<Event> {
    pub fn new() -> EventHeap<Event> {
        EventHeap {
            time:  0.,
            heap: BinaryHeap::new(),
        }
    }
    pub fn push(&mut self, dt: f64, event: Event) {
        let t_event = TimedEvent {
            time: self.time + dt,
            event: event,
        };
        self.heap.push(t_event);
    }
    pub fn pop(&mut self) -> Option<Event> {
        let mut pop = false;
        if let Some(timed_event) = self.heap.peek() {
            if timed_event.time <= self.time {
                pop = true;
            }
        }

        if pop {
            Some(self.heap.pop().unwrap().event)
        } else {
            None
        }
    }
    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }
}
