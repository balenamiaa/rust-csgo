use once_cell::sync::Lazy;
use std::sync::{Mutex, MutexGuard};

use crate::maths;
use crate::maths::Vector;
use crate::script_engine::aimbot::Aimbot;
use crate::script_engine::bhop::BHop;
use crate::structs::{
    CUserCMD, CmdButton, Entity, EntityIterator, EntityTeam, HitBoxes, PanelType,
};
use rlua::{
    Context, ExternalError, FromLua, Function, Lua, MetaMethod, ToLua, UserData, UserDataMethods,
    Value,
};

mod aimbot;
mod bhop;
mod modulesys;

struct LuaContext {
    ctx: Lua,
}

unsafe impl Send for LuaContext {}
unsafe impl Sync for LuaContext {}

pub struct ScriptEngine {
    modules: Vec<Box<dyn modulesys::Module + Send>>,
    lua: LuaContext,
}

impl UserData for Vector {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_method("Magnitude", |_, vec, ()| Ok(vec.len()));
        _methods.add_method("MagnitudeSqr", |_, vec, ()| Ok(vec.len_sqr()));
        _methods.add_method("InRadian", |_, vec, ()| Ok(*vec.clone().to_radians()));
        _methods.add_method("InDegree", |_, vec, ()| Ok(*vec.clone().to_degrees()));
        _methods.add_method_mut("Normalize", |_, vec, ()| {
            vec.normalize_yaw_cs();
            Ok(())
        });
        _methods.add_method_mut("Clamp", |_, vec, ()| {
            vec.clamp_csgo();
            Ok(())
        });

        _methods.add_meta_function(MetaMethod::Add, |_, (vec_a, vec_b): (Vector, Vector)| {
            Ok(vec_a + vec_b)
        });

        _methods.add_meta_function(MetaMethod::Add, |_, (mut vec_a, factor): (Vector, f32)| {
            vec_a.as_mut().iter_mut().for_each(|x| *x += factor);
            Ok(vec_a)
        });

        _methods.add_meta_function(MetaMethod::Sub, |_, (vec_a, vec_b): (Vector, Vector)| {
            Ok(vec_a - vec_b)
        });

        _methods.add_meta_function(MetaMethod::Sub, |_, (mut vec_a, factor): (Vector, f32)| {
            vec_a.as_mut().iter_mut().for_each(|x| *x -= factor);
            Ok(vec_a)
        });

        _methods.add_meta_function(MetaMethod::Div, |_, (vec_a, factor): (Vector, f32)| {
            Ok(vec_a / factor)
        });

        _methods.add_meta_function(MetaMethod::Mul, |_, (vec_a, factor): (Vector, f32)| {
            Ok(vec_a * factor)
        });

        _methods.add_meta_function(MetaMethod::Mul, |_, (vec_a, vec_b): (Vector, Vector)| {
            Ok(vec_a * vec_b)
        });
    }
}

impl<'lua> FromLua<'lua> for HitBoxes {
    fn from_lua(lua_value: Value<'lua>, lua: Context<'lua>) -> Result<Self, rlua::Error> {
        if let Value::Integer(hitbox_id) = lua_value {
            use std::convert::TryFrom;

            if let Ok(hitbox) = HitBoxes::try_from(hitbox_id as usize) {
                Ok(hitbox)
            } else {
                Err(rlua::Error::RuntimeError(
                    "Wrong hitbox argument! Must be an integer that's between 1 and 12.".to_owned(),
                )) //TODO: Use custom error types.
            }
        } else {
            Err(rlua::Error::FromLuaConversionError {
                from: "hitbox id",
                to: "hitbox",
                message: None,
            })
        }
    }
}

