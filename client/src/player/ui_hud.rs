use bevy::prelude::*;

use crate::{characters::setup_char::Health, player::setup_player::Player};

#[derive(Component)]
pub struct HealthBarDisplayMarker;

const HEALTH_COLOR: Color = Color::linear_rgb(1.0, 0.2, 0.2);
const MIN_FILL: f32 = 29.75/6.;
const MAX_FILL: f32 = 29.75 - MIN_FILL;

pub fn spawn_health_bar(
    mut commands: Commands,
) {
     commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::VMax(MAX_FILL),
            height: Val::VMax(5.0),
            left: Val::Percent(10.0),
            bottom: Val::Percent(10.0),
            // border_radius: BorderRadius::all(Val::VMax(5.0)),
            ..default()
        },
        // HealthBarDisplayMarker,
        BackgroundColor(Color::linear_rgb(0.5, 0.5, 0.5)),
    ))
     .with_child((
         Node {
             position_type: PositionType::Absolute,
             min_width: Val::Percent(0.0),
             // width: Val::VMax(MAX_FILL),
             width: Val::Percent(99.0),
             height: Val::Percent(95.),
             margin: UiRect::all(Val::VMax(0.125)),
             // border_radius: BorderRadius::all(Val::VMax(5.0)),
             ..default()
         },
         Text::new("100%"),
         // TextLayout::new_with_justify(Justify::Center),
         BackgroundColor(HEALTH_COLOR),
         HealthBarDisplayMarker,
     ));
}



pub fn update_health_bar(
    mut bars: Query<(&mut Text, &mut Node, &HealthBarDisplayMarker)>,
    health: Query<&mut Health, With<Player>>,
) {

    let Ok(hp) = health.single() else {
        return;
    };
    println!("Max Health is: {:?}", hp.max);
    println!("Current health is: {:?}", hp.current);

    for (mut text, mut bar, _config) in &mut bars {
        let hp_percent = hp.current/hp.max;

        if hp.current < hp.max {
            text.0 = (&hp_percent * 100.0).trunc().to_string() + "%";
            bar.width = Val::VMax(MAX_FILL * hp_percent);

        }
    }
}
