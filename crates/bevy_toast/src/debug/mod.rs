#[cfg(feature = "dev")]
pub fn show_debug_toast(world: &mut bevy_ecs::prelude::World, text: &str) {
    use crate::events::ToastEvent;
    world.send_event(ToastEvent::text(text));
}

#[cfg(feature = "dev")]
pub fn show_all_types_toast(world: &mut bevy_ecs::prelude::World) {
    use crate::events::ToastEvent;
    world.send_event(ToastEvent::success("Success Toast"));
    world.send_event(ToastEvent::error("Error Toast"));
    world.send_event(ToastEvent::warning("Warning Toast"));
    world.send_event(ToastEvent::info("Info Toast"));
}

#[cfg(feature = "dev")]
pub fn toast_stress_test(world: &mut bevy_ecs::prelude::World, count: usize) {
    use crate::events::ToastEvent;
    for i in 0..count {
        let text = format!("Stress test #{}", i + 1);
        world.send_event(ToastEvent::text(text));
    }
}
