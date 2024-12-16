use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <h1>{ "WebAssembly WebGPU Graphic" }</h1>
            <script>
            {
                "\
                    console.log('Hello World');\
                \
                "
            }
            </script>
        </>

    }
}