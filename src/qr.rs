use leptos::prelude::*;
use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct GenerateInput {
    pub input: String,
}

#[server(Generate, "/api/generate")]
pub async fn generate(input: GenerateInput) -> Result<String, ServerFnError> {
    println!("Received input: {:?}", input);
    Ok(format!("Received: {}", input.input))
}

#[component]
pub fn QrHeader(header_text: String) -> impl IntoView {
    view! {
        <header>
            <h1>{header_text}</h1>
        </header>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    // This creates the action bound to the `generate` function
    let generate = create_server_action::<Generate>();

    view! {
        <QrHeader header_text="Qr gen".to_string() />
        <QrForm generate />
    }
}

#[component]
pub fn QrForm(generate: ServerAction<Generate>) -> impl IntoView {
    let submission = generate.input();
    let response = generate.value();

    view! {
        <div>
            <ActionForm action=generate>
                <input type="text" name="input" />
                <button type="submit">"Generate"</button>
            </ActionForm>

            <Show
                when=move || response().is_some()
                fallback=|| view! { <p>Submit something to generate.</p> }
            >
                {move || match response() {
                    Some(Ok(message)) => view! { <p>{message}</p> },
                    Some(Err(e)) => view! { <p>"Error: "{e.to_string()}</p> },
                    None => view! { <p>"Loading..."</p> },
                }}
            </Show>
        </div>
    }
}
