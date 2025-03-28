use maud::{html, Markup};

/// # Footer
/// Model-less partial for `<footer>`
pub fn footer() -> Markup {
    html! {
        footer class="bg-zinc-800 text-white antialiased w-full flex items-center py-16 md:py-20 lg:py-28"
        {
            div class="container" {
                div class="w-full grid gap-10 grid-cols-12 auto-cols-max" {
                    div class="col-span-12 lg:col-span-3" {
                        a href="/" class="font-bold text-2xl" { "Strife Demo" }
                    }
                    div class="col-span-12 lg:col-span-3" {
                        h3 class="font-bold text-xl mb-3" { "About "}
                        ul class="space-y-2" {
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                        }
                    }
                    div class="col-span-12 lg:col-span-3" {
                        h3 class="font-bold text-xl mb-3" { "Services" }
                        ul class="space-y-2" {
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                        }
                    }
                    div class="col-span-12 lg:col-span-3" {
                        h3 class="font-bold text-xl mb-3" { "Company" }
                        ul class="space-y-2" {
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                            li {
                                a href="" class="underline underline-offset-4 hover:no-underline" { "Link" }
                            }
                        }
                    }
                }
            }
        }
    }
}
