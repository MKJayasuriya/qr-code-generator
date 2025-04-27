use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::qr;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="container">{children()}</div> }
}

#[component]
pub fn PageHeader() -> impl IntoView {
    view! {
        <div class="page-header">
            <nav class="navbar">
                <div class="navbar-title">Qr Code Generator</div>
                <div class="navbar-links">
                    <ul>
                        <li class="menu-item">
                            <a href="/">"Home"</a>
                        </li>
                        <li class="menu-item">
                            <a href="/generate">"Qr Generator"</a>
                        </li>
                    </ul>
                </div>
            </nav>
        </div>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <main class="content">
        <Router>
            <main>
                <Routes fallback=|| view! { <NotFound/> }>
                    <Route path=path!("") view=HomePage />
                    <Route path=path!("/generate") view=qr::Home />
                </Routes>
            </main>
        </Router>
        </main>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/qr-code-generator.css"/>
        <Title text="Welcome to Leptos"/>
        <Container>
            <PageHeader/>
            <Content/>
        </Container>
    }
}

#[component]
fn NotFound() -> impl IntoView {
    view! {
        <h1>"Not Found"</h1>
    }
}
