#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use crate::{components::{self, footer, head, navigation}, models::{Section, StrifeSection}};

/// # Sections
/// Rust model used to render the page.
/// Only used when compiled without `--features realtime`, when realtime
/// is used the model is handled in a store in Alpine.js
#[derive(Deserialize, Debug)]
pub struct Sections {
    #[cfg(not(feature = "realtime"))]
    #[serde(rename = "Results")]
    pub results: Vec<SectionsResult>
}

/// # SectionResult
/// List of results from Strife
#[cfg(not(feature = "realtime"))]
#[derive(Deserialize, Debug)]
pub struct SectionsResult {
    pub sections: Vec<Section>
}

impl Sections {
    /// Renders the view in either `realtime`, with Alpine.js store/reactivity,
    /// or straight from the server middleware/extractor.
    pub fn render(self, initial_model: Option<String>) -> Markup {
        #[cfg(feature = "realtime")]
        {
            html! {
                (head(initial_model))
                body class="bg-white flex flex-col" {
                    (navigation())
                    main id="main" class="flex-grow"
                    data-x-data="{ sections: $store?.strife?.sections || [] }"
                    data-x-init=(PreEscaped("
                        $watch('$store.strife', val => {
                            if (val = $store.strife.sections) {
                                sections = val;
                            }
                        });
                    "))
                    {
                        template data-x-if="sections.length" {
                            template data-x-for="section in sections" {
                                div {
                                    template data-x-if="section['@strife'].template === 'hero'" {
                                        (components::Hero::render())
                                    }

                                    template data-x-if="section['@strife'].template === 'content'" {
                                        (components::Content::render())
                                    }

                                    template data-x-if="section['@strife'].template === 'text'" {
                                        (components::Text::render())
                                    }

                                    template data-x-if="section['@strife'].template === 'expandable'" {
                                        (components::Expandable::render())
                                    }
                                    
                                    template data-x-if="section['@strife'].template === 'bento'" {
                                        (components::Bento::render())
                                    }
                                }
                            }
                        }
                    }
                    (footer())
                }
            }
        }
        #[cfg(not(feature = "realtime"))]
        {
            let result = self.results.into_iter().next();
            match result {
                Some(r) => {
                    html! {
                        (head(initial_model))
                        body class="bg-white flex flex-col" {
                            (navigation())
                            main id="main" class="flex-grow" {
                                @for section in r.sections {
                                    @match section.strife {
                                        StrifeSection::Hero(h) => (h.render()),
                                        StrifeSection::Content(c) => (c.render()),
                                        StrifeSection::Text(t) => (t.render()),
                                        StrifeSection::Expandable(e) => (e.render()),
                                        StrifeSection::Bento(b) => (b.render()),
                                        StrifeSection::Unknown => { }
                                    }
                                }
                            }
                            (footer())
                        }
                    }
                },
                None => {
                    html! {
                        h2 { "No CMS data" }
                    }
                }
            }
        }
    }
}