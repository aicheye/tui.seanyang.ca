//! Site content (jobs, projects, quotes, socials, adjectives).
//!
//! The data is fetched at runtime over HTTP from the website repo's
//! `public/data/*.json` (via jsDelivr by default — see [`store`]), which is
//! the single source of truth shared with the web frontend. A bundled snapshot in
//! [`fallback`] keeps the TUI fully functional when the site is unreachable.
//!
//! Use [`snapshot`] from the UI to read the current data, [`init`] once at
//! startup to perform the first (awaited) pull, and [`refresh_if_stale`] on
//! every SSH connection to keep it fresh (rate-limited to one pull per minute).

mod fallback;
mod model;
mod store;

pub use model::{Project, SiteData, Social};
pub use store::{init, refresh_if_stale, snapshot};
