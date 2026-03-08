use crate::components::trader_card::{TraderCardData, TraderCardSpawner};
use crate::tools::prelude::*;
use bevy::prelude::*;

const MAX_DISPLAY_CARDS: usize = 20;

#[derive(Resource, Default)]
pub struct CurrentTraderCards {
    pub cards: Vec<TraderCardData>,
}

#[derive(Component)]
pub struct TraderCardDataComponent {
    pub data: TraderCardData,
}

#[derive(Component)]
pub struct TraderCardScrollMarker;

impl ContentHashable for TraderCardData {
    fn content_hash(&self) -> ContentHash {
        let mut hasher = StableHasher::new();
        hasher.write_str(&self.name);
        hasher.write_i64(self.total_profit);
        hasher.write_str(&self.link);

        for item in &self.items {
            hasher.write_str(&item.image_url);
            hasher.write_u32(item.official as u32);
            hasher.write_str(&item.name);
            hasher.write_u32(item.quantity);
            hasher.write_i64(item.buy);
            hasher.write_i64(item.sell);
        }

        hasher.finish()
    }
}

pub fn handle_trader_card_update(
    cmd: &mut Commands,
    parent: Entity,
    current_cards: &mut CurrentTraderCards,
    new_data: Vec<TraderCardData>,
) -> bool {
    let display_data: Vec<TraderCardData> = new_data.into_iter().take(MAX_DISPLAY_CARDS).collect();

    if display_data.is_empty() {
        cmd.entity(parent).despawn_children();
        current_cards.cards.clear();
        return false;
    }

    let old_cards = current_cards.cards.clone();

    let display_data_clone = display_data.clone();
    let detector = OrderChangeDetector::new(old_cards, display_data_clone);

    match detector.detect() {
        Ok(report) => {
            if !report.has_changes {
                bevy::log::info!("handle_trader_card_update: no changes detected");
                return false;
            }

            bevy::log::info!(
                "handle_trader_card_update: changes detected, added: {}, changed: {}, removed: {}",
                report.added_count,
                report.content_changed_count,
                report.removed_count,
            );

            apply_changes(cmd, parent, &display_data);
            current_cards.cards = display_data;
            return report.added_count > 0;
        }
        Err(e) => {
            bevy::log::error!(
                "trader card detection failed: {:?}, fallback to full rebuild",
                e
            );
            apply_changes(cmd, parent, &display_data);
            current_cards.cards = display_data;
        }
    }

    false
}

fn apply_changes(cmd: &mut Commands, parent: Entity, new_data: &[TraderCardData]) {
    cmd.entity(parent).despawn_children();
    spawn_all_cards(cmd, parent, new_data);
}

fn spawn_all_cards(cmd: &mut Commands, parent: Entity, data: &[TraderCardData]) {
    let placeholder = Handle::<Image>::default();
    let bundles = data
        .iter()
        .map(|item| {
            let data_clone = item.clone();
            let bundle = TraderCardSpawner::new(item.clone(), placeholder.clone()).bundle();
            (bundle, TraderCardDataComponent { data: data_clone })
        })
        .collect::<Vec<_>>();
    cmd.entity(parent).with_children(|builder| {
        for bundle in bundles {
            builder.spawn(bundle);
        }
    });
}
