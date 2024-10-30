use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

use std::collections::HashMap;

pub mod ui_ast;
pub mod ui;
pub mod data;

struct Data {
    selects: HashMap<String, String>,
}

impl data::Listener for Data {
    fn start() -> Self {
        Data {
            selects: HashMap::new(),
        }
    }

    fn run(&mut self) {

    }

    fn accept(&mut self, event: data::Event) {
        match event {
            data::Event::Clicked(name) => {
                if &name == "my_button" {
                    println!("You clicked the button!");
                }
            },
            data::Event::ClickedElsewhere(name) => {
                if &name== "my_button" {
                    println!("You missed the button!");
                }
            }
            _ => (),
        }
    }

    fn new_select(&mut self, name: String, init: String) {
        self.selects.insert(name, init);
    }

    fn select(&mut self, name: &String) -> Option<&mut String> {
        self.selects.get_mut(name)
    }

    fn text_buffer(&mut self, name: &String) -> &mut String {
        if self.selects.keys().collect::<Vec<&String>>().contains(&name) {
            self.selects.get_mut(name).unwrap()
        } else {
            self.selects.insert(name.to_string(), "".to_string());
            self.selects.get_mut(name).unwrap()
        }
    }
}

fn main() {
    let draw = grammar::AppParser::new().parse(r#"


"Window" @ S {
    // empty
} {
    G {
        Label "" Label ! ($u) 60.0 "Log In" \
        Label "Username:" username = Input \
        Label "Password:" password = Input ! "Your password is safe" \
    }
} {
    // empty
}


"#);
    //println!("{:#?}", draw);
    crate::ui::render::<Data>(draw.unwrap());
}
