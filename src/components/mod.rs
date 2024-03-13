mod aprende;
mod button_link;
mod cards;
mod community_projects;
mod footer;
mod head_information;
mod header;
mod hero;
mod icons;
mod other_communities;
mod our_communities;
mod slogan_button;

pub use aprende::{Books, HeaderAprende, Roadmap, Youtube};
pub use button_link::ButtonLink;
pub use cards::{CardTitle, CommunityCard, ContributorCard, ProjectCard};
pub use community_projects::CommunityProjects;
pub use footer::Footer;
pub use head_information::HeadInformation;
pub use header::Header;
pub use hero::Hero;
pub use icons::{
    DiscordIcon, GithubIcon, LinkedinIcon, LocationIcon, NextIcon, TelegramIcon, TwitterIcon,
};
pub use other_communities::OtherCommunities;
pub use our_communities::OurCommunities;
pub use slogan_button::SloganButton;
