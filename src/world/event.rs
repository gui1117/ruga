use std::cmp::Ordering;
use world::World;

pub struct Event {
	pub date: f64,
	pub execute: &'static Fn(&mut World, EventArgs),
	pub args: EventArgs,
}

pub struct EventSettings {
	pub delta_time: f64,
	pub execute: &'static Fn(&mut World, EventArgs),
	pub args: EventArgs,
}

pub enum EventArgs {
	Float3([f64;3]),
	Usize1(usize),
	Nil,
}

impl PartialEq for Event {
	fn eq(&self, other: &Self) -> bool {
		self.date == other.date
	}

	fn ne(&self, other: &Self) -> bool {
		self.date != other.date
	}
}

impl Eq for Event {
}

impl PartialOrd for Event {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(-self.date).partial_cmp(&-other.date)
	}

	fn lt(&self, other: &Self) -> bool {
		self.date.gt(&other.date)
	}

	fn le(&self, other: &Self) -> bool {
		self.date.ge(&other.date)
	}

	fn gt(&self, other: &Self) -> bool {
		self.date.lt(&other.date)
	}

	fn ge(&self, other: &Self) -> bool {
		self.date.le(&other.date)
	}
}

impl Ord for Event {
	fn cmp(&self, other: &Self) -> Ordering {
		if self.date < other.date {
			return Ordering::Greater;
		} else if self.date > other.date {
			return Ordering::Less;
		} else {
			return Ordering::Equal;
		}
	}
}
