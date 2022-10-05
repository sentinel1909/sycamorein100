// home.rs

use sycamore::prelude::*;

use crate::components::header::Header;
use crate::components::navigation::Navigation;
use crate::components::footer::Footer;
use crate::domain::data::get_articles;

#[component]
pub fn Home<G: Html>(cx: Scope) -> View<G> {
    let articles = get_articles(); 
    let views = View::new_fragment(
        articles.iter().map(|&x| view! { cx, article { p { (x) } }}).collect()
    );
    view! { cx,
        Header {}
        Navigation {}
        
        main {
            section{
                h3 { "Learning Progress and Achievements" }
                (views)
            }
        }
        Footer {}
    }
}
