#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{Markup, PreEscaped, html};
use serde::Deserialize;

use super::{Image, Video};

/// # Content
/// Model for the `content` component, when rendered without js
#[derive(Deserialize, Debug)]
pub struct Content {
    pub text: String,
    pub background: Option<String>,
    #[serde(rename = "imageAlign", default)]
    pub image_align: String,
    pub video: Option<Video>,
    pub image: Option<Image>,
}

impl Content {
    /// Render with realtime preview, Alpine.js
    #[cfg(feature = "realtime")]
    pub fn render() -> Markup {
        html! {
            section data-strife-template="content" data-x-bind:class="{
                'py-16': section?.background != '0',
                'md:py-20': section?.background != '0',
                'lg:py-28': section?.background != '0',
                'bg-zinc-200': section?.background != '0',
                'my-16': !section?.background == '0',
                'md:my-20': !section.background == '0',
                'lg:my-28': !section.background == '0'
            }"
            {
                div class="container" {
                    div class="grid grid-cols-1 lg:grid-cols-2 gap-y-6 gap-x-10 xl:gap-16 items-center" {
                        div data-x-bind:class="{
                            'w-full': true,
                            'lg:order-2': section?.imageAlign === 'right'
                        }"
                        {
                            template data-x-if="!section?.video?.__media" {
                                picture {
                                    source data-x-bind:srcset="section?.image?.mobile?.source?.url"
                                    media="(max-width: 500px)" width="600" height="300" data-property-media="mobile";

                                    source data-x-bind:srcset="section?.image?.tablet?.source?.url"
                                    media="(min-width: 501px) and (max-width: 861px)" width="728" height="364" data-property-media="tablet";

                                    source data-x-bind:srcset="section?.image?.desktop?.source?.url"
                                    media="(min-width: 862px)" width="716" height="537" data-property-media="desktop";

                                    img data-x-bind:src="section?.image?.desktop?.source?.url"
                                    loading="lazy"
                                    alt=""
                                    width="716"
                                    height="537"
                                    class="w-full block rounded-2xl aspect-2/1 lg:aspect-4/3 object-cover";
                                }
                            }

                            template data-x-if="section?.video?.__media" {
                                video class="w-full block rounded-2xl object-cover aspect-2/1 lg:aspect-4/3"
                                playsinline
                                muted
                                data-x-bind:controls="section?.video?.controls"
                                data-x-bind:loop="section?.video?.loop"
                                data-x-bind:autoplay="section?.video?.playing"
                                {
                                    source data-x-bind:src="section?.video?.url + '#t=' + section?.video?.startTime" type="video/mp4";
                                }
                            }
                        }
                        div data-x-bind:class="{
                            'w-full': true,
                            'prose': true,
                            'lg:prose-lg': true,
                            'lg:order-1': section?.imageAlign === 'right'
                            }"
                            data-placeholder="Här kan vi skriva en placeholder-text"
                            data-x-html="section?.text"
                        {

                        }
                    }
                }
            }
        }
    }
    /// Render without realtime preview and js
    #[cfg(not(feature = "realtime"))]
    pub fn render(self) -> Markup {
        use crate::components::should_render_video;

        html! {
            section data-strife-template="content" class={
                @if let Some(background) = self.background {
                    @if background == "1" {
                        "py-16 md:py-20 lg:py-28 bg-zinc-200"
                    } @else {
                        "my-16 md:my-20 lg:my-28"
                    }
                } @else {
                    "my-16 md:my-20 lg:my-28"
                }
            }
            {
                div class="container" {
                    div class="grid grid-cols-1 lg:grid-cols-2 gap-y-6 gap-x-10 xl:gap-16 items-center" {
                        div class={
                            "w-full"
                            @if self.image_align == "right" {
                                " lg:order-2"
                            }
                        }
                        {

                            @if should_render_video(&self.video) {
                                @if let Some(video) = self.video {
                                    @if let Some(url) = video.url {
                                        @let poster_url = video.poster.and_then(|p| p.source.and_then(|s| Some(s.url))).unwrap_or("".into());
                                        video class="w-full block rounded-2xl object-cover aspect-2/1 lg:aspect-4/3"
                                        playsinline
                                        muted
                                        controls[video.controls]
                                        loop[video.is_loop]
                                        autoplay[video.playing]
                                        poster=(poster_url)
                                        {
                                            source src=(format!("{}#t={}", url, video.start_time)) type="video/mp4";
                                        }
                                    }
                                }
                            } @else if let Some(image) = self.image {
                                picture {
                                    @if let Some(mobile) = image.mobile {
                                        @if let Some(source) = mobile.source {
                                            @if let Some(url) = source.url {
                                                source srcset=(url)
                                                media="(max-width: 500px)" width="600" height="300" data-property-media="mobile";
                                            }
                                        }
                                    }

                                    @if let Some(tablet) = image.tablet {
                                        @if let Some(source) = tablet.source {
                                            @if let Some(url) = source.url {
                                                source srcset=(url)
                                                media="(min-width: 501px)and (max-width: 861px)" width="728" height="364" data-property-media="tablet";
                                            }
                                        }
                                    }

                                    @if let Some(desktop) = image.desktop {
                                        @if let Some(source) = desktop.source {
                                            @if let Some(url) = source.url {
                                                source srcset=(url)
                                                media="(min-width: 862px)" width="716" height="537" data-property-media="desktop";

                                                img src=(url)
                                                loading="lazy"
                                                alt=""
                                                width="716"
                                                height="537"
                                                class="w-full block rounded-2xl aspect-2/1 lg:aspect-4/3 object-cover";
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div data-placeholder="Här kan vi skriva en placeholder-text"
                            class={
                                "w-full prose lg:prose-lg"
                                @if self.image_align == "right" {
                                    " lg:order-1"
                                }
                            }
                        {
                            (PreEscaped(self.text))
                        }
                    }
                }
            }
        }
    }
}
