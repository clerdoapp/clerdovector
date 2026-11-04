//! Help → community and project links (the Clerdo Discord, website, this app's page and its
//! GitHub repository). The commands return the URL; the frontend opens it.

use serde_json::{Value, json};

use super::{CommandSpec, Result, always, cmd};

/// The app's id.
pub const APP_ID: &str = "clerdovector";
/// The ClerdoVector community Discord.
pub const DISCORD_URL: &str = "https://discord.gg/CHu4QqrWtf";
/// Clerdo's website.
pub const WEBSITE_URL: &str = "https://clerdo.app";
/// `owner/name` of the GitHub repository this build was made from (set by the release workflow).
pub const REPO: Option<&str> = option_env!("CLERDOVECTOR_REPO");

/// This app's page on the website.
pub fn app_page_url() -> String {
    WEBSITE_URL.to_string()
}

/// This app's source repository: the one it was built from, else the website.
pub fn github_url() -> String {
    match REPO {
        Some(repo) if !repo.is_empty() => format!("https://github.com/{repo}"),
        _ => WEBSITE_URL.to_string(),
    }
}

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(query "help.discord", "Join Our Discord", ["Help"], None, "{} → {url} the Clerdo community Discord", always, |_, _| url(DISCORD_URL.into())),
        cmd!(query "help.website", "Clerdo Website", ["Help"], None, "{} → {url}", always, |_, _| url(WEBSITE_URL.into())),
        cmd!(query "help.appPage", "ClerdoVector on clerdo.app", ["Help"], None, "{} → {url} this app's page", always, |_, _| url(app_page_url())),
        cmd!(query "help.github", "ClerdoVector on GitHub", ["Help"], None, "{} → {url} source code, issues and releases", always, |_, _| url(github_url())),
        cmd!(query "help.links", "Links", [], None, "{} → {discord, website, appPage, github}", always, |_, _| {
            Ok(json!({ "discord": DISCORD_URL, "website": WEBSITE_URL, "appPage": app_page_url(), "github": github_url() }))
        }),
    ]
}

fn url(u: String) -> Result<Value> {
    Ok(json!({ "url": u }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Session;

    #[test]
    fn links() {
        let mut s = Session::new();
        assert_eq!(s.execute("help.discord", &json!({})).unwrap()["url"], DISCORD_URL);
        assert_eq!(s.execute("help.appPage", &json!({})).unwrap()["url"], "https://clerdo.app");
        assert_eq!(s.execute("help.github", &json!({})).unwrap()["url"], github_url());
        assert_eq!(s.execute("help.links", &json!({})).unwrap()["website"], "https://clerdo.app");
    }
}
