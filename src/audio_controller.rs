use crate::{Action, AudioTest, CurrentAction, MyAssets, Unit};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct AudioControllerPlugin;

impl Plugin for AudioControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, audio_controller);
    }
}

fn audio_controller(
    unit_q: Query<(Entity, &CurrentAction, &MyAssets, &AudioTest), With<Unit>>,
    // mut audio_q: Query<&mut PlaybackSettings>,
    audio: Res<Audio>, // children_q: Query<&Children>,
    // assets: Res<AssetServer>,
    assets: Res<AssetServer>,
) {
    for (ent, action, my_assets, audio_test) in unit_q.iter() {
        if let Some(a) = &audio_test.0 {
            // a.
        }
        // if let Ok(mut my_audio) = audio_q.get_mut(ent) {
        // audio.paused = false;

        // println!("ACTION: {:?}", action.0);

        // let Some(my_audio) = my_assets.shoot_audio.clone() else {
        //     return;
        // };

        // my_assets.shoot_audio.unwrap();
        // let audio_source = my_assets.shoot_audio.clone().unwrap();
        // audio.play(my_audio);
        // audio.play(my_assets.shoot_audio.unwrap());
        // audio.play(audio_source)
        // let t: Handle<Source> = assets.load("audio/rifle_fire.ogg");
        println!("MADE IT");
        // audio.play(audio_source)
        // Handle<Audio>

        // bevy_kira_audio::prelude::AudioSou
        match action.0 {
            Action::Attack => {
                // audio.play(assets.load("audio./rifle_fire.ogg"));
                // audio.paused = false;
                // println!("shooting");
                // println!("Audio for Attack action: {:?}", audio);
            }
            Action::Relocate => {
                // audio.paused = false;
                // println!("Audio for relocate action: {:?}", audio);

                // println!("relocating")
            }
            Action::None => {
                // audio.paused = true;
            }
        }
        // }
    }
}
