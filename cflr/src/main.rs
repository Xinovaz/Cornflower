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


"My Cool Window" 800.0 x 400.0 @ V ! {
    my_button = Button ! /cool.png ($u) 40.0 "Click me!"
    Label "Okay!"
}


"#);
    //println!("{:#?}", draw);
    crate::ui::render::<Data>(draw.unwrap());
}
