use std::collections::{HashMap, HashSet};

pub use error::*;
pub use hash::*;
pub use result::*;

pub mod hash;

mod error;
mod result;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeType {
    NoChange,
    ContentChanged,
    OrderChanged,
    Added,
    Removed,
}

#[derive(Debug, Clone)]
pub struct OrderChangeDetector<T> {
    old_data: Vec<T>,
    new_data: Vec<T>,
}

impl<T: ContentHashable + PartialEq + Clone> OrderChangeDetector<T> {
    pub fn new(old_data: Vec<T>, new_data: Vec<T>) -> Self {
        Self { old_data, new_data }
    }

    pub fn detect(&self) -> Result<OrderChangeReport<T>> {
        let mut report = OrderChangeReport::default();

        let old_map: HashMap<ContentHash, T> = self
            .old_data
            .iter()
            .map(|item| (item.content_hash(), item.clone()))
            .collect();

        let new_map: HashMap<ContentHash, T> = self
            .new_data
            .iter()
            .map(|item| (item.content_hash(), item.clone()))
            .collect();

        let old_hashes: HashSet<ContentHash> = old_map.keys().cloned().collect();
        let new_hashes: HashSet<ContentHash> = new_map.keys().cloned().collect();

        let added_hashes = &new_hashes - &old_hashes;
        let removed_hashes = &old_hashes - &new_hashes;
        let common_hashes = &old_hashes & &new_hashes;

        let old_positions: HashMap<ContentHash, usize> = self
            .old_data
            .iter()
            .enumerate()
            .map(|(idx, item)| (item.content_hash(), idx))
            .collect();

        let mut order_changed_count = 0;
        let mut content_changed_count = 0;
        let mut unchanged_count = 0;

        for (new_idx, new_item) in self.new_data.iter().enumerate() {
            let new_hash = new_item.content_hash();

            if common_hashes.contains(&new_hash) {
                let old_idx = old_positions[&new_hash];
                let old_item = &self.old_data[old_idx];

                if new_item != old_item {
                    report.items.push(OrderChangeItem {
                        item: new_item.clone(),
                        change_type: ChangeType::ContentChanged,
                    });
                    content_changed_count += 1;
                } else if old_idx != new_idx {
                    report.items.push(OrderChangeItem {
                        item: new_item.clone(),
                        change_type: ChangeType::OrderChanged,
                    });
                    order_changed_count += 1;
                } else {
                    report.items.push(OrderChangeItem {
                        item: new_item.clone(),
                        change_type: ChangeType::NoChange,
                    });
                    unchanged_count += 1;
                }
            }
        }

        let added_count = added_hashes.len();
        let removed_count = removed_hashes.len();

        for hash in &added_hashes {
            if let Some(item) = new_map.get(hash) {
                report.items.push(OrderChangeItem {
                    item: item.clone(),
                    change_type: ChangeType::Added,
                });
            }
        }

        for hash in &removed_hashes {
            if let Some(item) = old_map.get(hash) {
                report.items.push(OrderChangeItem {
                    item: item.clone(),
                    change_type: ChangeType::Removed,
                });
            }
        }

        report.added_count = added_count;
        report.removed_count = removed_count;
        report.content_changed_count = content_changed_count;
        report.order_changed_count = order_changed_count;
        report.unchanged_count = unchanged_count;
        report.has_changes = added_count > 0
            || removed_count > 0
            || content_changed_count > 0
            || order_changed_count > 0;

        Ok(report)
    }
}
