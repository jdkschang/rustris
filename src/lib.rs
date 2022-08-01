use tetris::{Direction, Tetris};
use js_sys::{Function, Reflect};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_react::{
    c, export_components, h,
    hooks::{use_callback, use_state, use_effect, Deps},
    props::Style,
    Component,
};
use web_sys::{KeyboardEvent, window};

mod shape;
mod tetris;

pub struct App {
    width: u32,
    height: u32,
}

impl TryFrom<JsValue> for App {
    type Error = JsValue;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        Ok(App {
            width: Reflect::get(&value, &"width".into())?.as_f64().unwrap_or(10.0) as u32,
            height: Reflect::get(&value, &"height".into())?.as_f64().unwrap_or(30.0) as u32,
        })
    }
}

impl Component for App {
    fn render(&self) -> wasm_react::VNode {
        let tetris = use_state(|| Tetris::new(self.width, self.height));

        use_effect({
            let tetris = tetris.clone();

            move || {
                let tick_closure = Closure::new({
                    let mut tetris = tetris.clone();
                    move || {
                        tetris.set(|mut tetris| {
                            tetris.tick();
                            tetris
                        })
                    }
                });

                let handle = window()
                    .unwrap_throw()
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        tick_closure.as_ref().dyn_ref::<Function>().unwrap_throw(),
                        500)
                    .unwrap_throw();

                // destructor
                move || {
                    drop(tick_closure);
                    window()
                        .unwrap_throw()
                        .clear_interval_with_handle(handle)
                }
            }
        }, Deps::none());

        let handle_key_down = use_callback({
            let mut tetris = tetris.clone();

            move |evt: KeyboardEvent| {
                let code = evt.code();

                let direction = match &*code { // convert to &str
                    "ArrowLeft" => Some(Direction::Left),
                    "ArrowRight" => Some(Direction::Right),
                    _ => None,
                };

                if let Some(direction) = direction {
                    tetris.set(|mut tetris| {
                        tetris.shift(direction);
                        tetris
                    });
                }

                if code == "ArrowUp" {
                    tetris.set(|mut tetris| {
                        tetris.rotate();
                        tetris
                    })
                }
        }}, Deps::none());

        h!(div)
            .tabindex(0)
            .on_keydown(&handle_key_down)
            .style(
                &Style::new()
                    .display("inline-grid")
                    .grid_template(format!(
                        "repeat({}, 1em) / repeat({}, 1em)",
                        self.height, self.width))
                    .border("1px solid grey")
                    .outline("none")
            )
            .build(c![
            ..tetris.value().iter_positions().map(|pos| {
                let typ = tetris.value().get(pos);

                h!(div)
                    .style(
                  &Style::new().text_indent("-.2em").margin_top("-.2em")
                ).build(c![typ.unwrap_or_default()])
            })
        ])
    }
}

export_components! { App }
