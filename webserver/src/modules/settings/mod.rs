//! Module for storing application settings

use galvyn::contrib::settings::ApplicationSettings;
use galvyn::contrib::settings::RegisterError;
use galvyn::contrib::settings::SettingsHandle;
use galvyn::contrib::settings::SettingsStore;

/// Application settings
pub struct Settings {
    /// Example value to demonstrate how to register settings
    pub example: SettingsHandle<String>,
}

impl ApplicationSettings for Settings {
    fn init(store: &mut SettingsStore) -> Result<Self, RegisterError> {
        // TODO: register all settings here
        Ok(Self {
            example: store.register("app.example", || "this is a example value".to_string())?,
        })
    }
}
