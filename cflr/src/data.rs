pub trait Listener {
	fn start() -> Self;
	fn run(&mut self);
	fn accept(&mut self, _event: Event) { }
	fn new_select(&mut self, _name: String, _init: String) { }
	fn select(&mut self, _name: &String) -> Option<&mut String> { None }
	fn text_buffer(&mut self, _name: &String) -> &mut String;
}

pub enum Event {
	Clicked(String),
	ClickedElsewhere(String),
	DoubleClicked(String),
	AltClicked(String),
	Hovered(String),
}