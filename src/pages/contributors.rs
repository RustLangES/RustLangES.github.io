use leptos::prelude::*;
#[cfg(feature = "ssr")]
use leptos::serde_json;

use crate::components::ContributorCard;
#[cfg(feature = "ssr")]
use leptos::serde_json::json;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use std::collections::HashMap;

const NOT_COMMUNITY_MEMBERS: [&str; 550] = [
    "chriskrycho", "maxdeviant", "github-actions[bot]", "hustcer", "SpectralPixel", "carols10cents", 
    "amtoine", "NotTheDr01ds", "fdncred", "sholderbach", "kubouch", "devyn", "JosephTLyons", 
    "IanManske", "huacnlee", "sophiajt", "zarifpour", "Hofer-Julian", "Kissaki", "blaqat", "atifaziz", "jpmelos", 
    "maxbrunsfeld", "osiewicz", "WindSoilder", "d1y", "tecandrew", "dependabot[bot]", "someone13574", "Otterpocket", 
    "rgwood", "damoasda", "kwi-dk", "ayax79", "maxim-uvarov", "ssaunderss", "MordFustang21", "vitallium", "webhooked",
    "slymax", "bzierk", "davccavalcante", "filipjanevski", "petrisch", "bajrangCoder", "louni-longheval", "hasit",
    "JoaquinTrinanes", "ysthakur", "grndctrl", "mikasius", "mikesun", "mrnugget", "yukitomoda", "perragnar", 
    "luckydye", "sweetppro", "gdamore", "Imgkl", "e-simpson", "dangh", "ABckh", "valentinegb", "weirdan", 
    "MartinRybergLaude", "ThatOneCalculator", "Moshyfawn", "LaPingvino", "bryanbuchanan", "pingiun", "presidento",
    "rubjo", "foxoman", "joergsch", "meocoder31099", "davidkurilla", "jamesmunns", "denix666", "das-g", "clicktodev",
    "11bit", "tamimhasandev", "Feel-ix-343", "bswinnerton", "rzukic", "kevcamel", "WeetHet", "henrikruscon", 
    "Ifthel", "132ikl", "timcole", "PyaeSoneAungRgn", "timClicks", "mauscoelho", "anikinmd", "nwhetsell",
    "mshaugh", "4teapo", "p3rception", "TornaxO7", "bobhy", "trbutler4", "TWSiO", "vyadh", "notpeter",
    "segersniels", "PedroChaps", "xqsit94", "eth0net", "webbedspace", "borngraced", "h2000", "nathanjcollins",
    "fachammer", "kaathewisegit", "cange", "edhowland", "zaucy", "lnay", "jkasky", "mianlang", "elGusto",
    "yo-goto", "arnau", "teziovsky", "jaudiger", "jpearnshaw", "makifdb", "adorabilis", "stomar", "kirqe",
    "zetashift", "k0tran", "FilipAndersson245", "coghost", "caius", "Kalmaegi", "CAESIUS-TIM", "pcminh0505",
    "gx0r", "lv37", "flukejones", "YussufSassi", "cptpiepmatz", "wisn", "ralfdoering", "stfacc", "savente93",
    "mb21", "abt8601", "mandarvaze", "YukiOnodera", "salihguru", "calmyournerves", "texastoland",
    "connorjs", "abusch", "kai-tub", "mrkkrp", "drmikesamy", "AineeJames", "kejadlen", "Trent-Fellbootman", 
    "rgbkrk", "tesujimath", "BrianLondon", "mrkstwrt", "tbu-", "Abhinav5383", "shuklaayush", "isti115", 
    "koozz", "jafriyie1", "waldyrious", "narqo", "adamchalmers", "Cs4r", "ehuss", "NathanLovato", "Ph0enixKM",
    "LeoniePhiline", "fitzchivalrik", "MarikaChlebowska", "MichalPospech", "eldruin", "Narukara", "hanneskaeufler",
    "EliiseS", "demoded", "abcd-ca", "microsoft-github-policy-service[bot]", "apraga", "knickish", "busslina", 
    "Kenysdev", "burrbull", "dmatos2012", "stormasm", "kiarie404", "akkartik", "gdamjan", "Zarenor", 
    "dmatos-t32", "flying-sheep", "rxwb", "decorator-factory", "nthState-Chris", "OccupyMars2025", "nyurik",
    "FnControlOption", "Eekle", "mgeisler", "orangecms", "emilyalbini", "LUISEDOCCOR", "sharpe5", "josh-degraw",
    "armaxri", "CodeRunRepeat", "bneumann", "jacojvv-betsoftware", "Feriixu", "seishun", "szabgab", "CarlKCarlK",
    "engdoreis", "cassidoxa", "mpawelski", "ChrisDenton", "JoseAUrdanetaM", "eramoss", "aneojgurhem", 
    "9unkn", "zivarah", "BartMassey", "loadresource", "jannic", "fekie", "fhalim", "renovate[bot]", 
    "actions-user", "allcontributors[bot]", "qwandor", "lorenzolewis", "Ekwuno", "vasfvitor", "henrif75", 
    "FabianLars", "simonhyll", "dreyfus92", "djmitche", "lucasfernog", "kearfy", "dimitrianoudi", "igor-petruk",
    "GuoJikun", "dklassic", "fbornhofen", "cchiw", "Dhghomon", "tweidinger", "wnghl", "Odonno", "domenukk",
    "tauri-bot", "jiyongp", "randomPoison", "gguillemas", "pewsheen", "nodmp", "dawnlarsson", "tobiemh",
    "gendx", "jooyunghan", "reta", "johnathan79717", "seporterfield", "hurryabit", "fw-immunant", "kantasv",
    "rastringer", "AdrienBaudemont", "AlexFrid", "anforowicz", "sylxjtu", "timpratim", "emmanuel-keller", 
    "mo8it", "amrbashir", "kuanhungchen", "rbehjati", "mani-chand", "RoseBlume", "baltuky", "proski", 
    "naisofly", "adetaylor", "leonzchang", "tillmann-crabnebula", "brandonpollack23", "skarline",
    "justanotheranonymoususer", "RSS1102", "CoinEZ-JPN", "detro", "Throvn", "ronaldfw", "HidenoriKobayashi",
    "marshallpierce", "mingyc", "anlunx", "nidhalmessaoudi", "moaminsharifi", "errge", "njr0",
    "markozagar", "sakex", "pwnall", "adamac", "olivierlemasle", "joaovicmendes", "ChayimFriedman2",
    "ningcng", "raselmandol", "manyinsects", "yohcop", "thedataking", "superwhd", "mauvealerts", 
    "Embers-of-the-Fire", "KikkiZ", "mobyw", "emmali01", "jbolda", "tshepang", "hugojacob", 
    "keiichiw", "max-heller", "Enes1313", "suetfei", "ImUrX", "naman-crabnebula", "christophsanz", 
    "hamidrezakp", "DelSkayn", "NoahDragon", "SwiftSIQI", "hueich", "klensy", "chippers", "jcvicelli",
    "Doris-Ge", "cyevgeniy", "AgainstEntropy", "i-c-b", "MaschitaG", "LightQuanta", "yaremam", 
    "Rigidity", "x10an14", "athanclark", "friendlymatthew", "dyeroshenko", "ahqsoftwares", "zvonden",
    "eunsukimme", "phughk", "macjuul", "hades2510", "IP1llar", "noamzaks", "Sir-Thom", "guoard", 
    "victorhsieh", "khoaxuantu", "cleveng", "b-apperlo", "StepanZagray", "seanpoulter",
    "1dimir", "eroullit", "poneciak57", "sharunkumar", "JJungs-lee", "schultetwin1", "bholmesdev",
    "auipga", "ahresse", "timuric", "Hassunaama", "abhik-bits", "jrcarl624", "pierd", "EthnTuttle",
    "abhik-99", "kharbanda14", "LizzieH01", "ivan-kiselev", "hezhizhen", "0PandaDEV", "patelka2211", "b6k-dev",
    "sahennenkamp", "az0977776", "ternbusty", "Skwodo", "bean5", "danny900714", "alexandergill", 
    "freeaion", "byarbrough", "jasminewu229", "JohnScience", "aravindputrevu", "murnifine", "WalrusSoup",
    "ilyagr", "tklauser", "0ri0nexe", "BlackTiger007", "ferreira-tb", "rb5014", "husjon", "notriddle",
    "xubeiyan", "HangDu1995", "docwilco", "Gannu456", "akkie", "sgirones", "brenoepics", "loky-lp", "willhack",
    "luis-prates", "Legend-Master", "gabenddos", "welpie21", "jalejotorresm", "cobaltburn", "simonwong",
    "oalders", "ruizdiazever", "washanhanzi", "TACIXAT", "fcoury", "KirillTregubov", "cameronbraid",
    "jaydevelopsstuff", "ShaharNaveh", "dan-myles", "dalance", "KMJ-007", "KennanHunter", "dfaust",
    "sunng87", "giripriyadarshan", "NikolaRHristov", "GlobalHive", "ISSOtm", "alvin883", "tjensen42",
    "Pitasi", "idofilus", "samuelscheit", "reinhard-sanz", "ifraixedes", "matthewjnield", "lucascherzer",
    "Franco-Pertusati", "mateothegreat", "jaimemh", "ShadowWolf308", "ugsto", "AndrewKraevskii", "GhastCraftHD", 
    "JonahPlusPlus", "tfpk", "lilyyy411", "garyhai", "guofoo", "Ochibobo", "ActuallyHappening", "iSaborit",
    "m-c-frank", "nazmul-pro", "lkwr", "SiliconSelf", "pickfire", "lveillard", "Charlie-McLovins", 
    "Julian-Hackenberg", "noritada", "damccull", "skierpage", "gwen-lg", "AliSajid", "generalworksinc", 
    "yeonsy", "maxwellflitton", "5Dev24", "cyril-marpaud", "ilkerkorkut", "juangesino",
    "danieleades", "MatthewAry", "ManAnRuck", "rschooley", "magnusrodseth", "joserg1801", "zvictor",
    "Catchawink", "jlalfonso21", "ThatColdToast", "kotolex", "dbarrosop", 
    "ZibanPirate", "paradox8599", "Oughie", "alex-crabnebula", "jason89521", "Czxck001",
    "hangriver", "adrian-ub", "beninada", "Sindrir", "vjousse", "Uninen", 
    "denjell-crabnebula", "Trojanking123", "as-flow", "samcday", "knownasnaffy", "Juan-LukeKlopper",
    "kaikaibenkai", "peppergrayxyz", "dionysuzx", "Victoria-Casasampere-BeeTheData", "yixinNB",
    "skyslide22", "ahkohd", "swordtraveller", "vemonet", "cnlancehu", "noxan", "Niavana97",
    "ryansteil", "bitQ2019", "regexident", "jcasben", "waozixyz"
];


