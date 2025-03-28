use maud::{Markup, PreEscaped, html};

/// # Head
/// Partial for `<head>`
/// If compiled with `--features realtime` the head will include
/// various js scripts and imports, and also prepare and hookup
/// Strife subscribe to an Alpine.js store.
/// 
/// Without `realtime` this just adds head metadata and CSS, no js at all.
pub fn head(initial_model: Option<String>) -> Markup {
    html! {
        html lang="en";
        head {
            title { "Strife Demo" }
            meta charset="utf-8";
            meta name="viewport" content="width=device-width, initial-scale=1";
            meta http-equiv="X-UA-COMPATIBLE" content="IE=edge,chrome=1";
            meta name="author" content="Robert Blomqvist";
            link rel="preconnect" href="https://fonts.googleapis.com";
            link rel="preconnect" href="https://fonts.gstatic.com" crossorigin;
            link rel="stylesheet"q href="https://fonts.googleapis.com/css2?family=Manrope:wght@400..700&display=swap";
            link rel="stylesheet" href="/styles/main.css";
            @if cfg!(feature = "realtime") {
                script type="module" {
                    (PreEscaped(r#"
                    import { subscribe } from "https://www.unpkg.com/@strifeapp/strife@latest";

                    window.strife = {};
                    window.strife.subscribe = subscribe;
                    "#));

                    @if let Some(m) = initial_model {
                        "window.initialStore = "(PreEscaped(m))";"
                    }
                }
                script defer src="js/store.js" { }
                script defer src="https://www.unpkg.com/alpinejs@3.x.x/dist/cdn.min.js" { }
            }
        }
    }
}
