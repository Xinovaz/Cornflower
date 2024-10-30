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
            data::Event::ButtonClick(name) => {
                println!("{} was clicked!", name);
            },
            _ => (),
        }
    }
}

fn main() {
    let draw = grammar::AppParser::new().parse(r#"


@ V ! {
    my_button = Button ! /cool.png ($u) 40.0 "Click me!"
    Label "My Label"
    Label "Your Label"
}


"#);
    //println!("{:#?}", draw);
    crate::ui::render::<Data>(draw.unwrap());
}
