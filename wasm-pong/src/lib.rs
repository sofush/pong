use std::{cell::RefCell, rc::Rc};

use game_state::GameState;
use gloo::events::EventListener;
use log::Level;
use paddle::Direction;
use screen::Screen;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

mod ball;
mod bounding_box;
mod game_state;
mod paddle;
mod screen;
mod vec2;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn canvas() -> (HtmlCanvasElement, web_sys::CanvasRenderingContext2d) {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    (canvas, context)
}

#[wasm_bindgen(start)]
fn run() -> Result<(), JsValue> {
    console_log::init_with_level(Level::Info).unwrap();

    let (canvas, _) = canvas();
    let screen = Screen::new(
        canvas.offset_width().into(),
        canvas.offset_height().into(),
    );
    let state = Rc::new(RefCell::new(GameState::new(screen)));
    let state_clone = Rc::clone(&state);
    let render_ref = Rc::new(RefCell::new(None));
    let render_ref_clone = render_ref.clone();

    EventListener::new(&window(), "keydown", move |e| {
        let event = e.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
        log::info!("{:?}", event.key());
        let _ = state_clone
            .borrow_mut()
            .left_paddle_mut()
            .change_direction(Direction::Down);
        let _ = state_clone
            .borrow_mut()
            .right_paddle_mut()
            .change_direction(Direction::Down);
    })
    .forget();

    *render_ref.borrow_mut() = Some(Closure::new(move || {
        let screen = Screen::new(
            canvas.offset_width().into(),
            canvas.offset_height().into(),
        );

        canvas.set_height(screen.height() as u32);
        canvas.set_width(screen.width() as u32);

        state.borrow_mut().update(screen);
        render(&mut *state.borrow_mut());

        request_animation_frame(render_ref_clone.borrow().as_ref().unwrap());
    }));

    request_animation_frame(render_ref.borrow().as_ref().unwrap());
    Ok(())
}

fn render(state: &mut GameState) {
    log::debug!("Starting render...");

    let (_canvas, context) = canvas();
    state.ball().draw(state.screen(), &context);
    state.left_paddle().draw(state.screen(), &context);
    state.right_paddle().draw(state.screen(), &context);

    log::debug!("Render finished");
}
