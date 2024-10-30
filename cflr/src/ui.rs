use egui::{self, Ui};
use eframe;
use egui_extras;
use crate::ui_ast::*;
use crate::data::*;

pub fn render<L: Listener>(mut ds: DrawSpace) {
	let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([600.0, 360.0]),
        ..Default::default()
    };

	let _ = eframe::run_simple_native("Cornflower App", options, move |ctx, _frame| {
        egui_extras::install_image_loaders(&ctx);
        let mut listener = L::start();
        egui::CentralPanel::default().show(&ctx, |ui| {
            _render(ui, &mut ds.item, None, &mut listener);
        });
    });
}

fn _render(ui: &mut Ui, d: &mut Drawable, name: Option<String>, listener: &mut impl Listener) {
	match d {
		// Layout-Likes
		Drawable::Horizontal(h) => {
			let f = |ui: &mut Ui| {
				for child in h.items.iter_mut() {
					_render(ui, child, None, listener);
				}
			};
			if h.center {
				ui.horizontal_centered(f);
			} else {
				ui.horizontal(f);
			}
		},
		Drawable::Vertical(v) => {
			let f = |ui: &mut Ui| {
				for child in v.items.iter_mut() {
					_render(ui, child, None, listener);
				}
			};
			if v.center {
				ui.vertical_centered(f);
			} else {
				ui.vertical(f);
			}
		},

		// TODO get data
		// Widget-Likes
		Drawable::Label(l, enabled) => {
			ui.add_enabled(
				*enabled, 
				egui::Label::new(l.text.to_richtext())
			);
		}
		Drawable::Image(i, enabled) => {
			ui.add_enabled(
				*enabled,
				i.to_image(),
			);
		}
		Drawable::Button(b, enabled) => {
			let response = ui.add_enabled(
				*enabled, 
				{
					if b.icon.is_some() {
						if b.text.is_some() {
							egui::Button::image_and_text(
								b.icon.as_ref().unwrap().to_image(),
        						b.text.as_ref().unwrap().to_richtext()
							)
						} else {
							egui::Button::image(
								b.icon.as_ref().unwrap().to_image()
        					)
						}
					} else {
						if b.text.is_some() {
							egui::Button::new(
        						b.text.as_ref().unwrap().to_richtext()
							)
						} else {
							egui::Button::new(StyledText::from_unstyled("".to_string()).to_richtext())
						}
					}
				}
			);
			if let Some(n) = name {
				if response.clicked() {
					listener.accept(Event::ButtonClick(n));
				} else if response.hovered() {
					listener.accept(Event::ButtonHover(n));
				}
			}
		},

		Drawable::Named(n, draw) => {
			_render(ui, draw, Some(n.clone()), listener);
		},
		_ => (),
	}
}