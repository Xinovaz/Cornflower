use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub grammar);

pub mod ui_ast;
pub mod ui;
pub mod data;

struct Data {

}

impl data::Listener for Data {
    fn start() -> Self {
        Data {

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
}

fn main() {
    let draw = grammar::AppParser::new().parse(r#"


"Window" @ V ! {
    // this is a comment
    G ! {
        Label "" Label "12" Label "13" \
        Label "21" Label "22" Label "23" \
        Label "31" Label "32" Label "33" \
        Label "41" Label "42"
    }
}


"#);
    //println!("{:#?}", draw);
    crate::ui::render::<Data>(draw.unwrap());
}
