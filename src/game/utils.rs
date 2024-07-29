use crate::game::components::*;
use bevy::prelude::*;
use bevy_mod_billboard::BillboardMeshHandle;

pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, border_select_visibility);
    }
}

fn border_select_visibility(
    friendly_q: Query<(Entity, &Selected), With<Friendly>>,
    mut border_select_q: Query<(&mut BillboardMeshHandle, &BorderSelect), With<BorderSelect>>,
    children_q: Query<&Children>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (friendly_ent, selected) in friendly_q.iter() {
        for child in children_q.iter_descendants(friendly_ent) {
            if let Ok((mut billboard_mesh, border)) = border_select_q.get_mut(child) {
                let mut border_xy = Vec2::new(0.0, 0.0);
                if selected.0 {
                    border_xy = Vec2::new(border.width, border.height);
                }

                *billboard_mesh = BillboardMeshHandle(meshes.add(Rectangle::from_size(border_xy)));
            }
        }
    }
}
