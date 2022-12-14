// data.rs

pub fn get_posts() -> Vec<&'static str> {
    let content_articles = vec![
        "2022-10-01: Site initially created",
        "2022-10-02: Re-factored and broke code into a front end and server. Rocket serves the files generated by Trunk/Sycamore.",
        "2022-10-03: Today I learned how to use routes in Sycamore to enable navigation between pages.",
        "2022-10-04: Re-factored to create a data file containing a function that returns a vector of static strings. The static strings are rendered out as a view.",
        "2022-10-05: Thanks to the mdsycx crate, I've added the ability to work with Markdown.",
        "2022-10-06: Removed the mdsycx markdown crate (I'm not quite there yet in my ability to use it) and restructured the site with some new pages.",
        "2022-10-08: Added the chrono crate to handle dates and times within the app."
    ];
    content_articles
}
