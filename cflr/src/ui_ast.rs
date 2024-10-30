use egui::{
	self,
	Style, RichText, text::LayoutJob, Color32, FontSelection, Align,
};
use std::env;
use url::Url;
pub use uuid::Uuid;

#[derive(Debug)]
pub enum DrawDynamic {
	Dynamic,
	Static(f32, f32),
}

#[derive(Debug)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl Color {
	pub fn from_hex_string(hex: &str) -> Self {
		use std::i32;
		let no_prefix = hex.trim_start_matches("#");
		let bytes = i32::from_str_radix(no_prefix, 16).unwrap().to_be_bytes();
		Color {
			red: bytes[1],
			green: bytes[2],
			blue: bytes[3],
		}
	}

	pub fn to_color32(&self) -> Color32 {
		Color32::from_rgb(self.red, self.green, self.blue)
	}
}

#[derive(Debug)]
pub struct StyledText {
	pub text: String,
	pub color: Color,
	pub size: f32,

	pub underline: bool,
	pub strikethrough: bool,
	pub italics: bool,

	pub strong: bool,
	pub weak: bool,

	pub next: Option<Box<StyledText>>,
}

impl StyledText {
	pub fn from_all(text: String, color: Option<Color>, size: Option<f32>, flags: String) -> Self {
		let mut t = Self::from_unstyled(text);
		if let Some(c) = color { t.color = c; }
		if let Some(s) = size { t.size = s; }
		if flags.contains("u") { t.underline = true; }
		if flags.contains("r") { t.strikethrough = true; }
		if flags.contains("i") { t.italics = true; }
		if flags.contains("s") { t.strong = true; }
		if flags.contains("w") { t.weak = true; }
		t
	}

	pub fn from_unstyled(text: String) -> Self {
		let ch = &mut text.chars();
		ch.next();
	    ch.next_back();

		StyledText {
			text: ch.as_str().to_string(),
			color: Color { red: 0xFF, green: 0xFF, blue: 0xFF },
			size: 12.0,
			underline: false, strikethrough: false,
			italics: false, strong: false, weak: false,
			next: None,
		}
	}

	pub fn to_richtext(&self) -> LayoutJob {
		let mut layout_job = LayoutJob::default();
		self._to_richtext(&mut layout_job);
		layout_job
	}

	fn _to_richtext(&self, layout_job: &mut LayoutJob) {
		let mut richtext = RichText::new(&self.text)
			.color(self.color.to_color32())
			.size(self.size);
		if self.underline { richtext = richtext.underline(); }
		if self.strikethrough { richtext = richtext.strikethrough(); }
		if self.italics { richtext = richtext.italics(); }
		if self.strong { richtext = richtext.strong(); }
		if self.weak { richtext = richtext.weak(); }
		richtext.append_to(
			layout_job, 
			&Style::default(), 
			FontSelection::Default,
			Align::Center
		);
		if let Some(n) = &self.next {
			n._to_richtext(layout_job);
		}
	}
}

#[derive(Debug)]
pub struct Image {
	pub path: String,
}

impl Image {
	pub fn to_image(&self) -> egui::Image {
		let mut path = env::current_dir().unwrap();
		let ch = &mut self.path.chars();
		ch.next();
		let _path = ch.as_str().to_string();
		path.push(_path);

		egui::Image::from_uri(Url::from_file_path(path).unwrap().to_string())
	}
}

#[derive(Debug)]
pub struct Button {
	pub icon: Option<Image>,
	pub text: Option<StyledText>,
}

#[derive(Debug)]
pub struct Label {
	pub text: StyledText,
}

#[derive(Debug)]
pub struct Horizontal {
	pub center: bool,
	pub items: Vec<Drawable>,
}

#[derive(Debug)]
pub struct Vertical {
	pub center: bool,
	pub items: Vec<Drawable>,
}

#[derive(Debug)]
pub struct Grid {
	pub uuid: String,
	pub striped: bool,
	pub items: Vec<Drawable>,
}

#[derive(Debug)]
pub enum Drawable {
	// Layout-likes
	Horizontal(Horizontal),
	Vertical(Vertical),
	Grid(Grid),

	// Widget-likes (can be enabled/disabled)
	Label(Label, bool),
	Button(Button, bool),
	Image(Image, bool),

	// Special/Non-Drawable
	Named(String, Box<Drawable>),
	Comment,
	EndRow,
}

/**
 * Centers an item. 
 * If Dynamic, the item takes up the entire DrawSpace.
 * If Static, the item's dimensions are maintained independent of DrawSpace size.
 */
#[derive(Debug)]
pub struct DrawSpace {
	pub dynamic: DrawDynamic,
	pub item: Drawable,
	pub title: String,
}