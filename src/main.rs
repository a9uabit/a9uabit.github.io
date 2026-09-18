use leptos::{logging, prelude::*};
use leptos_meta::{Title, provide_meta_context};

fn main() {
    console_error_panic_hook::set_once();

    if let Some(document) = window().document()
        && let Some(initial) = document.get_element_by_id("initial")
    {
        initial.remove();
    }

    leptos::mount::mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Home - a9ua" />

        <div>
            <h1>"a9ua"</h1>
            <p>"AKA a9ua_bit, aquaabit"</p>
        </div>
    }
}
