use crate::{resources::Bank, BankBalanceTxt, BuySoldierBtn, BuyTankBtn, SOLDIER_COST, TANK_COST};
use bevy::prelude::*;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hud);
    }
}

fn spawn_hud(mut cmds: Commands, bank: Res<Bank>, assets: Res<AssetServer>) {
    let hud_container = (
        NodeBundle {
            border_color: BorderColor {
                0: Color::srgb(0.2, 0.19, 0.18),
                ..default()
            },
            border_radius: BorderRadius::left(Val::Px(6.0)),
            background_color: Color::srgb(0.28, 0.31, 0.26).into(),
            style: Style {
                border: UiRect {
                    top: Val::Px(17.0),
                    left: Val::Px(17.0),
                    ..default()
                },
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
        Name::new("Hud"),
    );

    let bank_balance = format!("${}", bank.0.to_string());
    let bank_balance_txt = (
        TextBundle {
            text: Text::from_section(
                bank_balance,
                TextStyle {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    font_size: 25.0,
                    ..default()
                },
            ),
            style: Style {
                align_self: AlignSelf::End,
                ..default()
            },
            ..default()
        },
        BankBalanceTxt,
        Name::new("Bank Balance"),
    );

    let buy_soldier_container = (
        NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        Name::new("Buy Solider Container"),
    );

    let buy_solider_txt = (
        TextBundle {
            text: Text::from_section(
                format!("${}", SOLDIER_COST),
                TextStyle {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    font_size: 25.0,
                    ..default()
                },
            ),
            style: Style {
                margin: UiRect {
                    top: Val::Auto,
                    bottom: Val::Auto,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        Name::new("Text"),
    );

    let buy_soldier_img = (
        ButtonBundle {
            image: assets.load("imgs/buy_soldier.png").into(),
            style: Style {
                height: Val::Px(50.0),
                width: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        BuySoldierBtn,
        Name::new("Image"),
    );

    let buy_tank_container = (
        NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        Name::new("Buy Tank Container"),
    );

    let buy_tank_txt = (
        TextBundle {
            text: Text::from_section(
                format!("${}", TANK_COST),
                TextStyle {
                    color: Color::srgb(1.0, 1.0, 0.0),
                    font_size: 25.0,
                    ..default()
                },
            ),
            style: Style {
                margin: UiRect {
                    top: Val::Auto,
                    bottom: Val::Auto,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        Name::new("Text"),
    );

    let buy_tank_img = (
        ButtonBundle {
            image: assets.load("imgs/buy_tank.png").into(),
            style: Style {
                height: Val::Px(50.0),
                width: Val::Px(50.0),
                ..default()
            },
            ..default()
        },
        BuyTankBtn,
        Name::new("Image"),
    );

    cmds.spawn(hud_container).with_children(|parent| {
        parent.spawn(bank_balance_txt);
        parent.spawn(buy_soldier_container).with_children(|parent| {
            parent.spawn(buy_solider_txt);
            parent.spawn(buy_soldier_img);
        });
        parent.spawn(buy_tank_container).with_children(|parent| {
            parent.spawn(buy_tank_txt);
            parent.spawn(buy_tank_img);
        });
    });
}
