use dioxus::prelude::*;

mod console;
mod logic;
mod frontend;
mod template;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element{
    logic::logic(cx);
    cx.render(rsx!{
        logic::logic {}
    })
}
