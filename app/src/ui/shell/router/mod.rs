//! The router: the [`Route`] enum, a navigation [`Request`] (a route, and
//! what `ctx.open(path, action)` and `?q=` carried in React), the
//! [`Location`] it becomes and the [`History`] of those.

mod history;
mod path;
mod route;
#[cfg(test)]
mod tests;

pub use self::{
    history::History,
    route::{ApiKind, Namespaced, Route, Tab},
};

/// What a navigation asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Request {
    pub route: Route,
    /// What the page there starts once open, as if its button was pressed
    /// (`ctx.open('/containers', 'run')`, `useActionRequest`).
    pub action: Option<&'static str>,
    /// The text its list starts searched with (`?q=`, `usePageQuery`), as
    /// global search opens an image, a volume or a network.
    pub query: Option<String>,
}

impl Default for Request {
    fn default() -> Self {
        Self::to(Route::Overview)
    }
}

impl Request {
    pub fn to(route: Route) -> Self {
        Self {
            route,
            action: None,
            query: None,
        }
    }

    /// The page starts `action` once open.
    #[must_use]
    pub fn starting(mut self, action: &'static str) -> Self {
        self.action = Some(action);
        self
    }

    /// The page's list starts searched with `query`.
    #[must_use]
    pub fn searching(mut self, query: impl Into<String>) -> Self {
        self.query = Some(query.into());
        self
    }
}

impl From<Route> for Request {
    fn from(route: Route) -> Self {
        Self::to(route)
    }
}

/// One navigation, as the page it leads to reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub route: Route,
    pub action: Option<&'static str>,
    pub query: Option<String>,
    /// Its own for each navigation (React Router's `location.key`): the
    /// same route asked for twice is two locations.
    pub key: u64,
}

impl Location {
    fn new(request: Request, key: u64) -> Self {
        Self {
            route: request.route,
            action: request.action,
            query: request.query,
            key,
        }
    }
}
