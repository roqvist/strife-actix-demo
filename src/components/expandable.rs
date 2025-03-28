#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

/// # Expandable
/// Model for the `expandable` component, when rendered without js
#[derive(Deserialize, Debug)]
pub struct Expandable {
    pub background: Option<String>,
    pub text: Option<String>
}

impl Expandable {
    /// Render with realtime preview, Alpine.js
    #[cfg(feature = "realtime")]
    pub fn render() -> Markup {
        html! {
            section data-strife-template="expandable" data-x-bind:class="{
                'py-16': section?.background !== '0',
                'md:py-20': section?.background !== '0',
                'lg:py-28': section?.background !== '0',
                'bg-zinc-200': section?.background !== '0',
                'my-16': section?.background !== '0',
                'md:my-20': section?.background !== '0',
                'lg:my-28': section?.background !== '0',
                'my-16': section?.background === '0',
                'md:my-20': section?.background === '0',
                'lg:my-28': section?.background === '0'
            }"
            {
                div class="container" {
                    div class="w-full prose" data-x-html="section?.text" {

                    }
                }
            }
        }
    }
    /// Render without realtime preview and js
    #[cfg(not(feature = "realtime"))]
    pub fn render(self) -> Markup {
        html! {
            section data-strife-template="expandable" class={
                @if let Some(background) = self.background {
                    @if background != "0" {
                        "py-16 md:py-20 lg:py-28 bg-zinc-200 my-16 md:my-20 lg:my-28"
                    } @else {
                        "my-16 md:my-20 lg:my-28"
                    }
                } @else {
                    "my-16 md:my-20 lg:my-28"
                }
            }
            {
                div class="container" {
                    @if let Some(text) = self.text {
                        div class="w-full prose" {
                            (PreEscaped(text))
                        }
                    }
                }
            }
        }
    }
}