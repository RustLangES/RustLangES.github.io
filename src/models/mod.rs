#[derive(Clone)]
pub struct CommunityItem {
    pub name: &'static [&'static str],
    pub description: &'static str,
    pub link: &'static str,
    pub icon: &'static str,
    pub brand_src: &'static str,
    pub brand_alt: &'static str,
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

mod github_user;
pub use github_user::GithubUser;
