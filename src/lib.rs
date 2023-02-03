use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {

    fn setup_clicker(document: &Document) {
        let num_clicks = document
            .get_element_by_id("num-clicks")
            .expect("should have #num-clicks on the page");
        let a = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            num_clicks.set_inner_html(&(event.client_x().to_string().to_owned() + ", " + &event.client_y().to_string()));
        });
        body()
            .dyn_ref::<HtmlElement>()
            .expect("body be an `HtmlElement`")
            .add_event_listener_with_callback("mousemove", a.as_ref().unchecked_ref()).expect("set event failed");
    
        // See comments in `setup_clock` above for why we use `a.forget()`.
        a.forget();
    }




    setup_clicker(&document());
    Ok(())
}