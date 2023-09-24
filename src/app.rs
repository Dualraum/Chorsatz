use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    let styler_class = stylers::style! { "App",
        div.one{
            color: red;
            content: raw_str(r#"\hello"#);
            font: "1.3em/1.2" Arial, Helvetica, sans-serif;
        }
        div {
            border: 1px solid black;
            margin: 25px 50px 75px 100px;
            background-color: lightblue;
        }
        h2 {
            color: purple;
        }
        @media only screen and (max-width: 1000px) {
            h3 {
                background-color: lightblue;
                color: blue
            }
        }
    };

    view! { class = styler_class,
    <div class="one">
        <h1 id="two">"Hello"</h1>
        <h2>"World"</h2>
        <h2>"and"</h2>
        <h3>"friends!"</h3>
    </div>
    }

    // view! { class = styler_class,
    //     <h2 class="my_head">"Chorsatz"</h2>
    //     <p>"Chorsatz has arrived! - Chorsatz"</p>
    //     <input type="text"></input>
    // }
}
