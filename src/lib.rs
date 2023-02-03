use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    fn setup_clicker(document: &Document) {
        let num_clicks = document
            .get_element_by_id("num-clicks")
            .expect("should have #num-clicks on the page");
        let a = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            num_clicks.set_inner_html(&(event.client_x().to_string().to_owned() + ", " + &event.client_y().to_string()));
        });
        document.body().expect("body doesn't exist")
            .dyn_ref::<HtmlElement>()
            .expect("body be an `HtmlElement`")
            .add_event_listener_with_callback("mousemove", a.as_ref().unchecked_ref()).expect("set event failed");
    
        // See comments in `setup_clock` above for why we use `a.forget()`.
        a.forget();
    }
    setup_clicker(&document);
    Ok(())
}