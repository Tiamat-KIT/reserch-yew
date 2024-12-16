use yew::prelude::*;
use crate::component::hello::App;
use crate::component::drag_drop_fields::MouseFields;

#[function_component(Roots)]
pub fn all_component() -> Html {

    html! {
        <>
            <App />
            <div >
                <canvas />
            </div>
            <MouseFields />
        </>
    }
}

