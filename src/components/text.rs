#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

/// # Text
/// Model for the `text` component, when rendered without js
#[derive(Deserialize, Debug)]
pub struct Text {
    pub background: Option<String>,
    pub align: Option<String>,
    #[serde(alias = "content")]
    pub text: Option<String>
}

impl Text {
    /// Render with realtime preview, Alpine.js
    #[cfg(feature = "realtime")]
    pub fn render() -> Markup {
        html! {
            template data-strife-template="text" data-x-if="section?.text || section?.content" {
                section data-x-bind:class="{
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
                        div data-x-bind:class="{
                            'prose': true,
                            'text-center': section?.align === 'center'
                        }"
                        data-x-html="section?.content"
                        {
                            "Hello"
                        }
                    }
                }
            }
        }
    }
    /// Render without realtime preview and js
    #[cfg(not(feature = "realtime"))]
    pub fn render(self) -> Markup {
        html! {
            div {
                section data-strife-template="text" class={
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
                            div class={
                                "prose"
                                @if let Some(align) = self.align {
                                    @if align == "center" {
                                        " text-center"
                                    }
                                }
                            }
                            {
                                (PreEscaped(text))
                            }
                        }
                    }
                }
            }
        }
    }
}