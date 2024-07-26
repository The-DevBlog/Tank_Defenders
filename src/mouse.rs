use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier3d::{pipeline::QueryFilter, plugin::RapierContext, render::ColliderDebugColor};
use bevy_rts_camera::RtsCamera;

use crate::{
    resources::{BoxCoords, CursorState, CustomCursor, GameCommands, MouseCoords},
    soldiers::set_unit_destination,
    Friendly, MapBase, Selected,
};

pub struct MousePlugin;

impl Plugin for MousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                set_mouse_coords,
                set_box_coords,
                set_drag_select,
                drag_select,
                single_select,
                set_selected,
                deselect_all,
            )
                .chain()
                .after(set_unit_destination),
        )
        .add_systems(Update, change_cursor);
    }
}

fn set_drag_select(box_coords: Res<BoxCoords>, mut game_cmds: ResMut<GameCommands>) {
    let drag_threshold = 2.5;
    let width_z = (box_coords.global_start.z - box_coords.global_end.z).abs();
    let width_x = (box_coords.global_start.x - box_coords.global_end.x).abs();

    game_cmds.drag_select = width_z > drag_threshold || width_x > drag_threshold;
}

fn set_box_coords(
    mut box_coords: ResMut<BoxCoords>,
    input: Res<ButtonInput<MouseButton>>,
    mouse_coords: Res<MouseCoords>,
) {
    if input.just_pressed(MouseButton::Left) {
        box_coords.global_start = mouse_coords.global;
        box_coords.local_start = mouse_coords.local;
    }

    if input.pressed(MouseButton::Left) {
        box_coords.local_end = mouse_coords.local;
        box_coords.global_end = mouse_coords.global;
    }

    if input.just_released(MouseButton::Left) {
        box_coords.empty_global();
    }
}

// referenced https://bevy-cheatbook.github.io/cookbook/cursor2world.html
fn set_mouse_coords(
    mut mouse_coords: ResMut<MouseCoords>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    cam_q: Query<(&Camera, &GlobalTransform), With<RtsCamera>>,
    map_base_q: Query<&GlobalTransform, With<MapBase>>,
) {
    let (cam, cam_trans) = cam_q.single();
    let map_base_trans = map_base_q.single();
    let window = window_q.single();
    let Some(local_cursor) = window.cursor_position() else {
        return;
    };

    let plane_origin = map_base_trans.translation();
    let plane = InfinitePlane3d::new(map_base_trans.up());
    // let plane = Plane3d::new(map_base_trans.up());
    let Some(ray) = cam.viewport_to_world(cam_trans, local_cursor) else {
        return;
    };
    let Some(distance) = ray.intersect_plane(plane_origin, plane) else {
        return;
    };
    let global_cursor = ray.get_point(distance);

    mouse_coords.global = global_cursor;
    mouse_coords.local = local_cursor;
}

pub fn drag_select(
    mut gizmos: Gizmos,
    mut friendly_q: Query<(&Transform, &mut Selected, &mut ColliderDebugColor), With<Friendly>>,
    box_coords: Res<BoxCoords>,
    game_cmds: Res<GameCommands>,
) {
    if !game_cmds.drag_select {
        return;
    }

    let start = box_coords.global_start;
    let end = box_coords.global_end;

    // draw rectangle
    let gray = Color::srgb(0.68, 0.68, 0.68);
    gizmos.line(start, Vec3::new(end.x, 0.0, start.z), gray);
    gizmos.line(start, Vec3::new(start.x, 0.0, end.z), gray);
    gizmos.line(Vec3::new(start.x, 0.0, end.z), end, gray);
    gizmos.line(Vec3::new(end.x, 0.0, start.z), end, gray);

    let min_x = start.x.min(end.x);
    let max_x = start.x.max(end.x);
    let min_z = start.z.min(end.z);
    let max_z = start.z.max(end.z);

    for (unit_trans, mut selected, mut collider_color) in friendly_q.iter_mut() {
        // check to see if units are within selection rectangle
        let unit_pos = unit_trans.translation;
        let in_box_bounds = unit_pos.x >= min_x
            && unit_pos.x <= max_x
            && unit_pos.z >= min_z
            && unit_pos.z <= max_z;

        selected.0 = in_box_bounds;
        if selected.0 {
            collider_color.0.alpha = 1.0;
        } else {
            collider_color.0.alpha = 0.0;
        }
    }
}

pub fn single_select(
    rapier_context: Res<RapierContext>,
    cam_q: Query<(&Camera, &GlobalTransform)>,
    mut select_q: Query<(Entity, &mut Selected, &mut ColliderDebugColor), With<Friendly>>,
    mouse_coords: Res<MouseCoords>,
    input: Res<ButtonInput<MouseButton>>,
    game_cmds: Res<GameCommands>,
) {
    if !input.just_released(MouseButton::Left) || game_cmds.drag_select {
        return;
    }

    let (cam, cam_trans) = cam_q.single();

    let Some(ray) = cam.viewport_to_world(cam_trans, mouse_coords.local) else {
        return;
    };

    let hit = rapier_context.cast_ray(
        ray.origin,
        ray.direction.into(),
        f32::MAX,
        true,
        QueryFilter::only_dynamic(),
    );

    if let Some((ent, _)) = hit {
        // deselect all currently selected entities
        for (selected_entity, mut selected, mut collider_color) in select_q.iter_mut() {
            let tmp = selected_entity.index() == ent.index();
            if tmp && !selected.0 {
                selected.0 = true;
                collider_color.0.alpha = 1.0;
            } else {
                selected.0 = false;
                collider_color.0.alpha = 0.0;
            }
        }
    }
}

pub fn deselect_all(
    mut select_q: Query<(&mut Selected, &mut ColliderDebugColor), With<Selected>>,
    input: Res<ButtonInput<MouseButton>>,
) {
    if input.just_pressed(MouseButton::Right) {
        for (mut selected, mut collider_color) in select_q.iter_mut() {
            selected.0 = false;
            collider_color.0.alpha = 1.0;
        }
    }
}

fn set_selected(mut game_cmds: ResMut<GameCommands>, select_q: Query<&Selected>) {
    game_cmds.selected = false;
    for selected in select_q.iter() {
        if selected.0 {
            game_cmds.selected = true;
        }
    }
}

fn change_cursor(mut window_q: Query<&mut Window, With<PrimaryWindow>>, cursor: Res<CustomCursor>) {
    let mut window = window_q.get_single_mut().unwrap();
    match cursor.state {
        CursorState::Attack => window.cursor.icon = CursorIcon::Crosshair,
        CursorState::Relocate => window.cursor.icon = CursorIcon::Pointer,
        CursorState::Normal => window.cursor.icon = CursorIcon::Default,
    }
}
