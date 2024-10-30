pub trait Listener {
	fn start() -> Self;
	fn run(&mut self);
	fn accept(&mut self, event: Event);
}

pub enum Event {
	// Buttons
	ButtonClick(String), ButtonHover(String),
}