impl UserData for &Entity {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        use rlua::ExternalResult;
        _methods.add_method("GetOrigin", |_, &ent, ()| Ok(ent.origin()));
        _methods.add_method("GetHealth", |_, &ent, ()| Ok(ent.health()));
        _methods.add_method("IsAlive", |_, &ent, ()| Ok(ent.alive()));
        _methods.add_method("GetTeam", |_, &ent, ()| Ok(ent.team() as usize));
        _methods.add_method("ActiveWeapon", |_, &ent, ()| {
            Ok(ent.active_weapon().unwrap()) //TODO: Use custom error types.
        });
        _methods.add_method("WeaponId", |_, &ent, ()| Ok(ent.weapon_id() as usize));
        _methods.add_method("GetHitboxCenter", |_, &ent, hitbox_id| {
            Ok(ent.hitbox_center(hitbox_id))
        });
        _methods.add_method("GetEyePosition", |_, &ent, ()| Ok(ent.eye()));
        _methods.add_method("GetAimPunch", |_, &ent, ()| Ok(ent.aimpunch()));
        _methods.add_method("GetViewPunch", |_, &ent, ()| Ok(ent.viewpunch()));
        _methods.add_method("GetShotsFired", |_, &ent, ()| Ok(ent.shotsfired()));
        _methods.add_method(
            "viewpunch",
            |_, &ent, (other, position): (&Entity, Vector)| Ok(ent.is_visible(other, position)),
        );
        _methods.add_method("GetVelocity", |_, &ent, ()| Ok(ent.vel()));
        _methods.add_method("IsDormant", |_, &ent, ()| Ok(ent.dormant()));
        _methods.add_method("GetFlags", |_, &ent, ()| Ok(ent.flags().bits() as usize));
        _methods.add_method("IsPlayer", |_, &ent, ()| Ok(ent.is_player()));
        _methods.add_method("IsWeapon", |_, &ent, ()| Ok(ent.is_weapon()));
        _methods.add_method("GetAmmoClip1", |_, &ent, ()| Ok(ent.clip1_rem()));
    }
}

impl UserData for &mut CUserCMD {
    fn add_methods<'lua, T: UserDataMethods<'lua, Self>>(_methods: &mut T) {
        _methods.add_method_mut("SetViewAngle", |_, cmd, (view_angle)| {
            cmd.view_angles = view_angle;
            Ok(())
        });
        _methods.add_method_mut("SetButtons", |_, cmd, (mask, state)| {
            cmd.buttons.set(CmdButton::from_bits(mask).unwrap(), state);
            Ok(())
        });
        _methods.add_method_mut("GetViewAngle", |_, cmd, ()| Ok(cmd.view_angles));
        _methods.add_method_mut("GetButtonsRaw", |_, cmd, ()| Ok(cmd.buttons.bits()));
        _methods.add_method_mut("IsButtonSet", |_, cmd, (mask)| {
            Ok(cmd.buttons.contains(CmdButton::from_bits(mask).unwrap()))
        });
    }
}

impl ScriptEngine {
    pub fn fire_post_createmove(&mut self, cmd: &mut CUserCMD, frame_time: f32) {
        self.modules.iter_mut().for_each(|module| {
            module.post_createmove(cmd, frame_time);
        });
    }

    pub fn fire_painttraverse(&mut self, panel_type: PanelType) {
        self.modules
            .iter_mut()
            .for_each(|module| module.paint_traverse(panel_type));
    }

    #[allow(unused_must_use)]
    pub fn lua_createmove(&mut self, cmd: &'static mut CUserCMD) {
        self.lua.ctx.context(|ctx| -> rlua::Result<()> {
            // I don't think this is idiomatic??? Probably there's an overhead for accessing the context and getting the globals each createmove tick.
            let globals = ctx.globals();
            let vec_createmove: Vec<rlua::Function> = globals.get("CreateMoves")?;
            vec_createmove.iter().for_each(|f| unsafe {
                f.call::<&mut CUserCMD, ()>(std::mem::transmute(cmd as *mut _)) // K, this is soooo unrusty. I gotta find a way to do it in a proper way.
                    .expect("Calling createmove in lua space failed.")
            });

            Ok(())
        });
    }

