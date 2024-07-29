use crate::{
    resources::MyAssets, AttackAudioEv, AttackAudioOptions, UnitAudioEv, UnitAudioOptions,
};
use bevy::{audio::PlaybackMode, prelude::*};
use rand::seq::SliceRandom;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .observe(unit_audio)
            .observe(attack_audio);
    }
}

fn setup(assets: Res<AssetServer>, mut my_assets: ResMut<MyAssets>) {
    // unit select variations
    let mut handles: Vec<Handle<AudioSource>> = Vec::new();
    handles.push(assets.load("audio/unit_select/awaiting_orders.ogg"));
    handles.push(assets.load("audio/unit_select/commander.ogg"));
    handles.push(assets.load("audio/unit_select/in_position.ogg"));
    handles.push(assets.load("audio/unit_select/on_your_mark.ogg"));
    handles.push(assets.load("audio/unit_select/orders.ogg"));
    handles.push(assets.load("audio/unit_select/reporting.ogg"));
    handles.push(assets.load("audio/unit_select/standing_by.ogg"));
    handles.push(assets.load("audio/unit_select/yes_sir.ogg"));
    handles.push(assets.load("audio/unit_select/2_awaiting_orders.ogg"));
    handles.push(assets.load("audio/unit_select/2_commander.ogg"));
    handles.push(assets.load("audio/unit_select/2_in_position.ogg"));
    handles.push(assets.load("audio/unit_select/2_on_your_mark.ogg"));
    handles.push(assets.load("audio/unit_select/2_orders.ogg"));
    handles.push(assets.load("audio/unit_select/2_standing_by.ogg"));
    my_assets.audio_unit_select.extend(handles);

    // unit move variations
    let mut handles: Vec<Handle<AudioSource>> = Vec::new();
    handles.push(assets.load("audio/unit_move/affirmative.ogg"));
    handles.push(assets.load("audio/unit_move/copy_that.ogg"));
    handles.push(assets.load("audio/unit_move/copy.ogg"));
    handles.push(assets.load("audio/unit_move/heading_out.ogg"));
    handles.push(assets.load("audio/unit_move/moving_out.ogg"));
    handles.push(assets.load("audio/unit_move/moving.ogg"));
    handles.push(assets.load("audio/unit_move/on_the_move.ogg"));
    handles.push(assets.load("audio/unit_move/on_it.ogg"));
    handles.push(assets.load("audio/unit_move/roger_that.ogg"));
    handles.push(assets.load("audio/unit_move/roger.ogg"));
    handles.push(assets.load("audio/unit_move/understood.ogg"));
    handles.push(assets.load("audio/unit_move/will_comply.ogg"));
    handles.push(assets.load("audio/unit_move/willco.ogg"));
    handles.push(assets.load("audio/unit_move/2_affirmative.ogg"));
    handles.push(assets.load("audio/unit_move/2_copy_that.ogg"));
    handles.push(assets.load("audio/unit_move/2_heading_out.ogg"));
    handles.push(assets.load("audio/unit_move/2_moving_out.ogg"));
    handles.push(assets.load("audio/unit_move/2_moving.ogg"));
    handles.push(assets.load("audio/unit_move/2_on_the_move.ogg"));
    handles.push(assets.load("audio/unit_move/2_on_it.ogg"));
    handles.push(assets.load("audio/unit_move/2_roger.ogg"));
    handles.push(assets.load("audio/unit_move/2_understood.ogg"));
    handles.push(assets.load("audio/unit_move/2_will_comply.ogg"));
    handles.push(assets.load("audio/unit_move/2_willco.ogg"));
    my_assets.audio_unit_move.extend(handles);

    // unit attack variations
    let mut handles: Vec<Handle<AudioSource>> = Vec::new();
    handles.push(assets.load("audio/unit_attack/consider_it_done.ogg"));
    handles.push(assets.load("audio/unit_attack/enemy_in_sight.ogg"));
    handles.push(assets.load("audio/unit_attack/engaging.ogg"));
    handles.push(assets.load("audio/unit_attack/fire_at_will.ogg"));
    handles.push(assets.load("audio/unit_attack/open_fire.ogg"));
    handles.push(assets.load("audio/unit_attack/target_acquired.ogg"));
    handles.push(assets.load("audio/unit_attack/weapons_hot.ogg"));
    handles.push(assets.load("audio/unit_attack/2_consider_it_done.ogg"));
    handles.push(assets.load("audio/unit_attack/2_enemy_in_sight.ogg"));
    handles.push(assets.load("audio/unit_attack/2_engaging.ogg"));
    handles.push(assets.load("audio/unit_attack/2_fire_at_will.ogg"));
    handles.push(assets.load("audio/unit_attack/2_open_fire.ogg"));
    handles.push(assets.load("audio/unit_attack/2_target_acquired.ogg"));
    handles.push(assets.load("audio/unit_attack/2_weapons_hot.ogg"));
    my_assets.audio_unit_attack.extend(handles);

    // attack sounds
    my_assets.audio_rifle_fire = assets.load("audio/rifle_fire.ogg");
    my_assets.audio_tank_fire = assets.load("audio/tank_fire.ogg")
}

fn attack_audio(trigger: Trigger<AttackAudioEv>, mut cmds: Commands, my_assets: Res<MyAssets>) {
    let mut bundle = AudioBundle::default();
    bundle.settings.mode = PlaybackMode::Despawn;

    bundle.source = match trigger.event().0 {
        AttackAudioOptions::Soldier => my_assets.audio_rifle_fire.clone(),
        AttackAudioOptions::Tank => my_assets.audio_tank_fire.clone(),
    };

    cmds.spawn(bundle);
}

fn unit_audio(trigger: Trigger<UnitAudioEv>, mut cmds: Commands, my_assets: Res<MyAssets>) {
    let mut bundle = AudioBundle::default();
    bundle.settings.mode = PlaybackMode::Despawn;

    let handles = match trigger.event().0 {
        UnitAudioOptions::Attack => my_assets.audio_unit_attack.clone(),
        UnitAudioOptions::Relocate => my_assets.audio_unit_move.clone(),
        UnitAudioOptions::Select => my_assets.audio_unit_select.clone(),
    };

    if let Some(handle) = handles.choose(&mut rand::thread_rng()) {
        bundle.source = handle.clone();
        cmds.spawn(bundle);
    } else {
        eprintln!("Audio Handle Missing.");
    }
}
