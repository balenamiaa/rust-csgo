use crate::structs::{
    CUserCMD, CmdButton, Entity, EntityFlags, EntityIterator, HitBoxes, ItemDefIndices,
};

use super::modulesys::Module;
use crate::interfaces::{Globals, NetvarEngine};
use crate::maths::{calc_angle, compensate_velocity, get_fov, smooth_angle, Vector};
use winapi::um::winuser::GetAsyncKeyState;

struct AimbotConfig {
    pistol_mode: bool,
    smooth_fn: Option<Box<dyn Fn(f32) -> f32>>,
    max_fov: f32,
    hitboxes: Vec<HitBoxes>,
    lock: bool,
}

impl Default for AimbotConfig {
    fn default() -> Self {
        Self {
            pistol_mode: false,
            smooth_fn: None,
            max_fov: 90e0,
            hitboxes: Vec::new(),
            lock: true,
        }
    }
}

pub struct Aimbot {
    current_target: Option<&'static Entity>,
    current_hitbox: Option<(HitBoxes, Vector)>,
    current_config: AimbotConfig,
    current_aimtime: Option<f32>,
    /// true if target is found and we're at tick 0 since left click was pressed, false otherwise.
    target_set_tick0: bool,
}

unsafe impl Send for Aimbot {}
unsafe impl Sync for Aimbot {}

impl Aimbot {
    pub fn new() -> Aimbot {
        Self {
            current_target: None,
            current_config: AimbotConfig::default(),
            current_hitbox: None,
            current_aimtime: None,
            target_set_tick0: false,
        }
    }

    fn triggered(&mut self, local_player: &'static Entity, cmd: &mut CUserCMD) -> bool {
        if cmd.buttons.contains(CmdButton::IN_ATTACK) {
            self.current_target = None;
            self.current_hitbox = None;
            self.current_aimtime = None;
            self.target_set_tick0 = false;

            false
        } else {
            true
        }
    }

    fn set_target(&mut self, local_player: &'static Entity, cmd: &mut CUserCMD) -> bool {
        if let Some(old_target) = self.current_target {
            if old_target.alive() && !old_target.dormant() {
                self.target_set_tick0 = false;

                true
            } else {
                self.current_target = None;
                self.current_hitbox = None;
                self.current_aimtime = None;
                self.target_set_tick0 = false;

                false
            }
        } else {
            let mut valid_targets: Vec<&'static Entity> = EntityIterator::new()
                .filter_map(|entity| entity)
                .filter(|&entity| {
                    !entity.dormant()
                        && entity.is_player()
                        && entity.alive()
                        && (entity.team() != local_player.team())
                })
                .collect();

            if valid_targets.is_empty() {
                return false;
            }

            valid_targets.sort_by(|&a, &b| {
                let pseudo_fov_a = (cmd.view_angles - calc_angle(local_player.eye(), a.eye()))
                    .normalize_yaw_cs()
                    .len();

                let pseudo_fov_b = (cmd.view_angles - calc_angle(local_player.eye(), b.eye()))
                    .normalize_yaw_cs()
                    .len();

                pseudo_fov_a.partial_cmp(&pseudo_fov_b).unwrap()
            });

            self.current_target = Some(*valid_targets.first().unwrap());

            self.target_set_tick0 = true;

            true
        }
    }

    fn set_hitbox(&mut self, local_player: &'static Entity, cmd: &mut CUserCMD) -> bool {
        if let Some(current_target) = self.current_target {
            if let Some(current_hitbox) = self.current_hitbox {
                if let Some(updated_hitbox) = current_target.hitbox_center(current_hitbox.0) {
                    self.current_hitbox = Some((current_hitbox.0, updated_hitbox));

                    true
                } else {
                    false
                }
            } else {
                let mut copy: Vec<_> = self
                    .current_config
                    .hitboxes
                    .clone()
                    .iter()
                    .map(|&hitbox| {
                        (
                            hitbox,
                            compensate_velocity(
                                current_target.hitbox_center(hitbox).unwrap(),
                                current_target.vel() - local_player.vel(),
                                (local_player.origin() - current_target.origin()).len(),
                            ),
                        )
                    })
                    .collect();
                copy.sort_by(|&a, &b| {
                    let lp_eye = local_player.eye();

                    let a_fov = get_fov(
                        cmd.view_angles,
                        calc_angle(lp_eye, a.1),
                        (lp_eye - a.1).len(),
                    );

                    let b_fov = get_fov(
                        cmd.view_angles,
                        calc_angle(lp_eye, b.1),
                        (lp_eye - b.1).len(),
                    );

                    a_fov.partial_cmp(&b_fov).unwrap()
                });

                self.current_hitbox = Some(*copy.first().unwrap());

                true
            }
        } else {
            false
        }
    }

