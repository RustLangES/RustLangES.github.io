use leptos::leptos_dom::logging::console_log;
use leptos::prelude::{use_context, RwSignal};
use leptos::server::codee::string::JsonSerdeCodec;
use leptos::*;
use leptos::logging::log;
use leptos_use::storage::use_local_storage;
use leptos_use::{use_media_query, use_preferred_dark};
use serde::{Deserialize, Serialize};
use leptos::children::Children;
use leptos::prelude::*;
/// Defines an enumeration for UI themes.
///
/// This enum can be cloned, copied, and compared for equality.
/// It also supports serialization and deserialization for local storage.
#[derive(Clone, Copy, PartialEq, Serialize, Deserialize, Debug)]
pub enum Theme {
    Light,
    Dark,
    System,
}

// Implementation of the default value for the `Theme` enum
impl Default for Theme {
    /// provides the default theme as `Dark`
    fn default() -> Self {
        Theme::Dark
    }
}

impl Theme {
    /// Converts the `Theme` variant into a corresponding string.
    pub fn to_string(self) -> String {
        String::from(match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
            Theme::System => "system",
        })
    }
}

/// Define a constant for the local storage key used to store the theme setting.
const STORAGE_KEY: &'static str = "theme";

/// Updates the class selector for the respective theme.
/// This function is responsible for applying the correct CSS class to the HTML and body elements based on the current theme.
///
/// ## Arguments
/// * `theme` - The current theme (Light, Dark, System)
/// * `prefers_dark` - Boolean flag indicating whether the system prefers a dark theme.
fn update_css_for_theme(theme: Theme, prefers_dark: bool, use_data_attribute: bool) {
    let document = web_sys::window().unwrap().document().unwrap();
    let html_element = document.document_element().unwrap();

    if use_data_attribute {
        match theme {
            Theme::Light => {
                html_element.set_attribute("data-theme", "light").unwrap();
            }
            Theme::Dark => {
                html_element.set_attribute("data-theme", "dark").unwrap();
            }
            Theme::System => match prefers_dark {
                true => html_element.set_attribute("data-theme", "dark").unwrap(),
                false => html_element.set_attribute("data-theme", "light").unwrap(),
            },
        }
    } else {
        match theme {
            Theme::Light => {
                html_element.class_list().remove_1("dark").unwrap();
            }
            Theme::Dark => {
                html_element.class_list().add_1("dark").unwrap();
            }
            Theme::System => match prefers_dark {
                true => html_element.class_list().add_1("dark").unwrap(),
                false => html_element.class_list().remove_1("dark").unwrap(),
            },
        }
    }
}

/// Provides the global `Theme` state
///
/// This function is used to access the current theme state from the global context.
/// The state is wrapped as an `RwSignal`.
pub fn use_theme() -> RwSignal<Theme> {
    use_context::<RwSignal<Theme>>().expect("there should be a global theme state")
}


use leptos::prelude::*;


/// The `ThemeProvider` component.
///
/// This component provides a theme context to its children, allowing them to access and react to theme changes.
///
/// ## Properties
/// * `enable_system` - A boolean flag to sync with the system theme preference.
///                     Defaults to `true`.
/// * `children` - The child components that will consume the theme context.
#[island]
pub fn ThemeProvider(
    children: Children,
) -> impl IntoView {
    let use_data_attribute = true;
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");
    let prefers_dark = move || if is_dark_preferred_signal.get() { true } else { false };
    // let (is_dark_preferred_signal, _) = signal(true);

    // Attempt to retrieve the theme from local storage
    let (theme_storage_state, set_theme_storage_state, _) =
        use_local_storage::<Theme, JsonSerdeCodec>(STORAGE_KEY);

    // Determine the initial theme from local storage
    let initial_theme = theme_storage_state.get();

    // Initialize the theme state with the determined initial theme
    let theme_state = RwSignal::new(initial_theme);
    provide_context(theme_state.clone());

    // // Update local storage whenever the theme state changes
    Effect::new(move |_| {
        let current_theme = theme_state.get();
        _ = move || set_theme_storage_state.set(current_theme.clone());
        update_css_for_theme(current_theme, prefers_dark(), use_data_attribute)
    });

    view! {
        {children()}
    }
}