#[derive(Clone)]
pub struct CommunityItem {
    pub name: &'static [&'static str],
    pub description: &'static str,
    pub link: &'static str,
    pub icon: &'static str,
    pub brand_src: &'static str,
    pub brand_alt: &'static str,
    pub country: Option<&'static str>,
}

impl CommunityItem {
    pub fn matches(&self, search: &str) -> bool {
        let name = self.name.concat();
        let country = self.country.unwrap_or_default();

        name.contains(search) || country.contains(search) || self.description.contains(search)
    }
}

pub struct ProjectItem {
    pub name: &'static [&'static str],
    pub category: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub brand_src: &'static str,
    pub button_link: &'static str,
    pub button_text: &'static str,
    pub brand_as_letter: bool,
    pub button_bg_color: &'static str,
}

#[derive(PartialEq)]
pub struct ResourceItem {
    pub name: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub language: &'static str,
    pub kinds: &'static [&'static str],
    pub featured: bool,
}

pub struct ChannelItem {
    pub name: &'static str,
    pub description: &'static str,
    pub avatar: &'static str,
    pub youtube: Option<&'static str>,
    pub twitch: Option<&'static str>,
}

mod contributor;
mod github_user;
mod top_project;
pub use contributor::Contributor;
pub use github_user::GithubUser;
pub use top_project::TopProject;