    fn set_config(&mut self, local_player: &'static Entity, cmd: &mut CUserCMD) -> bool {
        use std::ops::{Div, Mul, Neg, Sub};

        if let Some(wep) = local_player.active_weapon() {
            use ItemDefIndices::*;

            match wep.weapon_id() {
                AK47 | M4A1 | MP9 | MP7 | GALILAR | FAMAS | P90 | SG556 => {
                    self.current_config = AimbotConfig {
                        pistol_mode: false,
                        lock: true,
                        smooth_fn: Some(Box::new(|time| {
                            ((time.mul(2e0) - 1f32.div(2e0)).div(1.2e0).sin().powi(2)
                                + (time - 1f32.div(2e0)).div(1.2e0).cos()
                                - 1.2)
                                .abs()
                        })),
                        max_fov: 16e0,
                        hitboxes: vec![HitBoxes::Head, HitBoxes::LowerChest],
                    };

                    true
                }

                AWP | SSG08 => {
                    self.current_config = AimbotConfig {
                        pistol_mode: false,
                        lock: false,
                        smooth_fn: Option::None, //ambiguity introduced due to ItemDefIndices::None
                        max_fov: 24e0,
                        hitboxes: vec![
                            HitBoxes::Neck,
                            HitBoxes::Chest,
                            HitBoxes::Stomach,
                            HitBoxes::LowerChest,
                        ],
                    };

                    true
                }

                DEAGLE => {
                    self.current_config = AimbotConfig {
                        pistol_mode: false,
                        lock: false,
                        smooth_fn: Option::None, //ambiguity introduced due to ItemDefIndices::None
                        max_fov: 12e0,
                        hitboxes: vec![HitBoxes::Head],
                    };

                    true
                }

                P250 | TEC9 | GLOCK | USP_SILENCER | HKP2000 | FIVESEVEN => {
                    self.current_config = AimbotConfig {
                        pistol_mode: true,
                        lock: false,
                        smooth_fn: Option::None, //ambiguity introduced due to ItemDefIndices::None
                        max_fov: 14e0,
                        hitboxes: vec![HitBoxes::Head],
                    };

                    true
                }

                _ => {
                    self.current_config = AimbotConfig::default();

                    false
                }
            }
        } else {
            false
        }
    }

    fn is_ready(&mut self, local_player: &'static Entity, cmd: &mut CUserCMD) -> bool {
        if let Some(current_target) = self.current_target {
            if let Some(current_hitbox) = self.current_hitbox {
                if let Some(current_weapon) = local_player.active_weapon() {
                    if !local_player.is_visible(current_target, current_hitbox.1) {
                        return false;
                    }

                    if self.target_set_tick0 == true
                        && get_fov(
                            cmd.view_angles,
                            calc_angle(local_player.eye(), current_hitbox.1),
                            (current_hitbox.1 - local_player.eye()).len(),
                        ) > self.current_config.max_fov
                    {
                        self.current_target = None;
                        self.current_hitbox = None;
                        self.current_aimtime = None;
                        self.target_set_tick0 = false;

                        return false;
                    }

                    if current_weapon.is_reloading() || current_weapon.clip1_rem() <= 0 {
                        return false;
                    }

                    let server_time =
                        local_player.tickbase() as f32 * Globals::get().inv_tickspersecond;
                    if local_player.next_attack() > server_time {
                        return false;
                    }

                    if self.current_config.pistol_mode {
                        use ItemDefIndices::*;
                        let rem = current_weapon.clip1_rem();
                        match current_weapon.weapon_id() {
                            P250 => {
                                if (13 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            TEC9 => {
                                if (18 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            GLOCK => {
                                if (20 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            USP_SILENCER => {
                                if (12 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            HKP2000 => {
                                if (13 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            FIVESEVEN => {
                                if (20 - rem) % 2 == 0 {
                                    return false;
                                }
                            }
                            _ => {
                                return false;
                            }
                        }
                    }

                    current_weapon.next_primary_attack() <= server_time
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        }
    }

    fn aim_at_current_target(
        &mut self,
        local_player: &'static Entity,
        cmd: &mut CUserCMD,
        frame_time: f32,
    ) {
        if let Some(current_target) = self.current_target {
            if let Some((_, target_vec)) = self.current_hitbox {
                let mut target_angle = calc_angle(local_player.eye(), target_vec);

                let punch = Vector {
                    x: local_player.aimpunch().x,
                    y: if local_player.shotsfired() > 1 {
                        local_player.aimpunch().y
                    } else {
                        0e0
                    },
                    z: 0e0,
                };

                target_angle -= punch * 2e0;

                if let Some(current_aimtime) = self.current_aimtime {
                    if let Some(ref smooth_fn) = self.current_config.smooth_fn {
                        target_angle = smooth_angle(
                            cmd.view_angles,
                            target_angle,
                            smooth_fn(current_aimtime.clamp(0e0, 1e0)),
                        );
                    }

                    self.current_aimtime = Some(current_aimtime + frame_time)
                } else {
                    self.current_aimtime = Some(f32::default());

                    return;
                }

                cmd.view_angles = target_angle;
            }
        }
    }
}

impl Module for Aimbot {
    fn post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) -> () {
        if let Some(local_player) = Entity::local() {
            if !local_player.alive() {
                return;
            }

            if self.triggered(local_player, cmd) {
                if self.set_config(local_player, cmd) {
                    if self.set_target(local_player, cmd) {
                        if self.set_hitbox(local_player, cmd) {
                            if self.is_ready(local_player, cmd) {
                                self.aim_at_current_target(local_player, cmd, frame_time);
                            }
                        }
                    }
                }
            }
        }
    }
}
