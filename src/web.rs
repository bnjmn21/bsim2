use bevy::prelude::*;
use web_sys::wasm_bindgen::JsCast;
use web_sys::HtmlCanvasElement;

/// Provides Compatability for the web.
pub struct WebPlugin;

impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, fit_canvas_to_parent);
    }
}

/// Fits the canvas element to the size of the screen.
fn fit_canvas_to_parent() {
    let canvas: HtmlCanvasElement = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("canvas")
        .unwrap()
        .unwrap()
        .unchecked_into();
    let style = canvas.style();
    style.set_property("width", "100%").unwrap();
    style.set_property("height", "100%").unwrap();
}
