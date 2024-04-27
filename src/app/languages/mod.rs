/// Supported languages
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Language {
    English,
    German,
}

pub fn get_string_set(language: Language) -> LanguageStringSet {
    match language {
        Language::English => ENGLISH_STRINGS,
        Language::German => GERMAN_STRINGS,
    }
}

const GERMAN_STRINGS: LanguageStringSet = LanguageStringSet {
    intro: "Chorsatz ist eine Webapplikation zur automatischen Erstellung von vierstimmigen SATB-Stimmsätzen aus einer Reihenfolge von vorgegebenen Akkorden unter Beachtung der klassischen Stimmführungsregeln.",
    input_default: "Akkorde hier eingeben...",
    generate: "Generieren",
    options: "Optionen",
    help: "Eingabehilfe",
    options_content: [
        "Auswahlkriterien",
        "Maximale Stimmdifferenz Sopran-Alt"	,
        "Maximale Stimmdifferenz Alt-Tenor",
        "Maximale Stimmdifferenz Tenor-Bass	"    ,
        "Maximale Stimmdifferenz Sopran-Bass"    ,
        "Stimmkreuzungen erlauben"    ,
        "Gleiche Töne in Bass & Tenor erlauben"    ,
        "Aufeinanderfolgende gleiche Töne liegen lassen"    ,
        "Gegenbewegung zum Bass forcieren"    ,
        "Verbotene Parallelen"    ,
        "Maximale Sprungweite im Sopran	"    ,
        "Maximale Sprungweite im Alt"    ,
        "Maximale Sprungweite im Tenor	"    ,
        "Maximale Sprungweite im Bass"    ,
        "Ordnungskriterien"    ,
        "Relatives Gewicht geringe Bewegungen	"    ,
        "Relatives Gewicht Gegenbewegungen"    ,
        "Relatives Gewicht enge Lage"    ,
        "Malus für hohen Startton im Sopran	"    ,
        "Grenzton für hohen Startton im Sopran (englisches Format)"    ,
        "Malus für hohen Startton im Bass	"    ,
        "Grenzton für hohen Startton im Bass (englisches Format)",
    ],
    results_title: "Ergebnisse",
    results_algo_explation: "Eine kleinere Bewertung kennzeichnet eine optimalere Lösung. Da eine rein algorithmische Bewertung nicht perfekt ist, werden mehrere Ergebnisse zur Auswahl angezeigt.",
    results_total: |shown, total | format!("Es werden die besten {shown} Ergebnisse aus {total} berechneten Lösungen angezeigt."),
    result_title: "Ergebnis",
    result_play: "Abspielen",
    result_table: "Notentabelle",
    table_chord: "Akkord",
    table_voices: ["Sopran", "Alt", "Tenor", "Bass"],
    score_tooltip: "Bewertung dieses Ergebnisses berechnet aus der relativen Notenlage. Kleine Bewertungen kennzeichnen bessere Lösungen.",
    more: "Mehr Ergebnisse anzeigen...",
    authors: "Autoren",
    thanks: "Mit Dank an",
};

const ENGLISH_STRINGS: LanguageStringSet = LanguageStringSet {
    intro: "Chorsatz is a web application to automatically create four-voiced SATB-sheets from a series of chords, following classical rules and preferences.",
    input_default: "Enter chords...",
    generate: "Generate",
    options: "Options",
    help: "Help",
    options_content: [
        "Selection Criteria",
        "Maximum soprano-alto difference",
        "Maximum alto-tenor difference",
        "Maximum tenor-bass difference"    ,
        "Maximum soprano-bass difference"    ,
        "Allow voices to cross"    ,
        "Allow duplicate notes in alto and tenor"    ,
        "Let notes lie if consecutive and equal"    ,
        "Force countermovement to bass"    ,
        "Forbidden parallels"    ,
        "Maximum soprano jump"    ,
        "Maximum alto jump"    ,
        "Maximum tenor jump"    ,
        "Maximum bass jump"    ,
        "Scoring criteria"    ,
        "Relative weight of small movements"    ,
        "Relative weight countermovements"    ,
        "Relative weight of close notes"    ,
        "Malus for high initial soprano"    ,
        "Threshhold for high soprano"    ,
        "Malus for high initial bass"    ,
        "Threshhold for high bass"    ,
    ],
    results_title: "Results",
    results_algo_explation: "A lower score indicates a better result. As algorithmic ranking is not perfect, multiple results are shown.",
    results_total: |shown, total | format!("Showing the best {shown} out of {total} total results."),
    result_title: "Result",
    result_play: "Play",
    result_table: "Note Table",
    table_chord: "Chord",
    table_voices: ["Soprano", "Alto", "Tenor", "Bass"],
    score_tooltip: "The score of this result is based on the relative position of its notes. Smaller scores indicate a better result.",
    more: "Show more results...",
    authors: "Authors",
    thanks: "Thanks to",
};

pub struct LanguageStringSet {
    pub intro: &'static str,

    pub input_default: &'static str,

    pub generate: &'static str,
    pub options: &'static str,
    pub help: &'static str,

    pub options_content: [&'static str; 22],

    pub results_title: &'static str,
    pub results_algo_explation: &'static str,
    pub results_total: fn(usize, usize) -> String,

    pub result_title: &'static str,
    pub result_play: &'static str,
    pub result_table: &'static str,
    pub table_chord: &'static str,
    pub table_voices: [&'static str; 4],
    pub score_tooltip: &'static str,

    pub more: &'static str,
    pub authors: &'static str,
    pub thanks: &'static str,
}
