use std::any::TypeId;
use std::cell::Cell;

use orbtk::prelude::*;

#[derive(Default)]
pub struct MainViewState {
    counter: Cell<i32>,
}

impl MainViewState {
    fn increment(&self) {
        self.counter.set(self.counter.get() + 1);
    }
}

impl State for MainViewState {
    fn update(&self, context: &mut Context<'_>) {
        if self.counter.get() > 0 {
            context.widget().set(Text::from(format!(
                "You clicked me {} times",
                self.counter.get()
            )));
        }
    }
}

widget!(
    MainView<MainViewState> {
        counter_text: Text
    }
);

impl Template for MainView {
    fn template(self, id: Entity, context: &mut BuildContext) -> Self {
        println!("{:?}", id);
        println!("{:?}", TypeId::of::<Text>());
        let state = self.clone_state();

        self.name("MainView").counter_text("").child(
            Stack::create()
                .margin(16.0)
                .child(
                    TextBlock::create()
                        .selector(SelectorValue::new().with("text-block").class("h1"))
                        .text("cargo-node ui example")
                        .build(context),
                )
                .child(
                    Button::create()
                        .margin((0.0, 16.0, 0.0, 0.0))
                        .horizontal_alignment("Start")
                        .text("Click me")
                        .on_click(move |_| {
                            state.increment();
                            true
                        })
                        .build(context),
                )
                .child(
                    TextBlock::create()
                        .margin((0.0, 8.0, 0.0, 0.0))
                        .horizontal_alignment("Start")
                        .text(id)
                        .build(context),
                )
                .build(context),
        )
    }
}

fn main() {
    orbtk::initialize();

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("Ui")
                .position((100.0, 100.0))
                .size(420.0, 730.0)
                .child(MainView::create().build(ctx))
                .build(ctx)
        })
        .run();
}
