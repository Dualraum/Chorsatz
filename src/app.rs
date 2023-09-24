use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = stylers::style_sheet! {"./src/app/app.css"};

    view! { class = styler_class,
        <h1>"Chorsatz"</h1>
        <div>
            <p>"Chorsatz has arrived! - Chorsatz"</p>
        </div>
        <input type="text"></input>
    }
}
