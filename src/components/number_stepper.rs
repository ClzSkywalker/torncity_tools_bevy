use bevy::{ecs::relationship::RelatedSpawner, prelude::*};

/// 数值调节器配置
#[derive(Debug, Clone)]
pub struct NumberStepperConfig {
    pub id: Option<String>,  // 可选的唯一标识符
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

/// 数值调节器的运行时状态（每个 stepper 独立维护）
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

/// 数值变化事件（外部系统可监听此事件）
#[derive(Event, Debug, Clone)]
pub struct StepperValueChanged {
    pub id: Option<String>,
    pub stepper_entity: Entity,
    pub old_value: f32,
    pub new_value: f32,
}

/// 按钮方向枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonDirection {
    Decrement,
    Increment,
}

/// 按钮标记（通过 stepper_entity 关联到父 stepper）
#[derive(Component, Debug, Clone)]
#[require(Button)]
pub struct StepperButton {
    pub stepper_entity: Entity,
    pub direction: ButtonDirection,
}

/// 数值文本标记（通过 stepper_entity 关联到父 stepper）
#[derive(Component, Debug, Clone)]
pub struct StepperValueText {
    pub stepper_entity: Entity,
}

/// 数值调节器 Spawner
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

                // 减按钮
                spawner.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.29)),
                    StepperButton {
                        stepper_entity,
                        direction: ButtonDirection::Decrement,
                    },
                    Children::spawn(SpawnWith(|spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            Text::new("-"),
                            TextFont {
                                font_size: 20.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    })),
                ));

                // 数值文本
                spawner.spawn((
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.15, 0.15, 0.19)),
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
                                TextColor(Color::WHITE),
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
                    ));
                }

                // 加按钮
                spawner.spawn((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        border_radius: BorderRadius::all(Val::Px(6.0)),
                        ..Default::default()
                    },
                    BackgroundColor(Color::srgb(0.25, 0.25, 0.29)),
                    StepperButton {
                        stepper_entity,
                        direction: ButtonDirection::Increment,
                    },
                    Children::spawn(SpawnWith(|spawner2: &mut RelatedSpawner<ChildOf>| {
                        spawner2.spawn((
                            Text::new("+"),
                            TextFont {
                                font_size: 20.0,
                                ..Default::default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    })),
                ));
            })),
        )
    }
}

/// 处理数值调节器按钮点击
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

        // 通过 stepper_entity 找到对应的 StepperState
        let Ok(mut state) = state_query.get_mut(button.stepper_entity) else {
            continue;
        };

        let old_value = state.value;

        // 根据方向计算新值
        let new_value = match button.direction {
            ButtonDirection::Decrement => (state.value - state.step).max(state.min),
            ButtonDirection::Increment => (state.value + state.step).min(state.max),
        };

        // 更新状态
        state.value = new_value;

        // 更新对应的文本显示
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

        // 发送事件
        events.trigger(StepperValueChanged {
            id: state.id.clone(),
            stepper_entity: button.stepper_entity,
            old_value,
            new_value,
        });
    }
}
