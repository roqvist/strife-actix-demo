#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

/// # Bento
/// Model for the `bento` component, when rendered without js
#[derive(Deserialize, Debug)]
pub struct Bento {
    pub style: String,
    pub items: Vec<BentoItem>
}

#[derive(Deserialize, Debug)]
pub struct BentoItem {
    pub link: Option<BentoLink>,
    pub image: Option<BentoImage>,
    pub heading: String,
    pub text: String
}

#[derive(Deserialize, Debug)]
pub struct BentoLink {
    pub href: String
}

#[derive(Deserialize, Debug)]
pub struct BentoImage {
    pub source: BentoImageSource,
    pub alt: String
}

#[derive(Deserialize, Debug)]
pub struct BentoImageSource {
    pub url: String
}

impl Bento {
    /// Render with realtime preview, Alpine.js
    #[cfg(feature = "realtime")]
    pub fn render() -> Markup {
        html! {
            section data-strife-template="bento" class="my-16 md:my-20 lg:my-28" {
                div class="container" {
                    div class="w-full grid grid-cols-12 auto-rows-max gap-5 row-span" {
                        template data-x-for="(item, index) in section?.items" {
                            div data-x-bind:class="{
                                'relative rounded-xl col-span-12 p-10 pt-52': true,
                                'col-span-full': true,
                                'lg:col-span-6': (Number(section?.style) === 1 && [3, 4].includes(index)) || (Number(section?.style) === 2 && index > 3) || (Number(section?.style) === 4 && [2, 3].includes(index)),
                                'lg:col-span-8': (Number(section?.style) === 2 && [1, 2].includes(index)) || (Number(section?.style) === 3 && index === 0),
                                'lg:row-span-2': Number(section?.style) === 3 && index === 1,
                                'lg:col-span-3': Number(section?.style) === 4 && [0, 1, 4, 5].includes(index),
                                'lg:col-span-4': (Number(section?.style) === 3 && index > 0) || (Number(section?.style) === 2 && [0, 3].includes(index)) || (Number(section?.style) === 1 && index < 3)
                                }"
                            {
                                template data-x-if="item.link" {
                                    a data-x-bind:href="item.link?.href"
                                    {
                                        img data-x-bind:src="item.image?.source.url" data-x-bind:alt="item.image?.alt"
                                            class="absolute top-0 left-0 w-full h-full object-cover rounded-xl -z-10";
                                        div class="absolute bottom-0 left-0 right-0 top-0 bg-zinc-900/75 rounded-xl -z-10" { }
                                        h3 class="text-2xl font-bold mb-3 text-white" data-x-text="item.heading" { }
                                        p class="text-white" data-x-text="item.text" { }
                                    }
                                }
                                template data-x-if="!item.link" {
                                    div {
                                        img data-x-bind:src="item.image?.source.url" data-x-bind:alt="item.image?.alt"
                                            class="absolute top-0 left-0 w-full h-full object-cover rounded-xl -z-10";
                                        div class="absolute bottom-0 left-0 right-0 top-0 bg-zinc-900/75 rounded-xl -z-10" { }
                                        h3 class="text-2xl font-bold mb-3 text-white" data-x-text="item.heading" { }
                                        p class="text-white" data-x-text="item.text" { }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    /// Render without realtime preview and js
    #[cfg(not(feature = "realtime"))]
    pub fn render(self) -> Markup {
        let style: i32 = self.style.parse().unwrap_or(0);
        let original_item_class = "relative rounded-xl col-span-12 p-10 pt-52 col-span-full".to_owned();
        html! {
            section data-strife-template="bento" class="my-16 md:my-20 lg:my-28"
            {
                div class="container" {
                    div class="w-full grid grid-cols-12 auto-rows-max gap-5 row-span"
                    {
                        @for (i, item) in self.items.into_iter().enumerate() {
                            @let mut item_class = original_item_class.clone();
                            @if (style == 1 && [3,4].contains(&i)) || (style == 2 && i > 3) || (style == 4 && [2, 3].contains(&i)) {
                                @let _ = {
                                    item_class.push_str(" lg:col-span-6");
                                };
                            }
                            @if (style == 2 && [1, 2].contains(&i)) || (style == 3 && i == 0) {
                                @let _ = {
                                    item_class.push_str(" lg:col-span-8");
                                };
                            }
                            @if style == 3 && i == 1 {
                                @let _ = {
                                    item_class.push_str(" lg:row-span-2");
                                };
                            }
                            @if style == 4 && [0, 1, 4, 5].contains(&1) {
                                @let _ = {
                                    item_class.push_str(" lg:col-span-3");
                                };
                            }
                            @if (style == 3 && i > 0) || (style == 2 && [0, 3].contains(&i)) || (style == 1 && i < 3) {
                                @let _ = {
                                    item_class.push_str(" lg:col-span-4");
                                };
                            }
                            @if let Some(link) = item.link {
                                a href=(link.href) class=(item_class) {
                                    @if let Some(image) = item.image {
                                        img src=(image.source.url) alt=(image.alt)
                                            class="absolute top-0 left-0 w-full h-full object-cover rounded-xl -z-10";
                                        div class="absolute bottom-0 left-0 right-0 top-0 bg-zinc-900/75 rounded-xl -z-10" { }
                                        h3 class="text-2xl font-bold mb-3 text-white" { (item.heading) }
                                        p class="text-white" { (item.text) }
                                    }
                                }
                            } @else {
                                div class=(item_class) {
                                    @if let Some(image) = item.image {
                                        img src=(image.source.url) alt=(image.alt)
                                            class="absolute top-0 left-0 w-full h-full object-cover rounded-xl -z-10";
                                        div class="absolute bottom-0 left-0 right-0 top-0 bg-zinc-900/75 rounded-xl -z-10" { }
                                        h3 class="text-2xl font-bold mb-3 text-white" { (item.heading) }
                                        p class="text-white" { (item.text) }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}