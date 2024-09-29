// src/app.rs
#![allow(non_snake_case)]
pub mod components;
pub mod db;
pub mod models;
pub mod pages;
pub mod server_functions;
use pages::{HomePage, TeamPage};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/dashboard-app.css"/>
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <link data-trunk rel="tailwind-css" href="/style/input.css" />

        // sets the document title
        <Title text="Full-Stack Dashboard App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <HomePage />
                        }
                    }/>
                    <Route path="/team" view=move || {
                        view! {
                            <TeamPage />
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

// /// Renders the home page of your application.
// #[component]
// fn HomePage() -> impl IntoView {
//     // Creates a reactive value to update the button
//     let (count, set_count) = create_signal(0);
//     let on_click = move |_| set_count.update(|count| *count += 1);

//     view! {
//         <h1>"Welcome to Leptos!"</h1>
//         <button class="bg-red-500 rounded text-white px-2 py-2" 
//             on:click=on_click>"Click Me: " {count}</button>
//     }
// }

/// 404 - Not Found
#[allow(non_snake_case)]
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}