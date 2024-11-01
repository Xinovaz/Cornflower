use egui::{self, Ui, Response};
use eframe;
use egui_extras;
use catppuccin_egui;

use crate::ui_ast::*;
use crate::data::*;

pub fn render<L: Listener + 'static>(mut ds: DrawSpace) {
	let mut win_size: [f32; 2] = [600.0, 360.0];
	if let DrawDynamic::Static(width, height) = ds.dynamic {
		win_size = [width, height];
	}
	let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(win_size),
        ..Default::default()
    };

    let mut listener = L::start();

	let _ = eframe::run_simple_native(&ds.title, options, move |ctx, _frame| {
        egui_extras::install_image_loaders(&ctx);
        
        // TODO: more customisation
		catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO);
        egui::CentralPanel::default().show(&ctx, |ui| {
            let _ = _render(ui, &mut ds.item, None, &mut listener);
        });
    });
}

fn _render(ui: &mut Ui, d: &mut Drawable, name: Option<String>, listener: &mut impl Listener) -> Option<Response> {
	let response = match d {
		// Layout-Likes
		Drawable::Horizontal(h) => {
			let f = |ui: &mut Ui| {
				for child in h.items.iter_mut() {
					_render(ui, child, None, listener);
				}
			};
			if h.center {
				Some(ui.horizontal_centered(f).response)
			} else {
				Some(ui.horizontal(f).response)
			}
		},
		Drawable::Vertical(v) => {
			let f = |ui: &mut Ui| {
				for child in v.items.iter_mut() {
					_render(ui, child, None, listener);
				}
			};
			if v.center {
				Some(ui.vertical_centered(f).response)
			} else {
				Some(ui.vertical(f).response)
			}
		},
		Drawable::Grid(g) => {
			let n = match name {
				Some(ref new_name) => new_name,
				None => &g.uuid,
			};
			Some(egui::Grid::new(n).striped(g.striped).show(ui, |ui: &mut Ui| {
				for child in g.items.iter_mut() {
					_render(ui, child, None, listener);
				}
			}).response)
		},
		Drawable::Centered(c) => {
			let f = |ui: &mut Ui| {
				_render(ui, &mut c.item, None, listener);
			};
			Some(ui.centered_and_justified(f).response)
		},
		Drawable::Columns(c) => {
			Some(ui.vertical(|ui| { ui.columns(c.number.into(), |cols| {
				cols[0].vertical_centered_justified(|ui| {
					for child in &mut c.items[0] {
						_render(ui, child, None, listener);
					}
				});
				cols[1].vertical_centered_justified(|ui| {
					for child in &mut c.items[1] {
						_render(ui, child, None, listener);
					}
				});
				if c.number > 2 {
					cols[2].vertical_centered_justified(|ui| {
						for child in &mut c.items[2] {
						_render(ui, child, None, listener);
					}
					});
				}
		    })}).response)
		},


		// Widget-Likes
		Drawable::Label(l, enabled) => {
			Some(ui.add_enabled(
				*enabled, 
				egui::Label::new(l.text.to_richtext())
			))
		}
		Drawable::Image(i, enabled) => {
			Some(ui.add_enabled(
				*enabled,
				i.to_image(),
			))
		}
		Drawable::Button(b, enabled) => {
			Some(ui.add_enabled(
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
			))
		},
		Drawable::Select(s, enabled) => {
			let op: Option<String> = match listener.select(&s.name) {
				Some(o) => Some(o.clone()),
				None => match name {
					Some(ref nn) => {
						listener.new_select(s.name.clone(), nn.to_string());
						Some(listener.select(&s.name).unwrap().clone())
					},
					None => {
						listener.new_select(s.name.clone(), "".to_string());
						None
					}
				},
			};
			let res = ui.add_enabled(*enabled, egui::SelectableLabel::new(op == name, s.text.to_richtext()));
			if res.clicked() {
				match name {
					Some(ref new_name) => *listener.select(&s.name).unwrap() = new_name.clone(),
					_ => (),
				};
			}
			Some(res)
		},
		Drawable::Radio(r, enabled) => {
			let op: Option<String> = match listener.select(&r.name) {
				Some(o) => Some(o.clone()),
				None => match name {
					Some(ref nn) => {
						listener.new_select(r.name.clone(), nn.to_string());
						Some(listener.select(&r.name).unwrap().clone())
					},
					None => {
						listener.new_select(r.name.clone(), "".to_string());
						None
					}
				},
			};
			let res = ui.add_enabled(*enabled, egui::RadioButton::new(op == name, r.text.to_richtext()));
			if res.clicked() {
				match name {
					Some(ref new_name) => *listener.select(&r.name).unwrap() = new_name.clone(),
					_ => (),
				};
			}
			Some(res)
		},
		Drawable::Input(i, enabled) => {
			let n = match name {
				Some(ref nn) => nn,
				None => &"_last_text_input".to_string()
			};
			let mut edit = egui::TextEdit::singleline(listener.text_buffer(n)).password(i.password);
			if let Some(h) = &i.hint {
				edit = edit.hint_text(unquote(h.to_string()));
			}
			Some(ui.add_enabled(*enabled, edit))
		},



		// Specials <3
		Drawable::EndRow => {
			ui.end_row(); None
		},

		Drawable::Named(n, draw) => {
			_render(ui, draw, Some(n.clone()), listener)
		},
		_ => None,
	};
	if let Some(n) = name {
		if let Some(ref res) = response {
			if res.clicked() {
				listener.accept(Event::Clicked(n));
			} else if res.clicked_elsewhere() {
				listener.accept(Event::ClickedElsewhere(n));
			} else if res.double_clicked() {
				listener.accept(Event::DoubleClicked(n));
			} else if res.secondary_clicked() {
				listener.accept(Event::AltClicked(n));
			} else if res.hovered() {
				listener.accept(Event::Hovered(n));
			}
		}
	}
	response
}