#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Contributor {
    pub login: String,
    pub avatar_url: String,
    pub url: String,
    pub bio: Option<String>,
    pub twitter_username: Option<String>,
    pub location: Option<String>,
    pub contributions: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContributorsResponse {
    pub contributors: Vec<Contributor>,
    pub total: usize,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct GithubRepo {
    name: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct RepoStatsContributor {
    author: Option<RepoStatsAuthor>,
    weeks: Vec<RepoStatsWeek>,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct RepoStatsAuthor {
    login: String,
    avatar_url: String,
    html_url: String,
}

#[cfg(feature = "ssr")]
#[derive(Debug, Deserialize)]
struct RepoStatsWeek {
    w: i64,
    c: u64,
}

#[cfg(feature = "ssr")]
struct UserProfile {
    bio: Option<String>,
    twitter_username: Option<String>,
    location: Option<String>,
}

#[cfg(feature = "ssr")]
mod config {
    pub const ORGANIZATION_LOGIN: &str = "RustLangES";
    pub const API_DELAY_MS: u64 = 200;
    pub const PER_PAGE: u32 = 100;
    pub const CONTRIBUTIONS_SINCE: i64 = 1672531200;
}

#[cfg(feature = "ssr")]
const CONTRIBUTORS_CACHE_PATH: &str = "contributors.json";

pub async fn load_contributors() -> ContributorsResponse {
    #[cfg(feature = "ssr")]
    {
        fetch_contributors().await
    }
    #[cfg(not(feature = "ssr"))]
    {
        use gloo_net::http::Request;
        match Request::get("/contributors.json").send().await {
            Ok(resp) => resp.json::<ContributorsResponse>().await.unwrap_or_default(),
            Err(_) => ContributorsResponse::default(),
        }
    }
}

#[cfg(feature = "ssr")]
async fn fetch_contributors() -> ContributorsResponse {
    let is_debug = cfg!(debug_assertions);

    if is_debug {
        if let Ok(contents) = std::fs::read_to_string(CONTRIBUTORS_CACHE_PATH) {
            if let Ok(cached) = serde_json::from_str::<ContributorsResponse>(&contents) {
                println!("Loaded {} contributors from cache ({})", cached.total, CONTRIBUTORS_CACHE_PATH);
                let dist_path = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "dist".to_string());
                let _ = std::fs::create_dir_all(&dist_path);
                let json_path = format!("{}/contributors.json", dist_path);
                let _ = std::fs::write(&json_path, &contents);
                return cached;
            }
        }
    }

    let mut all_contributors = fetch_all_contributors().await;

    all_contributors.sort_by(|a, b| b.contributions.cmp(&a.contributions));

    let total = all_contributors.len();

    let response = ContributorsResponse {
        contributors: all_contributors,
        total,
    };

    if is_debug {
        match serde_json::to_string_pretty(&response) {
            Ok(json) => {
                if let Err(e) = std::fs::write(CONTRIBUTORS_CACHE_PATH, json) {
                    eprintln!("Failed to write contributors cache: {}", e);
                } else {
                    println!("Saved contributors cache to {}", CONTRIBUTORS_CACHE_PATH);
                }
            }
            Err(e) => eprintln!("Failed to serialize contributors cache: {}", e),
        }
    }

    if let Ok(json) = serde_json::to_string(&response) {
        let dist_path = std::env::var("LEPTOS_SITE_ROOT").unwrap_or_else(|_| "dist".to_string());
        let _ = std::fs::create_dir_all(&dist_path);
        let json_path = format!("{}/contributors.json", dist_path);
        match std::fs::write(&json_path, &json) {
            Ok(_) => println!("Wrote contributors to {}", json_path),
            Err(e) => eprintln!("Failed to write {}: {}", json_path, e),
        }
    }

    response
}

#[cfg(feature = "ssr")]
async fn fetch_all_contributors() -> Vec<Contributor> {
    let token = std::env::var("COLLABORATORS_API_TOKEN").unwrap_or_default();
    if token.is_empty() {
        leptos::logging::warn!("COLLABORATORS_API_TOKEN is empty or not set!");
        return Vec::new();
    }

    let client = reqwest::Client::new();

    let repos = fetch_org_repos(&client, &token).await;

    let mut seen: HashMap<String, (String, String, String, u64)> = HashMap::new();

    for repo in &repos {
        let stats = fetch_repo_stats(&client, &token, &repo.name).await;

        for stat in stats {
            let Some(author) = stat.author else {
                continue;
            };

            let commits: u64 = stat
                .weeks
                .iter()
                .filter(|w| w.w >= config::CONTRIBUTIONS_SINCE)
                .map(|w| w.c)
                .sum();

            if commits == 0 {
                continue;
            }

            if NOT_COMMUNITY_MEMBERS.contains(&author.login.as_str()) {
                continue;
            }

            let key = author.login.trim().to_lowercase();
            seen.entry(key)
                .and_modify(|(_, _, _, total)| *total += commits)
                .or_insert((author.login, author.avatar_url, author.html_url, commits));
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(config::API_DELAY_MS)).await;
    }

    let logins: Vec<String> = seen.values().map(|(login, _, _, _)| login.clone()).collect();
    let profiles = fetch_user_profiles_batch(&client, &token, &logins).await;

    let mut result = Vec::new();
    for (_key, (login, avatar_url, html_url, contributions)) in seen {
        let profile = profiles.get(&login);

        result.push(Contributor {
            login,
            avatar_url,
            url: html_url,
            bio: profile.and_then(|p| p.bio.clone()),
            twitter_username: profile.and_then(|p| p.twitter_username.clone()),
            location: profile.and_then(|p| p.location.clone()),
            contributions: contributions.max(1),
        });
    }

    result
}

#[cfg(feature = "ssr")]
async fn fetch_org_repos(client: &reqwest::Client, token: &str) -> Vec<GithubRepo> {
    let mut all_repos = Vec::new();
    let mut page = 1u32;

    loop {
        let url = format!(
            "https://api.github.com/orgs/{}/repos?per_page={}&page={}",
            config::ORGANIZATION_LOGIN,
            config::PER_PAGE,
            page
        );

        let res = client
            .get(&url)
            .header("User-Agent", "RustLangES Automation Agent")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;

        match res {
            Ok(r) => {
                let text = r.text().await.unwrap_or_default();
                let repos: Vec<GithubRepo> = serde_json::from_str(&text).unwrap_or_default();
                let count = repos.len();
                all_repos.extend(repos);
                if count < config::PER_PAGE as usize {
                    break;
                }
                page += 1;
            }
            Err(e) => {
                leptos::logging::error!("Failed to fetch repos page {}: {:?}", page, e);
                break;
            }
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(config::API_DELAY_MS)).await;
    }

    all_repos
}

#[cfg(feature = "ssr")]
async fn fetch_repo_stats(
    client: &reqwest::Client,
    token: &str,
    repo: &str,
) -> Vec<RepoStatsContributor> {
    let url = format!(
        "https://api.github.com/repos/{}/{}/stats/contributors",
        config::ORGANIZATION_LOGIN,
        repo
    );

    for _ in 0..3u32 {
        let res = client
            .get(&url)
            .header("User-Agent", "RustLangES Automation Agent")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await;

        match res {
            Ok(r) => {
                let status = r.status();
                if status == reqwest::StatusCode::ACCEPTED {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                    continue;
                }

                let text = r.text().await.unwrap_or_default();
                return serde_json::from_str(&text).unwrap_or_default();
            }
            Err(e) => {
                leptos::logging::error!("Failed to fetch stats for {}: {:?}", repo, e);
                return Vec::new();
            }
        }
    }

    leptos::logging::warn!("Stats for '{}' not ready after retries", repo);
    Vec::new()
}

#[cfg(feature = "ssr")]
async fn fetch_user_profiles_batch(
    client: &reqwest::Client,
    token: &str,
    logins: &[String],
) -> HashMap<String, UserProfile> {
    let mut profiles = HashMap::new();

    if logins.is_empty() {
        return profiles;
    }

    for chunk in logins.chunks(50) {
        let mut query_parts = Vec::new();
        for (i, login) in chunk.iter().enumerate() {
            let safe_login = login.replace('"', "");
            query_parts.push(format!(
                r#"u{i}: user(login: "{safe_login}") {{ bio twitterUsername location }}"#
            ));
        }
        let query = format!("query {{ {} }}", query_parts.join(" "));
        let request_body = json!({ "query": query });

        let res = client
            .post("https://api.github.com/graphql")
            .header("User-Agent", "RustLangES Automation Agent")
            .header("Authorization", format!("Bearer {}", token))
            .json(&request_body)
            .send()
            .await;

        let Ok(r) = res else {
            continue;
        };
        let text = r.text().await.unwrap_or_default();
        let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) else {
            continue;
        };
        let Some(data) = parsed.get("data") else {
            continue;
        };
        for (i, login) in chunk.iter().enumerate() {
            let key = format!("u{i}");
            let Some(user) = data.get(&key) else {
                continue;
            };
            if user.is_null() {
                continue;
            }
            profiles.insert(
                login.clone(),
                UserProfile {
                    bio: user
                        .get("bio")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    twitter_username: user
                        .get("twitterUsername")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    location: user
                        .get("location")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                },
            );
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(config::API_DELAY_MS)).await;
    }

    profiles
}

#[component]
pub fn Contributors() -> impl IntoView {
    let contributors = LocalResource::new(|| load_contributors());

    view! {
        <section class="bg-orange-300/30 dark:bg-transparent py-16 min-h-[80vh]">
            <div class="flex flex-col gap-y-6 container mx-auto px-4">
                <h2 class="text-3xl text-left mb-3">
                    <span class="font-work-sans font-light">"Nuestros "</span>
                    <span class="font-alfa-slab text-orange-500">"Colaboradores"</span>
                </h2>
                <p class="md:max-w-[800px] mb-2">
                    Gracias al esfuerzo y dedicación de estos extraordinarios colaboradores open source, los servicios y páginas de nuestra comunidad se mantienen activos y en constante evolución. Su pasión por el código abierto y el desarrollo de Rust es el corazón que impulsa nuestro crecimiento.
                </p>
                <p class="md:max-w-[800px] mb-2">
                    "Te invitamos a unirte a esta vibrante comunidad, explorar nuestros repositorios en "
                    <a href="https://github.com/RustLangES" class="underline" target="_blank">
                        GitHub
                    </a> " y contribuir con tu talento."
                </p>
                <p class="md:max-w-[800px]">
                    <strong>Juntos</strong>
                    , podemos seguir construyendo un ecosistema Rust más fuerte y accesible para todos.
                </p>
                <div class="mt-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 2xl:grid-cols-5 gap-6">
                    <Suspense fallback=|| ()>
                        {move || {
                            contributors
                                .get()
                                .map(|res| {
                                    res.contributors
                                        .clone()
                                        .into_iter()
                                        .map(|item| {
                                            view! {
                                                <ContributorCard
                                                    name=item.login.clone()
                                                    description=item.bio.clone()
                                                    link=item.url.clone()
                                                    brand_src=item.avatar_url.clone()
                                                    twitter=item.twitter_username.clone()
                                                    location=item.location.clone()
                                                    contributions=item.contributions
                                                />
                                            }
                                        })
                                        .collect::<Vec<_>>()
                                })
                        }}
                    </Suspense>
                </div>
            </div>
        </section>
    }
}
