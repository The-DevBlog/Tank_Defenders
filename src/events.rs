use bevy::prelude::*;

use crate::{
    resources::{Bank, MyAssets},
    BankBalanceTxt, Health, Healthbar, HealthbarBundle,
};

pub struct EventsPlugin;

impl Plugin for EventsPlugin {
    fn build(&self, app: &mut App) {
        app.observe(purchase_unit_request)
            .observe(update_bank_balance)
            .observe(update_healthbar);
    }
}

#[derive(Event)]
pub struct UpdateBankBalanceEv {
    pub amount: i32,
}

impl UpdateBankBalanceEv {
    pub fn new(amount: i32) -> Self {
        UpdateBankBalanceEv { amount }
    }
}

#[derive(Event)]
pub struct EnemyDestroyedEv;

#[derive(Event)]
pub struct AdvanceRound;

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
            let healthbar_mesh =
                meshes.add(Rectangle::from_size(Vec2::new(width, healthbar.height)));
            let healthbar_img = my_assets.full_health.clone();
            let new_healthbar = HealthbarBundle::new(
                healthbar.width,
                healthbar.height,
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
