use bevy::prelude::*;

use crate::{characters::setup_char::{Health, Target, Targettable}, player::setup_player::{Player, PlayerName}};

#[derive(Component)]
pub struct HealthBarDisplayMarker;

#[derive(Component)]
pub struct TargetHealthBarDisplayMarker;

#[derive(Component)]
pub struct TargetDisplayMarker;


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
            border_radius: BorderRadius::all(Val::VMax(5.0)),

            ..default()
        },
        Text::new(name.0.to_string()),
        TextLayout::new_with_justify(Justify::Center),
        BackgroundColor(Color::BLACK),
    ));
}

pub fn local_player_target_bar(
    mut commands: Commands,
    name_query: Query<Entity, With<Target>>,
) {
    for name in name_query.iter() {
        commands.spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(15.0),
                height: Val::Percent(6.0),
                // align_content: AlignContent::Center,
                justify_self: JustifySelf::Center,
                border_radius: BorderRadius::all(Val::VMax(5.0)),
                ..default()
            },
        ))
        .with_children((|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_content: AlignContent::Center,
                    justify_self: JustifySelf::Center,
                    border_radius: BorderRadius::all(Val::VMax(5.0)),
                    ..default()
                },
                Text::new(format!("Target: {:?}", name.to_string()).to_string()),
                TextLayout::new_with_justify(Justify::Center),
                BackgroundColor(Color::BLACK),
                TargetDisplayMarker,
            ));

            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(50.0),
                    height: Val::Percent(50.0),
                    bottom: Val::Percent(10.0),
                    // align_content: AlignContent::End,
                    // justify_self: JustifySelf::End,
                    ..default()
                },
                Text::new("100%"),
                BackgroundColor(HEALTH_COLOR),
                TargetHealthBarDisplayMarker,
            ));
        }));
    }
}

pub fn update_target_bar(
    mut bars: Query<(&mut Text, &mut Node), Or<(With<TargetDisplayMarker>, With<TargetHealthBarDisplayMarker>)>>,
    // targettable: Query<&Targettable>,
    // mut healthbar: Query<(&mut Text, &mut Node, &TargetHealthBarDisplayMarker)>,
    mut health: Query<&mut Health>,
    tar: Query<&Target>,
    // mut commands: Commands,
) {


    for (mut text, mut bar) in &mut bars {

        for targ in tar.iter() {
            if let Some(target) = targ.0 {
                text.0 = format!("Target is: {:?}", target).to_string();



                // if let Ok(hp) = health.get(target) {
                //     let hp_percent = hp.current/hp.max;
                //     if hp.current < hp.max {
                //         text.0 = (&hp_percent * 100.0).trunc().to_string() + "%";
                //         bar.width = Val::VMax(MAX_FILL * hp_percent);

                //     }
                // }

            }
        }
    }


}

pub fn update_health_bar(
    mut bars: Query<(&mut Text, &mut Node, &HealthBarDisplayMarker)>,
    health: Query<&mut Health, With<Player>>,
) {

    let Ok(hp) = health.single() else {
        return;
    };

    for (mut text, mut bar, _config) in &mut bars {
        let hp_percent = hp.current/hp.max;

        if hp.current < hp.max {
            text.0 = (&hp_percent * 100.0).trunc().to_string() + "%";
            bar.width = Val::VMax(MAX_FILL * hp_percent);

        }
    }
}


// When a player clicks another character on screen, NPC or player, the current selected target should be stored in some variable
// this way if the player decides to heal or damage the target, we can determine which target will receive the heals or damage
//
// When a player clicks a selectable object (enemy, friendly, npc, other player), we trigger an event saying "Hey, this guy clicked this thing. Store this thing
// for immediate or later use".  So the first thing that needs to happen is registering the click.  Right now colliders are the only real option to use for picking.
// What I need to understand and internalize is how exactly to click different entities.

pub fn get_target<E: EntityEvent>(
    // mut commands: Commands,
) -> impl Fn(On<E>, Query<Entity, With<Targettable>>, Query<&mut Target>, Commands) {

    move |event, targettable, mut target, mut commands| {
        // get the entity that was clicked
        let Ok(mut tar) = target.single_mut() else {
            return;
        };

        if let Ok(entity) = targettable.get(event.event_target()) {
            tar.0 = Some(entity);
            println!("Target is: {:?}", tar.0);

        }


    }
}
