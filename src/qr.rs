use leptos::{logging, prelude::*};

#[server]
async fn generate(input: String) -> Result<String, ServerFnError> {
    use base64::engine::general_purpose::STANDARD as base64_engine;
    use base64::Engine as _;
    use qrcode_generator::QrCodeEcc;

    // Generate SVG
    let svg_string =
        qrcode_generator::to_svg_to_string(&input, QrCodeEcc::Low, 200, None::<&str>).unwrap();

    // Convert SVG into base64
    let svg_bytes = svg_string.into_bytes();
    let base64_svg = base64_engine.encode(svg_bytes);

    // Return as a data URL
    Ok(format!("data:image/svg+xml;base64,{}", base64_svg))
}

#[component]
pub fn QrHeader(header_text: String) -> impl IntoView {
    view! {
        <header>
            <h1>{header_text}</h1>
        </header>
    }
}

/// Renders the home page of your application.
#[component]
pub fn Home() -> impl IntoView {
    let generate = ServerAction::<Generate>::new();
    view! {
        <QrHeader header_text="Qr gen".to_string() />
        <QrForm generate />
    }
}
#[component]
pub fn QrForm(generate: ServerAction<Generate>) -> impl IntoView {
    let response = generate.value();

    view! {
        <h1>"Test the action form!"</h1>
        <ErrorBoundary fallback=move |error| {
            format!("{:#?}", error.get())
        }>
            <div>
                // Success state
                <Show when=move || response.get().map(|res| res.is_ok()) == Some(true)>
                    {move || {
                        let data_url = response.get().unwrap().unwrap();
                        view! {
                            <div>
                                <img src={data_url} alt="QR Code" />
                            </div>
                        }
                    }}
                </Show>

                // Error state
                <Show when=move || response.get().map(|res| res.is_err()) == Some(true)>
                    {move || {
                        let err = response.get().unwrap().unwrap_err();
                        view! {
                            <div>
                                <p style="color: red;">"Error: " {err.to_string()}</p>
                            </div>
                        }
                    }}
                </Show>

                // Loading state
                <Show when=move || response.get().is_none()>
                    <div>
                        <p>"Generating QR code..."</p>
                    </div>
                </Show>
            </div>

            <ActionForm action=generate attr:class="form">
                <label>"Input: "<input type="text" name="input"/></label>
                <button type="submit">Submit</button>
            </ActionForm>
        </ErrorBoundary>
    }
}
