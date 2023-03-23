use super::*;

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        return fighter.status_pre_AttackAir();
    }
    else{
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_special_n_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        let fighter_log_attack_kind = *FIGHTER_LOG_ATTACK_KIND_ATTACK_AIR_F;
        smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2b94de0d96), FIGHTER_LOG_ACTION_CATEGORY_KEEP, fighter_log_attack_kind);
        let motion_kind = Hash40::new("attack_air_f").hash;//MotionModule::motion_kind(fighter.module_accessor);
        WorkModule::set_int64(fighter.module_accessor, motion_kind as i64, *FIGHTER_STATUS_ATTACK_AIR_WORK_INT_MOTION_KIND);


        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n"), 0.0, 1.0, false, 0.0, false, false);
        //fighter.sub_shift_status_main(L2CValue::Ptr(tantan_special_n_main_loop as *const () as _))
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackAir_Main as *const () as _))
    }
    else{
        return original!(fighter);
    }
}
unsafe extern "C" fn tantan_special_n_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if MotionModule::is_end(fighter.module_accessor) {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL_SPECIAL.into(), false.into());
    }
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_GROUND{
        if (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)){
            fighter.change_status(FIGHTER_TANTAN_STATUS_KIND_ATTACK_LANDING.into(), false.into());
        }
        else{
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    return true.into()
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_landing_air_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::prev_status_kind(fighter.module_accessor, 0) == *FIGHTER_STATUS_KIND_SPECIAL_N
    {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("landing_special_n"), 0.0, 1.0, false, 0.0, false, false);
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_LandingAttackAir_Main as *const () as _))
    }
    else{
        return original!(fighter);
    }
}

 
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_N, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_disable_special_n_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
        return false.into();
    }
    else
    {
        return original!(fighter);
    }
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_SPECIAL_S, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_disable_special_s_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::situation_kind(fighter.module_accessor) ==*SITUATION_KIND_AIR
    {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
        return false.into();
    }
    else
    {
        return original!(fighter);
    }
}
pub fn install() {
    if data::use_Specials(){
        install_status_scripts!(
            tantan_special_n_pre,
            tantan_special_n_main,
            tantan_landing_air_main
        );
    }
    else{
        install_status_scripts!(
            tantan_disable_special_n_pre,
            tantan_disable_special_s_pre
        );
    }
} 