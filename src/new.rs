use dialoguer::{theme::ColorfulTheme, Input};
use structopt::StructOpt;

/// 🆕 Creates a new local article md file to get you going
#[derive(StructOpt, Debug)]
pub struct New {
    /// title for article
    #[structopt(long)]
    title: Option<String>,
}

pub async fn run(new: New) -> anyhow::Result<()> {
    let New { title, .. } = new;
    let theme = ColorfulTheme::default();
    let title: String = title
        .ok_or_else(|| anyhow::anyhow!("article title is required"))
        .or_else::<anyhow::Error, _>(|_| {
            Ok(Input::<String>::with_theme(&theme)
                .with_prompt("article title")
                .interact()?)
        })?;
    println!(
        r#"---
title: {title}
published: false
# tags: foo, bar
# date: ...
# series: ...
# canonical_url: ..
# cover_image: ...
---
"#,
        title = title
    );
    Ok(())
}
