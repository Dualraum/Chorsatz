use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    //let styler_class = stylers::style_sheet! {"./src/app/app.css"};

    //t

    view! { //class = styler_class,
        <h1>"Chorsatz"</h1>
        <div class = "visible">
            <div>
                <p> "Chorsatz ist eine Webapplikation zur automatischen Erstellung von vierstimmigen SATB-Stimmsätzen aus einer Reihenfolge von vorgegebenen Akkorden unter Beachtung der klassischen Stimmführungsregeln." </p>
            </div>
        </div>

        <div class="visible">
            <div>
                <h2>Eingabe:</h2>
                <input type="text" placeholder="Akkorde hier eingeben..."></input>
            </div>
        </div>
    }
}
