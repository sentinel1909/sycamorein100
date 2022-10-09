// footer.rs

use chrono::{Datelike, Local};
use sycamore::prelude::*;

#[component]
pub fn Footer<G: Html>(cx: Scope) -> View<G> {
    let year = Local::now().year();
    view! { cx,
        footer {
            p { "Copyright" " " (year) " " "Jeffery D Mitchell All Rights Reserved" }
            ul {
              li {
                a(href="https://twitter.com/sentinel1909") { "Twitter" }
              }
              li {
                a(href="https://www.instagram.com/thesentinel1909") { "Instagram" }
              }
            }
            a(href="https://sycamore-rs.netlify.app/") { "Made with Sycamore" }
      }
    }
}
