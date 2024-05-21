#![allow(non_snake_case)]

use chrono::{DateTime, Utc};
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::Level;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/test")]
    Test(),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryPageData {
    #[serde(flatten)]
    pub item: StoryItem,
    #[serde(default)]
    pub comments: Vec<Comment>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub text: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    #[serde(default)]
    pub sub_comments: Vec<Comment>,
    pub r#type: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StoryItem {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub text: Option<String>,
    #[serde(default)]
    pub by: String,
    #[serde(default)]
    pub score: i64,
    #[serde(default)]
    pub descendants: i64,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    #[serde(default)]
    pub kids: Vec<i64>,
    pub r#type: String,
}
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Blog(id: i32) -> Element {
    rsx! {
        Link { to: Route::Home {}, "Go to counter" }
        "Blog post {id}"
    }
}

#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Link {
    class:"hover:underline",
            to: Route::Blog {
                id: count()
            },
            "Go to blog"
        }
        br {}
        Link {
    class:"hover:underline",
            to: Route::Test{},
            "Go to test"
        }
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

#[component]
fn Test() -> Element {
    rsx! {StoryListing{
        story: StoryItem  {
            id: 0,
            title: "hello hackernews".to_string(),
        url: None,
        text: None,
        by: "Author".to_string(),
        score: 0,
        descendants: 0,
        time: chrono::Utc::now(),
        kids: vec![],
        r#type: "".to_string(),
        }
    }}
}

#[component]
fn StoryListing(story: ReadOnlySignal<StoryItem>) -> Element {
    let StoryItem {
        title,
        url,
        by,
        score,
        time,
        kids,
        ..
    } = &*story.read();

    let url = url.as_deref().unwrap_or_default();
    let hostname = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");

    let score = format!("{score} {}", if *score == 1 { "point" } else { "points" });
    let time = time.format("%D %l:%M %p");

    let comments = kids.len();

    rsx! {
            div { padding: "0.5rem", position: "relative",
                div {
                font_size: "1.5rem",
                a { href: url, "{title}"}
                a {
            color: "gray",
            href: "https://news.ycombinator.com/from?site={hostname}",
            text_decoration: "none", " ({hostname})"}
            }
        div { display: "flex", flex_direction: "row", color: "gray",
        div {"{score}"}
        div {padding_left: "0.5rem", "by {by}"}
        div {padding_left: "0.5rem", "{time}"}
        div {padding_left: "0.5rem", "{comments}"}
    }
            }
        }
}
