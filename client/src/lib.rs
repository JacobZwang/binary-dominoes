mod utils;

use std::fmt;
use std::{cell::RefCell, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use web_sys::console;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug)]
#[repr(u8)]
enum DominoValue {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct Domino {
    top: DominoValue,
    bottom: DominoValue,
}

#[wasm_bindgen]
pub struct Game {
    dominoes: Vec<Domino>,
    ctx: web_sys::CanvasRenderingContext2d,
    mouse_x: Rc<RefCell<i32>>,
    mouse_y: Rc<RefCell<i32>>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for domino in self.dominoes.as_slice() {
            write!(f, "{:?}", domino)?;
        }

        Ok(())
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new() -> Game {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        // canvas.

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mouse_x = Rc::new(RefCell::new(0.to_owned()));
        let mouse_x_copy = mouse_x.clone();

        let mouse_y = Rc::new(RefCell::new(0.to_owned()));
        let mouse_y_copy = mouse_y.clone();

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            *mouse_x_copy.borrow_mut() = event.offset_x().to_owned();
            *mouse_y_copy.borrow_mut() = event.offset_y().to_owned();

            unsafe {
                console::log_1(&"Hello using web-sys".into());
            };
        }) as Box<dyn FnMut(_)>);

        canvas
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        Game {
            dominoes: (0..10)
                .map(|_| Domino {
                    top: DominoValue::One,
                    bottom: DominoValue::Five,
                })
                .collect(),
            ctx: context,
            mouse_x: mouse_x,
            mouse_y: mouse_y,
        }
    }

    pub fn render(&self) -> () {
        for domino in self.dominoes.as_slice() {
            self.ctx.rect(*self.mouse_x.borrow() as f64, *self.mouse_y.borrow() as f64, 10.0, 10.0);
            self.ctx.stroke();
        }
    }
}
