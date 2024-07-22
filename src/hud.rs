use crate::resources::Bank;
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud);
    }
}

fn spawn_hud(mut cmds: Commands, bank: Res<Bank>, assets: Res<AssetServer>) {
    let bank_container = (
        NodeBundle {
            background_color: Color::srgb(0.0, 0.0, 0.0).into(),
            style: Style {
                width: Val::Auto,
                height: Val::Auto,
                padding: UiRect::all(Val::Px(10.0)),
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::End,
                margin: UiRect::left(Val::Auto),
                ..default()
            },
            ..default()
        },
        Name::new("Bank"),
    );

    let bank_balance = format!("${}", bank.0.to_string());
    let bank_balance_txt = (
        TextBundle::from_section(
            bank_balance,
            TextStyle {
                color: Color::srgb(1.0, 1.0, 0.0),
                font_size: 25.0,
                ..default()
            },
        ),
        Name::new("Bank Balance"),
    );

    let buy_soldier_btn = (
        ButtonBundle {
            background_color: Color::srgb(0.0, 1.0, 0.0).into(),
            style: Style {
                width: Val::Px(50.0),
                height: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        Name::new("Buy Solider Button"),
    );

    let buy_soldier_img = (
        ImageBundle {
            image: assets.load("imgs/buy_soldier.png").into(),
            style: Style {
                height: Val::Px(50.0),
                width: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        Name::new("Image"),
    );

    cmds.spawn(bank_container).with_children(|parent| {
        parent.spawn(bank_balance_txt);
        parent.spawn(buy_soldier_btn).with_children(|parent| {
            parent.spawn(buy_soldier_img);
        });
    });
}
