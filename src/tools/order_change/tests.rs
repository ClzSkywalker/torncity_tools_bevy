use super::*;
use super::hash::ContentHashable;

#[derive(Debug, Clone, PartialEq)]
struct TestItem {
    id: i32,
    name: String,
    value: f32,
}

impl ContentHashable for TestItem {
    fn content_hash(&self) -> ContentHash {
        let mut hasher = super::hash::StableHasher::new();
        hasher.write_i32(self.id);
        hasher.write_str(&self.name);
        hasher.write_f32(self.value);
        hasher.finish()
    }
}

fn count_by_type<T>(items: &[OrderChangeItem<T>], change_type: ChangeType) -> usize {
    items.iter().filter(|x| x.change_type == change_type).count()
}

#[test]
fn test_no_change() {
    let items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let detector = OrderChangeDetector::new(items.clone(), items.clone());
    let report = detector.detect().unwrap();

    assert!(!report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::NoChange), 3);
}

#[test]
fn test_order_only_changed() {
    let old_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let new_items = vec![
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::OrderChanged), 3);
}

#[test]
fn test_content_only_changed() {
    let old_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
    ];

    let new_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::Added), 1);
}

#[test]
fn test_content_changed_same_position() {
    #[derive(Debug, Clone, PartialEq)]
    struct ContentChangeTestItem {
        id: i32,
        name: String,
    }

    impl ContentHashable for ContentChangeTestItem {
        fn content_hash(&self) -> ContentHash {
            let mut hasher = super::hash::StableHasher::new();
            hasher.write_i32(self.id);
            hasher.finish()
        }
    }

    let old_items = vec![
        ContentChangeTestItem { id: 1, name: "OldA".to_string() },
        ContentChangeTestItem { id: 2, name: "OldB".to_string() },
        ContentChangeTestItem { id: 3, name: "OldC".to_string() },
    ];

    let new_items = vec![
        ContentChangeTestItem { id: 1, name: "OldA".to_string() },
        ContentChangeTestItem { id: 2, name: "NewB".to_string() },
        ContentChangeTestItem { id: 3, name: "OldC".to_string() },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::ContentChanged), 1);
    assert_eq!(count_by_type(&report.items, ChangeType::NoChange), 2);

    let content_changed: Vec<_> = report.items
        .iter()
        .filter(|x| x.change_type == ChangeType::ContentChanged)
        .collect();
    assert_eq!(content_changed[0].item.id, 2);
}

#[test]
fn test_mixed_changes() {
    let old_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let new_items = vec![
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 4, name: "D".to_string(), value: 40.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::Added), 1);
    assert_eq!(count_by_type(&report.items, ChangeType::Removed), 1);
}

#[test]
fn test_removed() {
    let old_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
    ];

    let new_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::Removed), 1);
}

#[test]
fn test_content_hashable_string() {
    let s1 = "test";
    let s2 = "test";
    let s3 = "different";

    assert_eq!(s1.content_hash(), s2.content_hash());
    assert_ne!(s1.content_hash(), s3.content_hash());
}

#[test]
fn test_content_hashable_primitives() {
    assert_eq!(42i32.content_hash(), 42i32.content_hash());
    assert_ne!(42i32.content_hash(), 43i32.content_hash());

    assert_eq!(3.14f32.content_hash(), 3.14f32.content_hash());
    assert_ne!(3.14f32.content_hash(), 2.71f32.content_hash());
}

