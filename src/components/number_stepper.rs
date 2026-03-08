use bevy::{ecs::relationship::RelatedSpawner, prelude::*};
use bevy_theme::prelude::*;

#[derive(Debug, Clone)]
pub struct NumberStepperConfig {
    pub id: Option<String>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub decimal_places: usize,
    pub unit: Option<String>,
}

impl Default for NumberStepperConfig {
    fn default() -> Self {
        Self {
            id: None,
            value: 4.0,
            min: 0.0,
            max: 100.0,
            step: 0.5,
            decimal_places: 1,
            unit: None,
        }
    }
}

#[derive(Component, Debug, Clone)]
pub struct StepperState {
    pub id: Option<String>,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub decimal_places: usize,
}

impl From<NumberStepperConfig> for StepperState {
    fn from(config: NumberStepperConfig) -> Self {
        Self {
            id: config.id,
            value: config.value,
            min: config.min,
            max: config.max,
            step: config.step,
            decimal_places: config.decimal_places,
        }
    }
}

#[derive(Event, Debug, Clone)]
pub struct StepperValueChanged {
    pub id: Option<String>,
    pub stepper_entity: Entity,
    pub old_value: f32,
    pub new_value: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonDirection {
    Decrement,
    Increment,
}

#[derive(Component, Debug, Clone)]
#[require(Button)]
pub struct StepperButton {
    pub stepper_entity: Entity,
    pub direction: ButtonDirection,
}

#[derive(Component, Debug, Clone)]
pub struct StepperValueText {
    pub stepper_entity: Entity,
}

pub struct NumberStepperSpawner {
    pub config: NumberStepperConfig,
}

impl NumberStepperSpawner {
    pub fn new(config: NumberStepperConfig) -> Self {
        Self { config }
    }

    pub fn bundle(self) -> impl Bundle {
        let decimal_places = self.config.decimal_places;
        let value_text = format!("{:.prec$}", self.config.value, prec = decimal_places);
        let unit = self.config.unit.clone();

        (
            Node {
                width: Val::Auto,
                height: Val::Px(40.0),
                flex_direction: FlexDirection::Row,
                column_gap: Val::Px(8.0),
                align_items: AlignItems::Center,
                ..Default::default()
            },
            StepperState::from(self.config),
            Children::spawn(SpawnWith(move |spawner: &mut RelatedSpawner<ChildOf>| {
                let stepper_entity = spawner.target_entity();

                spawner.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    ThemedPrimaryButton,
                    BackgroundColor(Color::BLACK),
                    StepperButton {
                        stepper_entity,
                        direction: ButtonDirection::Decrement,
                    },
                    Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            Text::new("-"),
                            TextFont {
                                font_size: 20.0,
                                ..Default::default()
                            },
                            ThemedText::primary(),
                            TextColor(Color::BLACK),
                        ));
                    })),
                ));

                spawner.spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    ThemedBackground::primary(),
                    BackgroundColor(Color::BLACK),
                    StepperValueText { stepper_entity },
                    Children::spawn(SpawnWith({
                        let value_text = value_text.clone();
                        move |spawner2: &mut RelatedSpawner<ChildOf>| {
                            spawner2.spawn((
                                Text::new(value_text.clone()),
                                TextFont {
                                    font_size: 16.0,
                                    ..Default::default()
                                },
                                ThemedText::primary(),
                                TextColor(Color::BLACK),
                            ));
                        }
                    })),
                ));

                if let Some(unit) = unit {
                    spawner.spawn((
                        Text::new(unit),
                        TextFont {
                            font_size: 16.0,
                            ..Default::default()
                        },
                        ThemedText::primary(),
                        TextColor(Color::BLACK),
                    ));
                }

                spawner.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    ThemedPrimaryButton,
                    BackgroundColor(Color::BLACK),
                    StepperButton {
                        stepper_entity,
                        direction: ButtonDirection::Increment,
                    },
                    Children::spawn(SpawnWith(move |spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            Text::new("+"),
                            TextFont {
                                font_size: 20.0,
                                ..Default::default()
                            },
                            ThemedText::primary(),
                            TextColor(Color::BLACK),
                        ));
                    })),
                ));
            })),
        )
    }
}

pub fn handle_stepper_buttons(
    button_query: Query<(&Interaction, &StepperButton), (Changed<Interaction>, With<Button>)>,
    mut state_query: Query<&mut StepperState>,
    text_query: Query<(&StepperValueText, &Children)>,
    mut text_content_query: Query<&mut Text>,
    mut events: Commands,
) {
    for (interaction, button) in &button_query {
        if *interaction != Interaction::Pressed {
            continue;
        }

        let Ok(mut state) = state_query.get_mut(button.stepper_entity) else {
            continue;
        };

        let old_value = state.value;

        let new_value = match button.direction {
            ButtonDirection::Decrement => (state.value - state.step).max(state.min),
            ButtonDirection::Increment => (state.value + state.step).min(state.max),
        };

        state.value = new_value;

        let decimal_places = state.decimal_places;
        for (stepper_text, children) in &text_query {
            if stepper_text.stepper_entity == button.stepper_entity {
                for child in children.iter() {
                    if let Ok(mut text) = text_content_query.get_mut(child) {
                        **text = format!("{:.prec$}", new_value, prec = decimal_places);
                    }
                }
            }
        }

        events.trigger(StepperValueChanged {
            id: state.id.clone(),
            stepper_entity: button.stepper_entity,
            old_value,
            new_value,
        });
    }
}
