use bevy::prelude::*;

use crate::{resources::Bank, BankBalanceTxt, Barracks, Commandable, HealthbarBundle, UnitBundle};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(purchase_unit_request)
            .observe(update_bank_balance)
            .observe(build_unit);
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
    let soldier_scene = assets.load("soldier.glb#Scene0");
    let mut soldier = (
        UnitBundle::new(
            "Soldier".to_string(),
            5000.0,
            Vec3::new(2., 2., 2.),
            50,
            soldier_scene,
            Vec3::new(pos.x - 30.0, 1.0, pos.z + 20.0),
        ),
        Commandable,
    );

    soldier.0.destination.0 = Some(Vec3::new(pos.x - 100.0, 1.0, pos.z + 60.0));

    let healthbar_mesh = meshes.add(Rectangle::from_size(Vec2::new(
        soldier.0.scene_bundle.transform.scale.x * 5.0,
        1.0,
    )));
    let healthbar_img = assets.load("imgs/full_health.png");
    let healthbar = HealthbarBundle::new(Vec3::new(0.0, 4.5, 0.0), healthbar_img, healthbar_mesh);

    cmds.spawn(soldier).with_children(|parent| {
        parent.spawn(healthbar);
    });
    println!("Building Unit");
}
