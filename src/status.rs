use super::*;
pub const SUB_STATUS3:                     i32 = 0x13;


#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S3, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s3_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS3();
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
    println!("S4");
    //return fighter.status_pre_AttackS4();
    return false.into();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //return fighter.status_AttackS4_Main();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4_Main as *const () as _))
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_s4_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackS4();
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s4_start_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackS4Start();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_start_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("S4 start");
    return fighter.status_AttackS4Start();
    /* 
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Start as *const () as _))
    */
    //fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Start_Main as *const () as _))
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_s4_start_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackS4Start();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_START, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_attack_s4_start_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_s4_hold_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    macros::PLAY_SE(fighter, Hash40::new("se_common_smash_start"));
    return fighter.status_pre_AttackS4Hold();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_s4_hold_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    println!("S4 hold");
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_AttackS4Hold as *const () as _))
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_s4_hold_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackS4Hold();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_attack_s4_hold_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_AttackAir();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn tantan_attack_air_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_end_AttackAir();
}


#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn tantan_attack_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    return fighter.status_pre_Attack();
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn tantan_attack_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if !StopModule::is_stop(fighter.module_accessor) {
        fighter.check_attack_mtrans();
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(L2CFighterCommon_bind_address_call_check_attack_mtrans as *const () as _));
    fighter.sub_status_AttackComboCommon();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Attack_Main as *const () as _))
}
#[status_script(agent = "tantan", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn tantan_attack_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}

pub fn install() {
    install_status_scripts!(
        tantan_attack_pre,
        tantan_attack_main,
        tantan_attack_exec,

        tantan_attack_s3_pre,
        tantan_attack_s3_main,
        tantan_attack_s3_end,

        //tantan_attack_s4_pre,
        //tantan_attack_s4_main,
        //tantan_attack_s4_end,
        
        tantan_attack_s4_start_pre,
        tantan_attack_s4_start_main,
        tantan_attack_s4_start_exec,

        tantan_attack_s4_hold_pre,
        tantan_attack_s4_hold_main,
        tantan_attack_s4_hold_exec,

        tantan_attack_air_pre,
        tantan_attack_air_end
    );
} 