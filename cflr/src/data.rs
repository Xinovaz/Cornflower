pub trait Listener {
	fn start() -> Self;
	fn run(&mut self);
	fn accept(&mut self, event: Event) { }
	fn new_select(&mut self, name: String, init: String) { }
	fn select(&mut self, name: &String) -> Option<&mut String> { None }

}

pub enum Event {
	Clicked(String),
	ClickedElsewhere(String),
	DoubleClicked(String),
	AltClicked(String),
	Hovered(String),
}