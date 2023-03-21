use super::*;

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Attack();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_Attack_Main();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_Attack();
}


#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS3();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn tantan_attack_s3_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s3_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS3_Main();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_s3_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackS3();
}


#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s4_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS4();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_AttackS4_Main();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackS4();
}


#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackAir();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn tantan_attack_air_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_uniq_process_init();
    0.into()
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let nair = Hash40::new("attack_air_n").hash as i32;
    let fair = Hash40::new("attack_air_f").hash as i32;
    let bair = Hash40::new("attack_air_b").hash as i32;
    let upair = Hash40::new("attack_air_hi").hash as i32;
    let dair = Hash40::new("attack_air_lw").hash as i32;
    let fighter_log_attack_kind = match motion_kind {
        nair => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_N,
        fair => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F,
        bair => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_B,
        dair => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_LW,
        upair => *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_HI,
        _ => {
            fighter.sub_attack_air_common(false.into());
            MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
            return fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status as *const () as _))
        }
    };
    smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    return fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_loop as *const () as _))
}

pub unsafe extern "C" fn attack_air_main_status(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_attack_air_common(L2CValue::Bool(false));
    MotionModule::set_trans_move_speed_no_scale(fighter.module_accessor, true);
    fighter.sub_shift_status_main(L2CValue::Ptr(attack_air_main_status_loop as *const () as _))
}

pub unsafe extern "C" fn attack_air_main_status_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_AttackAir_Main_common().get_bool() {
        fighter.sub_air_check_superleaf_fall_slowly();
        //if !fighter.global_table[IS_STOPPING].get_bool() {
            fighter.sub_attack_air_inherit_jump_aerial_motion_uniq_process_exec_fix_pos();
        //}
        0.into()
    }
    else {
        1.into()
    }
}

unsafe extern "C" fn attack_air_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("attacking in the air!");
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);

    let _ = fighter.status_AttackAir_Main_common();
    WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);
    0.into()
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackAir();
}



pub fn install() {
    install_status_scripts!(
        tantan_attack_pre,
        tantan_attack_main,
        tantan_attack_end,

        tantan_attack_s3_pre,
        tantan_attack_s3_main,
        tantan_attack_s3_end,

        tantan_attack_s4_pre,
        tantan_attack_s4_main,
        tantan_attack_s4_end,
        
        tantan_attack_air_init,
        tantan_attack_air_pre,
        tantan_attack_air_main,
        tantan_attack_air_end
    );
} 