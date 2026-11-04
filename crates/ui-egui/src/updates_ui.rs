//! Help › Check for Updates, and the check when the app starts.
//!
//! The app asks GitHub for the latest release of the repository it was built from and, when that
//! release is newer, offers its download page. The repository comes from the build:
//! `CLERDOVECTOR_REPO=owner/name` (the release workflow sets it to the repository it runs in). A
//! build without it, such as a local `cargo build`, doesn't check.

use serde_json::Value;

use crate::ClerdovectorApp;

/// `owner/name` of the GitHub repository whose releases this build follows, if any.
pub const REPO: Option<&str> = option_env!("CLERDOVECTOR_REPO");

/// Where an update check stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Status {
    Idle,
    Checking,
    UpToDate,
    /// A newer release: its version and its page on GitHub.
    Available {
        version: String,
        url: String,
    },
    Failed(String),
    /// This build follows no repository (or runs in the browser).
    NotSetUp,
}

pub(crate) struct Runtime {
    status: std::sync::Arc<std::sync::Mutex<Status>>,
    /// The desktop app turns the check at start on; tests and the web build leave it off.
    startup_check: bool,
    startup_started: bool,
    /// The check at start runs quietly: its window opens only when it finds a newer release.
    quiet: bool,
    check_requested: bool,
    /// The context of the last frame, to wake the UI when a check finishes or is requested.
    repaint: Option<egui::Context>,
    /// Whether the update window is in view.
    pub(crate) manual_window_open: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self {
            status: std::sync::Arc::new(std::sync::Mutex::new(Status::Idle)),
            startup_check: false,
            startup_started: false,
            quiet: false,
            check_requested: false,
            repaint: None,
            manual_window_open: false,
        }
    }
}

impl Runtime {
    pub(crate) fn status(&self) -> Status {
        self.status.lock().map(|s| s.clone()).unwrap_or(Status::Idle)
    }

    fn set(&self, status: Status) {
        if let Ok(mut s) = self.status.lock() {
            *s = status;
        }
    }