#[test]
fn test_order_change_detection() {
    let old_items = vec![
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let new_items = vec![
        TestItem { id: 2, name: "B".to_string(), value: 20.0 },
        TestItem { id: 1, name: "A".to_string(), value: 10.0 },
        TestItem { id: 3, name: "C".to_string(), value: 30.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::OrderChanged), 2);
}

#[test]
fn test_hash_stability() {
    let item = TestItem { id: 1, name: "test".to_string(), value: 1.0 };

    let hash1 = item.content_hash();
    let hash2 = item.content_hash();

    assert_eq!(hash1, hash2, "Hash should be stable across multiple calls");
}

#[test]
fn test_hash_collision_detection() {
    #[derive(Debug, Clone, PartialEq)]
    struct CollisionTestItem {
        id: i32,
        name: String,
    }

    impl ContentHashable for CollisionTestItem {
        fn content_hash(&self) -> ContentHash {
            let mut hasher = super::hash::StableHasher::new();
            hasher.write_i32(self.id);
            hasher.finish()
        }
    }

    let old_items = vec![
        CollisionTestItem { id: 1, name: "OldName".to_string() },
    ];

    let new_items = vec![
        CollisionTestItem { id: 1, name: "NewName".to_string() },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let report = detector.detect().unwrap();

    assert!(report.has_changes);
    assert_eq!(count_by_type(&report.items, ChangeType::ContentChanged), 1);
}

#[test]
fn test_float_precision_issue_in_hash() {
    #[derive(Debug, Clone, PartialEq)]
    struct FloatIssueItem {
        id: i32,
        name: String,
        float_field: f32,
    }

    impl ContentHashable for FloatIssueItem {
        fn content_hash(&self) -> ContentHash {
            let mut hasher = super::hash::StableHasher::new();
            hasher.write_i32(self.id);
            hasher.write_str(&self.name);
            hasher.write_f32(self.float_field);
            hasher.finish()
        }
    }

    let old_items = vec![
        FloatIssueItem { id: 1, name: "A".to_string(), float_field: 10.0 },
    ];

    let new_items = vec![
        FloatIssueItem { id: 1, name: "A".to_string(), float_field: 10.0 },
    ];

    let detector = OrderChangeDetector::new(old_items, new_items);
    let _report = detector.detect().unwrap();

    eprintln!("Float precision test - float_field bit pattern differs but value looks same:");
    eprintln!("  old bit pattern: {:032b}", 10.0f32.to_bits());
    eprintln!("  new bit pattern: {:032b}", 10.0f32.to_bits());
}

#[test]
fn test_trader_card_hash_without_float() {
    use crate::components::trader_card::{TraderCardData, TraderItemData};

    let card1 = TraderCardData {
        name: "FrankCastle".to_string(),
        total_profit: 11750,
        link: "https://www.torn.com".to_string(),
        items: vec![
            TraderItemData {
                image_url: "https://www.torn.com/images/items/35/large.png".to_string(),
                official: true,
                name: "Cobra Derringer".to_string(),
                quantity: 1,
                buy: 50000,
                sell: 55000,
                single_profit: 503,
                total_profit: 5003,
                percent: 3.09,
            },
        ],
        ..Default::default()
    };

    let mut card2 = card1.clone();
    card2.items[0].percent = 3.10;

    let hash1 = card1.content_hash();
    let hash2 = card2.content_hash();

    eprintln!("TraderCardData hash test:");
    eprintln!("  card1.percent = {}, hash = {:?}", card1.items[0].percent, hash1);
    eprintln!("  card2.percent = {}, hash = {:?}", card2.items[0].percent, hash2);
    eprintln!("  hashes equal: {}", hash1 == hash2);

    assert_eq!(hash1, hash2, "Hash should ignore percent field");
}

#[test]
fn test_trader_card_detect_no_change() {
    use crate::components::trader_card::{TraderCardData, TraderItemData};

    let old_cards = vec![
        TraderCardData {
            name: "FrankCastle".to_string(),
            total_profit: 11750,
            link: "https://www.torn.com".to_string(),
            items: vec![
                TraderItemData {
                    image_url: "https://www.torn.com/images/items/35/large.png".to_string(),
                    official: true,
                    name: "Cobra Derringer".to_string(),
                    quantity: 1,
                    buy: 50000,
                    sell: 55000,
                    single_profit: 503,
                    total_profit: 5003,
                    percent: 3.09,
                },
            ],
            ..Default::default()
        },
    ];

    let mut new_cards = old_cards.clone();
    new_cards[0].items[0].percent = 3.10;
    new_cards[0].items[0].single_profit = 999;

    let detector = OrderChangeDetector::new(old_cards, new_cards);
    let report = detector.detect().unwrap();

    eprintln!("TraderCardData detect no change test:");
    eprintln!("  has_changes: {}", report.has_changes);

    assert!(!report.has_changes, "Should detect no changes when only ignored fields differ");
}
