#![allow(unused_imports)]
#![allow(dead_code)]
use maud::{html, Markup, PreEscaped};
use serde::Deserialize;

use super::{Image, Video};

/// # Hero
/// Model for the `hero` component, when rendered without js
#[derive(Deserialize, Debug)]
pub struct Hero {
    pub heading: String,
    pub text: String,
    pub cta: Option<Cta>,
    pub video: Option<Video>,
    pub image: Option<Image>,
    pub align: Option<String>,
    pub size: Option<String>
}

#[derive(Deserialize, Debug)]
pub struct Cta {
    pub text: String,
    pub href: String,
    pub target: String,
    pub source: String    
}

impl Hero {
    /// Render with realtime preview, Alpine.js
    #[cfg(feature = "realtime")]
    pub fn render() -> Markup {
        html! {
            div class="flex" {
                section data-strife-template="hero" data-x-bind:class="{
                    'relative': true,
                    'text-white': true,
                    'antialiased': true,
                    'h-hero': section?.size === 'small',
                    'h-screen': section?.size === 'large',
                    'max-h-[900px]': section?.size === 'large'
                }" {
                    div class="absolute inset-0 z-20 bg-zinc-900/75 flex justify-center flex-col" {
                        div data-x-bind:class="{
                        'container': true,
                        'mx-auto': true,
                        'flex': true,
                        'flex-col': true,
                        'text-center': section?.align === 'center',
                        'items-center': section?.align === 'center'
                        }" {
                            h1 class="text-2xl md:text-4xl lg:text-7xl font-bold mb-2 max-w-4xl"
                            data-x-text="section?.heading" { }

                            p class="max-w-2xl text-lg mb-7" data-x-text="section?.text" { }
                        
                            template data-x-if="section?.cta?.text && section?.cta?.href" {
                                p class="max-w-2xl" {
                                    a href="section?.cta?.href"
                                    class="font-bold underline underline-offset-4 hover:no-underline"
                                    target="section?.cta?.target"
                                    data-x-text="section?.cta?.text" { }
                                }
                            }
                        }
                    }
                    
                    template data-x-if="!section?.video?.__media" {
                        picture {
                            source data-x-bind:srcset="section?.image?.mobile?.source?.url"
                            media="(max-width: 400px)" width="400" height="500" data-property-media="mobile";

                            source data-x-bind:srcset="section?.image?.tablet?.source?.url"
                            media="(min-width: 401px) and (max-width: 860px)" width="860" height="500" data-property-media="tablet";

                            source data-x-bind:srcset="section?.image?.desktop?.source?.url"
                            media="(min-width: 861px)" width="1366" height="500" data-property-media="desktop";
                            
                            img data-x-bind:src="section?.image?.desktop?.source?.url"
                            loading="lazy"
                            alt=""
                            width="1366"
                            height="500"
                            class="w-full h-full object-cover";
                        }
                    }

                    template data-x-if="section?.video?.__media" {
                        video class="w-full object-cover"
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
            }
        }
    }
    /// Render without realtime preview and js
    #[cfg(not(feature = "realtime"))]
    pub fn render(self) -> Markup {
        use crate::components::should_render_video;

        html! {
            div class="flex" {
                section data-strife-template="hero" class={
                    "relative text-white antialiased"
                    @if let Some(size) = self.size {
                        @if size == "small" {
                            " h-hero"
                        } @else if size == "large" {
                            " h-screen max-h-[900px]"
                        }
                    }
                } {
                    div class="absolute inset-0 z-20 bg-zinc-900/75 flex justify-center flex-col" {
                        div class={
                            "container mx-auto flex flex-col"
                            @if let Some(align) = self.align {
                                @if align == "center" {
                                    " text-center items-center"
                                }
                            }
                        } {
                            h1 class="text-2xl md:text-4xl lg:text-7xl font-bold mb-2 max-w-4xl leading-28" { (self.heading) }
                            
                            p class="max-w-2xl text-lg mb-7" { (self.text) }
                            
                            @if let Some(cta) = self.cta {
                                @if cta.text != "" && cta.href != "" {
                                    p class="max-w-2xl" {
                                        a href=(cta.href)
                                        class="font-bold underline underline-offset-4 hover:no-underline"
                                        target=(cta.target) {
                                            (cta.text)
                                        }
                                    }
                                }
                            }
                        }
                    }

                    @if should_render_video(&self.video) {
                        @if let Some(video) = self.video {
                            @if let Some(url) = video.url {
                                @let poster_url = video.poster.and_then(|p| p.source.and_then(|s| Some(s.url))).unwrap_or("".into());
                                video class="w-full h-full object-cover"
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
                                        media="(max-width: 400px)" width="400" height="500" data-property-media="mobile";
                                    }
                                }
                            }

                            @if let Some(tablet) = image.tablet {
                                @if let Some(source) = tablet.source {
                                    @if let Some(url) = source.url {
                                        source srcset=(url)
                                        media="(min-width: 401px)and (max-width: 860px)" width="860" height="500" data-property-media="tablet";
                                    }
                                }
                            }

                            @if let Some(desktop) = image.desktop {
                                @if let Some(source) = desktop.source {
                                    @if let Some(url) = source.url {
                                        source srcset=(url)
                                        media="(min-width: 861px)" width="1366" height="500" data-property-media="desktop";

                                        img src=(url)
                                        loading="lazy"
                                        width="1366"
                                        height="500"
                                        alt=""
                                        class="w-full h-full object-cover";
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