    /// Start a check in the background (the network call never holds up a frame).
    fn start(&mut self, ctx: &egui::Context) {
        if self.status() == Status::Checking {
            return;
        }
        let Some(repo) = REPO.filter(|r| valid_repo(r)) else {
            self.set(Status::NotSetUp);
            return;
        };
        if cfg!(target_arch = "wasm32") {
            self.set(Status::NotSetUp);
            return;
        }
        self.set(Status::Checking);
        let (status, ctx) = (self.status.clone(), ctx.clone());
        #[cfg(not(target_arch = "wasm32"))]
        {
            let spawned = std::thread::Builder::new().name("update-check".into()).spawn(move || {
                let result = latest_release(repo).and_then(|body| newer_release(&body, env!("CARGO_PKG_VERSION")));
                let next = match result {
                    Ok(Some((version, url))) => Status::Available { version, url },
                    Ok(None) => Status::UpToDate,
                    Err(why) => Status::Failed(why),
                };
                if let Ok(mut s) = status.lock() {
                    *s = next;
                }
                ctx.request_repaint();
            });
            if let Err(e) = spawned {
                self.set(Status::Failed(e.to_string()));
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = (status, ctx, repo);
    }
}

impl ClerdovectorApp {
    /// Whether the app looks for a newer release when it starts (off until the desktop app turns it on).
    pub fn set_startup_update_check(&mut self, on: bool) {
        self.updates_rt.startup_check = on;
    }
}

/// Help › Check for Updates.
pub(crate) fn check(app: &mut ClerdovectorApp) -> Result<Value, String> {
    let rt = &mut app.updates_rt;
    rt.check_requested = true;
    rt.quiet = false;
    rt.manual_window_open = true;
    if let Some(ctx) = &rt.repaint {
        ctx.request_repaint();
    }
    Ok(Value::Null)
}

pub(crate) fn tick(app: &mut ClerdovectorApp, ctx: &egui::Context) {
    let rt = &mut app.updates_rt;
    rt.repaint = Some(ctx.clone());
    if std::mem::take(&mut rt.check_requested) {
        rt.start(ctx);
    } else if rt.startup_check && !rt.startup_started {
        rt.startup_started = true;
        rt.quiet = true;
        rt.start(ctx);
    }
    // The quiet check at start shows its window only for a newer release.
    if rt.quiet {
        match rt.status() {
            Status::Available { .. } => {
                rt.quiet = false;
                rt.manual_window_open = true;
            }
            Status::Checking | Status::Idle => {}
            _ => rt.quiet = false,
        }
    }
}

pub(crate) fn show(app: &mut ClerdovectorApp, ctx: &egui::Context) {
    if !app.updates_rt.manual_window_open {
        return;
    }
    let status = app.updates_rt.status();
    let current = env!("CARGO_PKG_VERSION");
    let mut open = true;
    let mut action = None;
    egui::Window::new(tl!("ClerdoVector Updates"))
        .open(&mut open)
        .collapsible(false)
        .resizable(false)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .show(ctx, |ui| {
            ui.set_min_width(320.0);
            match &status {
                Status::Idle | Status::Checking => {
                    ui.horizontal(|ui| {
                        ui.spinner();
                        ui.label(tl!("Checking for updates…"));
                    });
                }
                Status::UpToDate => {
                    ui.label(crate::i18n::fmt(tl!("ClerdoVector {version} is the latest version."), &[("version", current)]));
                }
                Status::Available { version, .. } => {
                    ui.label(crate::i18n::fmt(
                        tl!("ClerdoVector {new} is available. You have {current}."),
                        &[("new", version.as_str()), ("current", current)],
                    ));
                }
                Status::Failed(why) => {
                    ui.label(tl!("Couldn't check for updates."));
                    ui.label(egui::RichText::new(why).small());
                }
                Status::NotSetUp => {
                    ui.label(tl!("This build of ClerdoVector doesn't check for updates."));
                }
            }
            ui.add_space(10.0);
            ui.horizontal(|ui| match &status {
                Status::Available { url, .. } => {
                    if ui.button(tl!("Download")).clicked() {
                        action = Some(Action::Open(url.clone()));
                    }
                    if ui.button(tl!("Later")).clicked() {
                        action = Some(Action::Close);
                    }
                }
                Status::Failed(_) => {
                    if ui.button(tl!("Check Again")).clicked() {
                        action = Some(Action::Again);
                    }
                    if ui.button(tl!("Close")).clicked() {
                        action = Some(Action::Close);
                    }
                }
                Status::Checking | Status::Idle => {}
                _ => {
                    if ui.button(tl!("OK")).clicked() {
                        action = Some(Action::Close);
                    }
                }
            });
        });
    match action {
        Some(Action::Open(url)) => {
            app.open_url(&url);
            open = false;
        }
        Some(Action::Again) => app.updates_rt.start(ctx),
        Some(Action::Close) => open = false,
        None => {}
    }
    if !open {
        app.updates_rt.manual_window_open = false;
        if app.updates_rt.status() != Status::Checking {
            app.updates_rt.set(Status::Idle);
        }
    }
}

enum Action {
    Open(String),
    Again,
    Close,
}

/// `owner/name` with GitHub's characters only, so it can't steer the request elsewhere.
fn valid_repo(repo: &str) -> bool {
    let ok = |part: &str| !part.is_empty() && part.len() <= 100 && part.chars().all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c));
    matches!(repo.split_once('/'), Some((owner, name)) if ok(owner) && ok(name) && !name.contains('/'))
}

/// The JSON of `repo`'s latest published release.
#[cfg(not(target_arch = "wasm32"))]
fn latest_release(repo: &str) -> Result<String, String> {
    use std::time::Duration;
    let tls = ureq::tls::TlsConfig::builder().root_certs(ureq::tls::RootCerts::PlatformVerifier).build();
    let agent: ureq::Agent = ureq::Agent::config_builder().tls_config(tls).timeout_global(Some(Duration::from_secs(15))).build().into();
    let url = format!("https://api.github.com/repos/{repo}/releases/latest");
    match agent
        .get(&url)
        .header("User-Agent", &format!("ClerdoVector/{}", env!("CARGO_PKG_VERSION")))
        .header("Accept", "application/vnd.github+json")
        .call()
    {
        Ok(mut response) => response.body_mut().with_config().limit(2 << 20).read_to_string().map_err(|e| e.to_string()),
        Err(ureq::Error::StatusCode(404)) => Err("No release has been published yet.".into()),
        Err(ureq::Error::StatusCode(403 | 429)) => Err("GitHub is busy. Try again in a few minutes.".into()),
        Err(e) => Err(e.to_string()),
    }
}

