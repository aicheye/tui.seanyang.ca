//! Owned data models, deserialized from the website's JSON.
//!
//! The website serves a strict superset of what the TUI needs (e.g. job
//! `logo`s, social `label`s) — serde simply ignores the extra fields, so the
//! TUI only declares what it renders.

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Job {
    pub title: String,
    pub company: String,
    pub description: String,
    pub website: String,
    pub location: String,
    pub technologies: Vec<String>,
    pub dates: Vec<String>,
    pub current: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub title: String,
    pub description: String,
    pub technologies: Vec<String>,
    #[serde(default)]
    pub github: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Quote {
    pub text: String,
    pub author: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Social {
    pub name: String,
    pub url: String,
    pub handle: String,
}

/// Shape of `socials.json`: the primary email plus the social links.
#[derive(Clone, Debug, Deserialize)]
pub struct SocialsDoc {
    #[serde(rename = "primaryEmail")]
    pub primary_email: PrimaryEmail,
    pub socials: Vec<Social>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PrimaryEmail {
    /// Bare address shown in the TUI, e.g. `sean@seanyang.me`.
    pub label: String,
}

/// A complete, consistent set of site content.
#[derive(Clone, Debug)]
pub struct SiteData {
    pub adjectives: Vec<String>,
    pub jobs: Vec<Job>,
    pub projects: Vec<Project>,
    pub quotes: Vec<Quote>,
    pub socials: Vec<Social>,
    pub primary_email: PrimaryEmail,
}
