use leptos::{logging, prelude::*};
use leptos_meta::{Stylesheet, Title, provide_meta_context};
use leptos_router::{components::{A, Router}, hooks::use_query_map};

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

    let query = || use_query_map();
    let page = move || query().read().get("page").unwrap_or_else(|| "".into());

    view! {
        <Stylesheet href="/styles.css" />

        <Router>
            <nav>
                <A href="/">"Home"</A>
                <A href="/?page=about">"About"</A>
            </nav>

            {move || match page().as_str() {
                "about" => About().into_any(),
                "" => Home().into_any(),
                _ => view! { <h1>Not Found</h1> }.into_any(),
            }}
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Home - a9ua" />

        <div class="home">
            <img src="/icon.png" />
            <div class="home-name">
                <h1>"a9ua"</h1>
                <p>"AKA a9ua_bit, aquaabit"</p>
            </div>
        </div>
    }
}

#[component]
fn About() -> impl IntoView {
    view! {
        <h1>"Hi, I'm a9ua!"</h1>
    }
}
