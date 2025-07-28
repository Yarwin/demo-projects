use godot::classes::{AnimationPlayer, CharacterBody3D, ICharacterBody3D};
use godot::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
pub struct Mob {
    // Minimum speed of the mob in meters per second.
    #[export]
    min_speed: f32,
    // Maximum speed of the mob in meters per second.
    #[export]
    max_speed: f32,
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Mob {
    fn physics_process(&mut self, _delta: f64) {
        self.base_mut().move_and_slide();
    }
}
#[godot_api]
impl Mob {
    #[func]
    pub fn initialize(&mut self, start_position: Vector3, player_position: Vector3) {
        self.base_mut()
            .look_at_from_position(start_position, player_position);

        self.base_mut()
            .rotate_y(rand::rng().random_range(-PI / 4.0..PI / 4.0));

        let random_speed = rand::rng().random_range(self.min_speed..self.max_speed);

        // We calculate a forward velocity first, which represents the speed.
        self.base_mut()
            .set_velocity(Vector3::FORWARD * random_speed);

        let rotation = self.base().get_rotation();
        let velocity = self.base().get_velocity();

        // We then rotate the vector based on the mob's Y rotation to move in the direction it's looking.
        self.base_mut()
            .set_velocity(velocity.rotated(Vector3::UP, rotation.y));

        let animation_speed = rand::rng().random_range(1.0..6.0);

        self.base()
            .get_node_as::<AnimationPlayer>("AnimationPlayer")
            .set_speed_scale(animation_speed as f32)
    }

    // Emitted when the player jumped on the mob.
    #[signal]
    pub fn squashed();

    #[func]
    pub fn squash(&mut self) {
        self.signals().squashed().emit();

        self.base_mut().queue_free();
    }

    #[func]
    fn on_visible_on_screen_notifier_3d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}
