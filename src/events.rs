use bevy::prelude::*;

use crate::{
    resources::{Bank, MyAssets},
    BankBalanceTxt, Barracks, Friendly, Health, Healthbar, HealthbarBundle, UnitBundle,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(purchase_unit_request)
            .observe(update_bank_balance)
            .observe(build_unit)
            .observe(update_healthbar);
    }
}

#[derive(Event)]
pub struct UpdateBankBalanceEv {
    pub amount: i32,
}

impl UpdateBankBalanceEv {
    fn new(amount: i32) -> Self {
        UpdateBankBalanceEv { amount }
    }
}

#[derive(Event)]
pub struct InvokeDamage {
    pub amount: f32,
    pub target: Entity,
}

impl InvokeDamage {
    pub fn new(amount: f32, target: Entity) -> Self {
        Self { amount, target }
    }
}

#[derive(Event)]
pub struct BuildUnitEv;

#[derive(Event)]
pub struct PurchaseSoldierRequestEv {
    pub price: i32,
}

impl PurchaseSoldierRequestEv {
    pub fn new(price: i32) -> Self {
        PurchaseSoldierRequestEv { price }
    }
}

fn purchase_unit_request(
    trigger: Trigger<PurchaseSoldierRequestEv>,
    bank: Res<Bank>,
    mut cmds: Commands,
) {
    let unit_price = trigger.event().price;
    if bank.0 >= unit_price {
        cmds.trigger(UpdateBankBalanceEv::new(-unit_price));
        cmds.trigger(BuildUnitEv);
    }
}

fn update_bank_balance(
    trigger: Trigger<UpdateBankBalanceEv>,
    mut txt_q: Query<&mut Text, With<BankBalanceTxt>>,
    mut bank: ResMut<Bank>,
) {
    bank.0 = bank.0 + trigger.event().amount;
    // println!("Bank balance: {}", bank.0);
    let new_balance = format!("${}", bank.0);
    if let Ok(mut txt) = txt_q.get_single_mut() {
        txt.sections[0].value = new_balance;
    }
}

fn build_unit(
    _trigger: Trigger<BuildUnitEv>,
    barracks_q: Query<&Transform, With<Barracks>>,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut cmds: Commands,
) {
    let Ok(barracks_transform) = barracks_q.get_single() else {
        return;
    };

    let pos = barracks_transform.translation;
    let mut soldier = (
        UnitBundle::new(
            "Soldier".to_string(),
            5000.0,
            5.0,
            Vec3::new(2., 2., 2.),
            50.0,
            Timer::from_seconds(0.25, TimerMode::Repeating),
            assets.load("soldier_animations.glb#Scene0"),
            Vec3::new(pos.x - 30.0, 1.0, pos.z + 20.0),
        ),
        Friendly,
    );

    soldier.0.audio.source = assets.load("audio/rifle_fire.ogg");
    soldier.0.destination.0 = Some(Vec3::new(pos.x - 100.0, 1.0, pos.z + 60.0));

    let healthbar_width = 5.0;
    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(healthbar_width, 1.0)));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(
        healthbar_width,
        Vec3::new(0.0, 4.5, 0.0),
        healthbar_img,
        healthbar_mesh,
    );

    cmds.spawn(soldier).with_children(|parent| {
        parent.spawn(healthbar);
    });
    println!("Building Unit");
}

fn update_healthbar(
    trigger: Trigger<InvokeDamage>,
    health_q: Query<&Health>,
    healthbar_q: Query<&Healthbar>,
    mut cmds: Commands,
    children_q: Query<&Children>,
    mut meshes: ResMut<Assets<Mesh>>,
    my_assets: Res<MyAssets>,
) {
    let Ok(health) = health_q.get(trigger.event().target) else {
        return;
    };
    // println!("HEALTH:  {}", health.original);

    for child in children_q.iter_descendants(trigger.event().target) {
        if let Ok(healthbar) = healthbar_q.get(child) {
            let width = healthbar.width / (health.original / health.current);
            let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(width, 1.5)));
            let healthbar_img = my_assets.full_health.clone();
            let new_healthbar = HealthbarBundle::new(
                healthbar.width,
                Vec3::new(0.0, healthbar.y_position, 0.0),
                healthbar_img,
                healthbar_mesh,
            );

            cmds.entity(child).despawn();
            cmds.entity(trigger.event().target).with_children(|parent| {
                parent.spawn(new_healthbar);
            });
        }
    }
}
