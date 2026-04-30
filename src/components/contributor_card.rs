use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct ContributorInfo {
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub twitter_username: Option<String>,
    pub location: Option<String>,
    pub total_contributions: u64,
}

#[component]
pub fn ContributorCard(contributor: ContributorInfo) -> impl IntoView {
    let twitter = contributor.twitter_username.clone();
    let github_url = contributor.url.clone();
    let avatar = contributor.avatar_url.clone();
    let login = contributor.login.clone();
    let location = contributor.location.clone();

    view! {
        <div class="flex flex-row items-center gap-4 p-4 border-2 border-black dark:border-neutral-600 rounded-2xl bg-white dark:bg-neutral-900">
            <img
                src=avatar
                alt=login.clone()
                class="w-14 h-14 rounded-full border-2 border-black dark:border-neutral-500 flex-shrink-0 object-cover"
            />
            <div class="flex flex-col flex-1 min-w-0">
                <span class="font-bold text-sm truncate">{login}</span>
                {location.map(|l| view! {
                    <div class="flex items-center gap-1 mt-1">
                        <span class="text-xs bg-orange-100 dark:bg-neutral-800 px-2 py-0.5 rounded-full">
                            "📍 "{l}
                        </span>
                    </div>
                })}
            </div>
            <div class="flex gap-1.5 flex-shrink-0">
                {twitter.map(|tw| view! {
                    <a
                        href=format!("https://twitter.com/{}", tw)
                        target="_blank"
                        class="p-2 rounded-full border border-black dark:border-neutral-500 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                    >
                        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M18.244 2.25h3.308l-7.227 8.26 8.502 11.24H16.17l-4.714-6.231-5.401 6.231H2.74l7.73-8.835L1.254 2.25H8.08l4.713 6.231zm-1.161 17.52h1.833L7.084 4.126H5.117z"/>
                        </svg>
                    </a>
                })}
                <a
                    href=github_url
                    target="_blank"
                    class="p-2 rounded-full border border-black dark:border-neutral-500 hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.019 10.019 0 0022 12.017C22 6.484 17.522 2 12 2z"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}
