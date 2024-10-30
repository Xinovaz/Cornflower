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
}

fn main() {
    let draw = grammar::AppParser::new().parse(r#"


"Window" @ V {
    H {
        my_selector.first = Select "my_selector" "My First"
        my_selector.second = Select "my_selector" ! ($i #ffff00) "My Second"
    }
    H {
        your_selector.first = Radio "your_selector" "Your First"
        your_selector.second = Radio "your_selector" "Your Second"
    }
}


"#);
    //println!("{:#?}", draw);
    crate::ui::render::<Data>(draw.unwrap());
}
