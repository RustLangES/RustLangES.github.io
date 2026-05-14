use leptos::{
    children::Children,
    leptos_dom::logging::console_log,
    logging::log,
    prelude::{use_context, RwSignal, *},
    server::codee::string::JsonSerdeCodec,
    *,
};
use leptos_use::{
    storage::use_local_storage, use_media_query, use_preferred_dark, watch_with_options,
    WatchOptions,
};
use serde::{Deserialize, Serialize};
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
    let document = web_sys::window()
        .expect("should get window")
        .document()
        .expect("document should be available");
    let html_element = document
        .document_element()
        .expect("document should have a root element");

    // TODO: Change the theme following the system preference when Theme::System is selected
    if use_data_attribute {
        match theme {
            Theme::Light => {
                html_element
                    .set_attribute("data-theme", "light")
                    .expect("should set data-theme to light");
            }
            Theme::Dark => {
                html_element
                    .set_attribute("data-theme", "dark")
                    .expect("should set data-theme to dark");
            }
            Theme::System => match prefers_dark {
                true => html_element
                    .set_attribute("data-theme", "dark")
                    .expect("should set data-theme to dark"),
                false => html_element
                    .set_attribute("data-theme", "light")
                    .expect("should set data-theme to light"),
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
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let use_data_attribute = true;
    let is_dark_preferred_signal = use_media_query("(prefers-color-scheme: dark)");

    // Attempt to retrieve the theme from local storage
    let (theme_storage_state, set_theme_storage_state, _) =
        use_local_storage::<Theme, JsonSerdeCodec>(STORAGE_KEY);

    let theme_state = RwSignal::new(theme_storage_state.get_untracked());
    provide_context(theme_state.clone());

    // Update local storage and CSS whenever the theme state changes
    Effect::new(move |_| {
        let current_theme = theme_state();
        set_theme_storage_state.set(current_theme.clone());
        update_css_for_theme(
            current_theme,
            is_dark_preferred_signal(),
            use_data_attribute,
        )
    });

    view! { {children()} }
}
