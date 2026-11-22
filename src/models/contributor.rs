#[derive(Debug, Clone)]
pub struct Contributor {
    pub login: &'static str,
    pub name: Option<&'static str>,
    pub avatar: &'static str,
    pub location: Option<&'static str>,
    pub twitter: Option<&'static str>,
    pub contributions: u32,
}

impl Contributor {
    pub fn display_name(&self) -> &'static str {
        self.name.unwrap_or(self.login)
    }

    pub fn github_url(&self) -> String {
        format!("https://github.com/{}", self.login)
    }

    pub fn twitter_url(&self) -> Option<String> {
        self.twitter.map(|t| format!("https://x.com/{t}"))
    }
}
