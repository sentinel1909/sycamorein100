// main.rs

use sycamore::prelude::*;

use sycamorein100_lib::routes::router::SiteRouter;

fn main() {
    sycamore::render(|cx| {
        view! { cx,
            SiteRouter {}
        }
    });
}
