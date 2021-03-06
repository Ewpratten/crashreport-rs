use std::{panic::PanicInfo, time::SystemTime};

use url::Url;

use crate::panic_handler::CargoPanicMetadata;

/// Describes supported repository providers.
#[derive(Debug)]
pub enum RepositoryProvider {
    GitHub(Url),
    GitLab(Url),
}

impl RepositoryProvider {
    /// Returns the full URL required to file an issue in the repository of the client crate.
    ///
    /// Most Git providers allow issue creation through query params
    pub fn build_issue_url(&self, info: &PanicInfo<'_>, metadata: &CargoPanicMetadata) -> Url {
        // No matter the provider, some of the strings are the same.
        let body = vec![
            "\n<h2>Information</h2>\n\n<!-- Please describe what happened to the best of your ability here -->",
            "\n<h3>Error</h3>\n\n```",
            &info.to_string(),
            "```",
            "\n<h3>Additional Info</h3>\n\n```",
            &format!("OS: {}", std::env::consts::OS),
            &format!("Architecture: {}", std::env::consts::ARCH),
            &format!("Timestamp: {:?}", SystemTime::now()),
            &format!("Version: {}", metadata.version),
            &format!("Package: {}", metadata.pkg_name),
            &format!("Crate: {}", metadata.crate_name),
            "```\n",
            "---",
            "*This issue was auto-generated by the [`crashreport`](https://github.com/ewpratten/crashreport-rs) crate.*",
            "*If you would like to improve these diagnostics, please contribute on GitHub.*"
        ].join("\n");

        match self {
            RepositoryProvider::GitHub(url) => {
                let mut output_url = url.clone();
                let path = url.path();
                output_url.set_path(&format!("{}/issues/new", path));
                output_url.set_query(Some(&format!(
                    "body={}&labels=bug",
                    urlencoding::encode(&body)
                )));
                output_url
            }
            RepositoryProvider::GitLab(url) => {
                let mut output_url = url.clone();
                let path = url.path();
                output_url.set_path(&format!("{}/issues/new", path));
                output_url.set_query(Some(&format!(
                    "issue[description]={}&issuable_template=bug",
                    urlencoding::encode(&body)
                )));
                output_url
            }
        }
    }
}
