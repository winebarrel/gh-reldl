use crate::cli;

use anyhow::anyhow;
use anyhow::Result;
use octocrab::Octocrab;
use reqwest::header;
use reqwest::StatusCode;
use std::io;
use std::io::Write;

pub async fn download(opts: &cli::Options) -> Result<()> {
    let octocrab = if let Some(token) = &opts.token {
        Octocrab::builder()
            .personal_token(token.clone())
            .build()
            .unwrap()
    } else {
        Octocrab::default()
    };

    let rel = octocrab
        .repos(&opts.release.owner, &opts.release.repo)
        .releases()
        .get_by_tag(&opts.release.tag)
        .await?;

    let asset = if let Some(a) = rel.assets.iter().find(|a| a.name == opts.release.name) {
        a
    } else {
        return Err(anyhow!("File not found: {}", &opts.release.name));
    };

    let cli = reqwest::Client::new();
    let mut req = cli
        .get(asset.url.as_str())
        .header(header::USER_AGENT, format!("gh-reldl/{}", cli::VERSION))
        .header(header::ACCEPT, "application/octet-stream");

    if let Some(token) = &opts.token {
        req = req.header(header::AUTHORIZATION, format!("token {}", token));
    }

    let res = req.send().await?;
    let status = res.status();
    let content = res.bytes().await?;

    if status != StatusCode::OK {
        return Err(anyhow!(
            "{}: {}",
            status,
            String::from_utf8(content.to_vec()).unwrap().trim()
        ));
    }

    if opts.file == "-" {
        io::stdout().write_all(&content).unwrap();
    } else {
        let mut dest = std::fs::File::create(&opts.file).unwrap();
        dest.write_all(&content).unwrap();
    }

    Ok(())
}
