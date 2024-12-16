use gloo::utils::document;
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::{js_sys::Function, HtmlElement};
use yew::prelude::*;

/* fn submit_handler(elem: HtmlElement) {
    let drag_start_binding = Closure::wrap(Box::new(|event: DragEvent| {
        event.prevent_default();
    let elem = event
        .target()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    elem.set_class_name(format!("{} dragging", elem.class_name()).as_str());
    }) as Box<dyn FnMut(DragEvent)>);
    let drag_start = drag_start_binding
        .as_ref()
        .dyn_ref::<Function>()
        .unwrap();
    let drag_end_binding = Closure::wrap(Box::new(|event: DragEvent| {
        event.prevent_default();
        let elem = event
            .target()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap();
        elem.set_class_name(elem.class_name().replace("dragging", "").as_str());
    }) as Box<dyn FnMut(DragEvent)>);
    let drag_end = drag_end_binding
            .as_ref()
            .dyn_ref::<Function>()
            .unwrap();
    let _ = elem.add_event_listener_with_callback("dragstart", drag_start);
    let _ = elem.add_event_listener_with_callback("dragend", drag_end);
} */

#[function_component(MouseFields)]
pub fn drag_drop_fields() -> Html {

    /* use_effect(|| {
        let items = document().query_selector_all(".item").unwrap();
        let _ = items.for_each(Closure::wrap(Box::new(
            submit_handler
        ) as Box<dyn FnMut(HtmlElement)>).as_ref().dyn_ref::<Function>().unwrap());
    }); */

    let buttons = vec!["box","triangle","rainbow_triangle"];
    let button_html = buttons.iter().map(move |btn_text| html! {
        <div class="item" draggable="true">{format!("{}",btn_text)}</div>
    }).collect::<Html>();


    html! {
        <div class="box">
            <div class="unrender">
                {button_html}
            </div>
            <div class="render">
            
            </div>
        </div>
    }
}
