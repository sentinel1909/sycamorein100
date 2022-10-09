// home.rs

use sycamore::prelude::*;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::domain::data::get_posts;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    let posts = get_posts();
    let latest_posts = View::new_fragment(
        posts
            .iter()
            .map(|&x| view! { cx, article { p { (x) } }})
            .collect(),
    );
    view! { cx,
        Header {}
        main {
            section{
                h1 { "Greetings, my name is Jeff" }
                p { "Welcome to the reflection of me." }
                p { "I'm a Certified Professional and Fire Engineer by day, but turn into a self-taught, fledgling developer and IT junkie by night...can also be dangerous with a camera. "}
                p { "This site serves as my portfolio of projects, writings, and photography"}
                h2 { "Latest Posts" }
                (latest_posts)
            }
        }
        Footer {}
    }
}
