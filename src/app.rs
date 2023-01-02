use crate::dark_mode::{DarkModeToggle, DarkModeToggleProps, ToggleDarkMode};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Helper to register all our server functions, if we're in SSR mode
#[cfg(feature = "ssr")]
pub fn register_server_functions() {
    _ = ToggleDarkMode::register();
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(cx, 0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        cx,

        <Router>
            // injects a stylesheet into the document <head>
            // id=leptos means cargo-leptos will hot-reload this stylesheet
            <Stylesheet id="leptos" href="/style.css"/>

            // sets the document title
            <Title text="Welcome to Leptos"/>

            // content for this welcome page
            <main>
                <DarkModeToggle/>
                <h1>"Welcome to Leptos!"</h1>
                <button on:click=on_click>"Click Me: " {count}</button>
            </main>
        </Router>
    }
}
