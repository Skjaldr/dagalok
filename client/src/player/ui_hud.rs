use bevy::prelude::*;

use crate::{characters::setup_char::{Health, Target}, player::setup_player::{Player, PlayerName}};

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

pub fn local_player_name_bar(
    mut commands: Commands,
    name_query: Query<&PlayerName, With<Player>>,
) {
    let Ok(name) = name_query.single() else {
        return;
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(5.0),
            height: Val::Percent(5.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Text::new(name.0.to_string()),
        TextLayout::new_with_justify(Justify::Center),
        BackgroundColor(Color::BLACK),
    ));
}

// Playing around with the clicking colliders and storing values in the component struct for target to get a better idea
// as to how exactly this should work before going with a more finalized idea.
pub fn get_targeted_player<E: EntityEvent>(
    // mut commands: Commands,
) -> impl Fn(On<E>, Query<Entity, With<Player>>, Query<&mut Target>) {

    move |event, mut player, mut target | {
        if let Ok(mut p) = player.get_mut(event.event_target()) {
            for mut tar in target.iter_mut() {
                let t = tar.selected.get_or_insert(p);
                println!("CLICKED ON: {:?}", t);

            }
        }
    }
}

pub fn local_player_target_bar(
    mut commands: Commands,
    name_query: Query<&PlayerName, With<Target>>,
) {
    let Ok(name) = name_query.single() else {
        return;
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(10.0),
            height: Val::Percent(5.0),
            // justify_content: JustifyContent::Center,
            align_content: AlignContent::Center,
            top: Val::Percent(5.0),
            right: Val::Percent(50.0),
            left: Val::Percent(50.0),

            ..default()
        },
        Text::new(name.0.to_string()),
        TextLayout::new_with_justify(Justify::Center),
        BackgroundColor(Color::BLACK),
    ));
}

pub fn update_health_bar(
    mut bars: Query<(&mut Text, &mut Node, &HealthBarDisplayMarker)>,
    health: Query<&mut Health, With<Player>>,
) {

    let Ok(hp) = health.single() else {
        return;
    };
    // println!("Max Health is: {:?}", hp.max);
    // println!("Current health is: {:?}", hp.current);

    for (mut text, mut bar, _config) in &mut bars {
        let hp_percent = hp.current/hp.max;

        if hp.current < hp.max {
            text.0 = (&hp_percent * 100.0).trunc().to_string() + "%";
            bar.width = Val::VMax(MAX_FILL * hp_percent);

        }
    }
}
