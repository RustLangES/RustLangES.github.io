mod head_information;
mod header;
mod icons;

pub use head_information::HeadInformation;
pub use header::Header;
pub use icons::{
    CloudflareIcon, DiscordIcon, GithubIcon, LinkedinIcon, LocationIcon, NextIcon, TelegramIcon,
    TwitterIcon,
};

pub mod became_sponsor;
pub mod challenges;
pub mod channels;
pub mod community_project;
pub mod footer;
pub mod our_community;
pub mod our_sponsors;
pub mod project_card;
pub mod resources;
pub mod sponsor_block;
pub mod why_rust;

const BOOK_PATH: &str = "https://book.rustlang-es.org/";
const JOIN_PATH: &str = "https://discord.rustlang-es.org/";
const GITHUB_PATH: &str = "https://github.com/RustLangES";
const LINKEDIN_PATH: &str = "https://www.linkedin.com/company/rustlanges";
const TELEGRAM_PATH: &str = "https://t.me/rust_es";
