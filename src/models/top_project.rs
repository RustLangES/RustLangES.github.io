use super::GithubUser;

#[derive(Debug, Clone)]
pub struct TopProject {
    pub name: &'static str,
    pub description: &'static str,
    pub link: &'static str,
    pub stars: Option<u32>,
    pub contributors: &'static [GithubUser],
}
