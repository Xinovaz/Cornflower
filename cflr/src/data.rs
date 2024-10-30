pub trait Listener {
	fn start() -> Self;
	fn run(&mut self);
	fn accept(&mut self, event: Event);
}

pub enum Event {
	Clicked(String),
	ClickedElsewhere(String),
	DoubleClicked(String),
	AltClicked(String),
	Hovered(String),
}