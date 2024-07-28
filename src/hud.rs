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
        ImageBundle {
            image: assets.load("imgs/hud_container.png").into(),
            style: Style {
                width: Val::Percent(12.5),
                height: Val::Percent(30.0),
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
                    font_size: 37.0,
                    ..default()
                },
            ),
            style: Style {
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Percent(10.0), Val::Auto),
                align_self: AlignSelf::End,
                ..default()
            },
            ..default()
        },
        BankBalanceTxt,
        Name::new("Bank Balance"),
    );

    let buy_soldier_container = (
        ButtonBundle {
            image: UiImage::new(assets.load("imgs/hud_button.png")),
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(31.0),
                margin: UiRect::new(Val::Percent(14.0), Val::Auto, Val::Percent(10.0), Val::Auto),
                justify_content: JustifyContent::SpaceEvenly,
                ..default()
            },
            ..default()
        },
        BuySoldierBtn,
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
        ImageBundle {
            image: assets.load("imgs/buy_soldier.png").into(),
            style: Style {
                height: Val::Percent(70.0),
                width: Val::Percent(50.0),
                margin: UiRect::vertical(Val::Auto),
                ..default()
            },
            ..default()
        },
        Name::new("Image"),
    );

    let buy_tank_container = (
        ButtonBundle {
            image: UiImage::new(assets.load("imgs/hud_button.png")),
            style: Style {
                width: Val::Percent(80.0),
                height: Val::Percent(31.0),
                margin: UiRect::new(Val::Percent(14.0), Val::Auto, Val::Percent(10.0), Val::Auto),
                justify_content: JustifyContent::SpaceEvenly,

                ..default()
            },
            ..default()
        },
        BuyTankBtn,
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
        ImageBundle {
            image: assets.load("imgs/buy_tank.png").into(),
            style: Style {
                height: Val::Percent(70.0),
                width: Val::Percent(50.0),
                margin: UiRect::vertical(Val::Auto),
                ..default()
            },
            ..default()
        },
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