/// From a release's JSON: its version and page when it is newer than `current`, `None` when it
/// isn't. The page must be on github.com, as the app opens it in the browser.
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn newer_release(body: &str, current: &str) -> Result<Option<(String, String)>, String> {
    let v: Value = serde_json::from_str(body).map_err(|e| format!("GitHub's answer couldn't be read ({e})"))?;
    let tag = v["tag_name"].as_str().ok_or("the release has no version tag")?;
    let url = v["html_url"].as_str().ok_or("the release has no page")?;
    if !url.starts_with("https://github.com/") {
        return Err("the release page isn't on github.com".into());
    }
    let (Some(latest), Some(mine)) = (version_parts(tag), version_parts(current)) else {
        return Err(format!("the release version `{tag}` isn't a version number"));
    };
    let version = tag.trim_start_matches(['v', 'V']).to_string();
    Ok((latest > mine).then(|| (version, url.to_string())))
}

/// `v1.2.3` or `1.2.3-beta` → `[1, 2, 3]` (at most four numbers; pre-release text ignored).
#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn version_parts(s: &str) -> Option<Vec<u64>> {
    let core = s.trim().trim_start_matches(['v', 'V']).split(['-', '+']).next()?;
    let parts: Option<Vec<u64>> = core.split('.').map(|p| p.parse().ok()).collect();
    parts.filter(|p| !p.is_empty() && p.len() <= 4)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clerdovector_engine::Session;

    #[test]
    fn default_runtime_is_idle() {
        let rt = Runtime::default();
        assert!(!rt.manual_window_open);
        assert!(!rt.startup_check);
        assert_eq!(rt.status(), Status::Idle);
    }

    #[test]
    fn versions_compare_by_number() {
        assert_eq!(version_parts("v1.10.0"), Some(vec![1, 10, 0]));
        assert_eq!(version_parts("0.21.0-beta.1"), Some(vec![0, 21, 0]));
        assert!(version_parts("v1.10.0") > version_parts("1.9.9"));
        assert_eq!(version_parts("latest"), None);
    }

    #[test]
    fn only_a_newer_release_on_github_is_offered() {
        let release = |tag: &str, url: &str| format!(r#"{{"tag_name":"{tag}","html_url":"{url}"}}"#);
        let page = "https://github.com/clerdo/clerdovector/releases/tag/v0.21.0";
        assert_eq!(newer_release(&release("v0.21.0", page), "0.20.0"), Ok(Some(("0.21.0".into(), page.into()))));
        assert_eq!(newer_release(&release("v0.20.0", page), "0.20.0"), Ok(None));
        assert_eq!(newer_release(&release("v0.19.0", page), "0.20.0"), Ok(None));
        assert!(newer_release(&release("v9.0.0", "https://example.com/x"), "0.20.0").is_err());
        assert!(newer_release("not json", "0.20.0").is_err());
    }

    #[test]
    fn repository_names_are_checked() {
        assert!(valid_repo("clerdo/clerdovector"));
        assert!(valid_repo("some-user/my.app_2"));
        for bad in ["", "clerdo", "a/b/c", "../x/y", "a b/c", "owner/name?x=1", "/name"] {
            assert!(!valid_repo(bad), "{bad}");
        }
    }

    #[test]
    fn the_window_shows_each_outcome() {
        let mut app = ClerdovectorApp::new(Session::new(), Default::default());
        let ctx = egui::Context::default();
        crate::theme::install_fonts(&ctx);
        for status in [
            Status::Checking,
            Status::UpToDate,
            Status::Available { version: "9.0.0".into(), url: "https://github.com/o/r/releases/tag/v9.0.0".into() },
            Status::Failed("offline".into()),
            Status::NotSetUp,
        ] {
            app.updates_rt.set(status);
            app.updates_rt.manual_window_open = true;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| show(&mut app, ui.ctx()));
            out.textures_delta.clear();
            assert!(app.updates_rt.manual_window_open);
        }
    }
}