    #[allow(unused_must_use)]
    pub fn initialize(&mut self, scripts_dir: &str) {
        self.modules.push(Box::new(BHop::new()));
        self.modules.push(Box::new(Aimbot::new()));

        self.lua.ctx.context(|ctx| -> rlua::Result<()> {
            let globals = ctx.globals();

            let maths_table = ctx.create_table()?;
            let calc_angle = ctx.create_function(|_, (src, target): (Vector, Vector)| {
                Ok(maths::calc_angle(src, target))
            })?;
            let get_fov = ctx.create_function(|_, (src, target, r): (Vector, Vector, f32)| {
                Ok(maths::get_fov(src, target, r))
            })?;
            let compensate_velocity =
                ctx.create_function(|_, (src, rel_velocity, r): (Vector, Vector, f32)| {
                    Ok(maths::compensate_velocity(src, rel_velocity, r))
                })?;
            let smooth_angle = ctx.create_function(
                |_, (src, target, delta_coefficient): (Vector, Vector, f32)| {
                    Ok(maths::smooth_angle(src, target, delta_coefficient))
                },
            )?;
            maths_table.set("CalcAngle", calc_angle);
            maths_table.set("GetFov", get_fov);
            maths_table.set("CompensateVelocity", compensate_velocity);
            maths_table.set("SmoothAngle", smooth_angle);

            globals.set("Maths", maths_table);
            //TODO: Engine interfaces.
            globals.set("Players", Vec::<&Entity>::new());

            let functions_table = ctx.create_table()?;
            globals.set("CreateMoves", functions_table);
            globals.set("CreateMovesCounter", 0);

            let register_createmove = ctx.create_function(|ctx, function: Function| {
                let globals = ctx.globals();
                let functions_table: rlua::Table = globals.get("CreateMoves")?;
                let counter: usize = globals.get("CreateMovesCounter")?;
                functions_table.set(counter, function);
                globals.set("CreateMovesCounter", counter + 1);
                Ok(())
            })?;

            globals.set("RegisterCreateMove", register_createmove);

            //load file into ctx
            use std::fs::File;
            use std::io::prelude::*;
            use std::path::Path;
            let lua_scripts_dir = Path::new(scripts_dir);

            //TODO: GOD, I need better error handling throughout the entire project.
            for entry in lua_scripts_dir.read_dir().expect("Invalid lua directory") {
                if let Ok(entry) = entry {
                    let path_buf = entry.path();

                    if let Some(file_type) = path_buf.as_path().extension() {
                        let file_type = file_type.to_str().unwrap(); //D;
                        if file_type == "lua" {
                            let src = std::fs::read_to_string(path_buf.as_path()).unwrap(); // D;

                            ctx.load(src.as_str()).exec();
                        }
                    }
                }
            }
            Ok(())
        });
    }

    #[allow(unused_must_use)]
    pub fn update_entities(&mut self) -> () {
        self.lua.ctx.context(|ctx| {
            ctx.globals().set(
                "players",
                EntityIterator::new()
                    .filter_map(|ent| match ent {
                        Some(ent) => {
                            if ent.dormant() || !ent.alive() || !ent.is_player() {
                                return None;
                            }

                            Some(ent)
                        }

                        None => None,
                    })
                    .collect::<Vec<&Entity>>(),
            );
        });
    }

    pub fn set_localplayer(&mut self) -> () {
        self.lua.ctx.context(|ctx| {
            if let Some(local_player) = Entity::local() {
                ctx.globals().set("LocalPlayer", local_player);
            }
        });
    }

    pub fn global() -> MutexGuard<'static, ScriptEngine> {
        INSTANCE.lock().unwrap()
    }
}

static INSTANCE: Lazy<Mutex<ScriptEngine>> = Lazy::new(|| Mutex::new(ScriptEngine::default()));

impl Default for ScriptEngine {
    fn default() -> Self {
        Self {
            modules: Vec::new(),
            lua: LuaContext { ctx: Lua::new() },
        }
    }
}
