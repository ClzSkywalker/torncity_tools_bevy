pub mod tab;

use bevy_app::prelude::*;
use tab::{
    ActiveTab, ViewTabConfig, build_tab_view, switch_active_tab, sync_active_tab_style,
    update_tab_visibility,
};

#[derive(Default)]
pub struct BevyTabPlugin {
    config: ViewTabConfig,
}

impl BevyTabPlugin {
    pub fn new(config: ViewTabConfig) -> Self {
        Self { config }
    }
}

impl Plugin for BevyTabPlugin {
    fn build(&self, app: &mut App) {
        let config = self.config.clone().normalized();
        let initial_tab = config.initial_tab.clone();

        app.insert_resource(config)
            .insert_resource(ActiveTab(initial_tab))
            .add_systems(Startup, build_tab_view)
            .add_systems(
                Update,
                (
                    switch_active_tab,
                    update_tab_visibility,
                    sync_active_tab_style,
                ),
            );
    }
}