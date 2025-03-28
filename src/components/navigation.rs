use maud::{html, Markup};

/// # Navigation
/// Model-less partial for `<nav>`
pub fn navigation() -> Markup {
    html! {
        div {
            div class="px-8 md:px-12 mx-auto py-4 lg:px-32" {
                div data-x-data="{ open: false }"
                class="relative flex flex-col w-full mx-auto rounded-xl md:rounded-full md:items-center md:justify-between md:flex-row" {
                    div class="flex flex-row items-center justify-between md:justify-start" {
                        a class="text-black hover:text-zinc-500 text-2xl font-bold" href="/" { "Strife Demo "}
                        button data-x-on:click="open = !open"
                        class="inline-flex items-center justify-center p-2 text-black hover:text-blue-500 focus:outline-none focus:text-black md:hidden" {
                            svg  class="w-6 h-6"
                            stroke="currentColor"
                            fill="none"
                            viewBox="0 0 24 24" {
                                path data-x-bind:class="{'hidden': open, 'inline-flex': !open }"
                                  class="inline-flex"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M4 6h16M4 12h16M4 18h16" { }
                                path data-x-bind:class="{'hidden': !open, 'inline-flex': open }"
                                  class="hidden"
                                  stroke-linecap="round"
                                  stroke-linejoin="round"
                                  stroke-width="2"
                                  d="M6 18L18 6M6 6l12 12" { }
                            }
                        }
                    }
                    nav data-x-bind:class="{'flex': open, 'hidden': !open}"
                        class="flex-col flex-grow hidden py-12 md:py-0 md:flex md:items-end justify-center md:flex-row" {
                            ul class="space-y-2 list-none md:space-y-0 md:ml-auto items-center md:inline-flex justify-center text-center md:text-left gap-x-8" {
                                li {
                                    a href="https://strife.app" class="text-black hover:text-emerald-500" { "Sign in" }
                                }
                            }
                        }
                }
            }
        }
    }
}