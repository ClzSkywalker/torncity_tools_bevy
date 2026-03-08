use super::ChangeType;

#[derive(Debug, Clone)]
pub struct OrderChangeReport<T> {
    pub items: Vec<OrderChangeItem<T>>,
    pub has_changes: bool,
    pub added_count: usize,
    pub removed_count: usize,
    pub content_changed_count: usize,
    pub order_changed_count: usize,
    pub unchanged_count: usize,
}

impl<T> Default for OrderChangeReport<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            has_changes: false,
            added_count: 0,
            removed_count: 0,
            content_changed_count: 0,
            order_changed_count: 0,
            unchanged_count: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OrderChangeItem<T> {
    pub item: T,
    pub change_type: ChangeType,
}
