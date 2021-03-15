use atom_syndication::Feed;
use std::{fs::File, io::Write};
extern crate handlebars;

use handlebars::Handlebars;
use serde::Serialize;

#[derive(Serialize)]
struct Item {
    url: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prepare output
    let mut file = File::create("urls.html")?;
    let source = "{{url}} {{title}}";

    // Grab Reddit Rss Feed
    let content = reqwest::get("https://www.reddit.com/.rss")
        .await?
        .bytes()
        .await?;

    let feed = Feed::read_from(&content[..]).unwrap();

    // Loop on different entries
    let mut handlebars = Handlebars::new();
    handlebars.register_template_string("tpl", source)?;

    let mut data;

    for x in feed.entries() {
        data = Item {
            url: x.links[0].href.to_string(),
            title: x.title().to_string(),
        };

        let rendered = format!("{}", handlebars.render("tpl", &data)?);

        println!("{}", rendered);
        file.write_all(rendered.into_bytes().as_ref());
    }

    Ok(())
}
