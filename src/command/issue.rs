use std::fs::File;
use std::io::prelude::*;

struct RSA {
    name: String,
    address: String,
    city: String,
    private: bool,
    telephone: String,
    email: String,
    website: String,
}

pub async fn generate_issue(number: i64) {
    let airtable = crate::airtable::Airtable::new(
        &std::env::var("AIRTABLE_TOKEN").expect("AIRTABLE_TOKEN must be set"),
    );
    let records = airtable.records("app0PapFh5s0YJGkj", "RSA").await;

    let mut rsas = Vec::new();
    for record in records {
        // links.push(link);
    }

    let body = format!(
        r#"+++
title="{title}"
date="{date}"
[extra]
issue={number}
+++
{articles}

### 📰 News
{news}

### 📺 Videos
{videos}
"#
    );

    let mut file = File::create(format!("./website/content/{}.md", number)).unwrap();
    file.write_all(body.as_bytes()).unwrap();
    println!("{:?}", title);
}
