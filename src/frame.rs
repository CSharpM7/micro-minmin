use super::*;

unsafe fn tilt_cancel(fighter: &mut L2CFighterCommon,boma: &mut BattleObjectModuleAccessor,status: i32)
{
    if (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_CHARGE))
    {
        return;
    }

    let armType =  WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R);
    //Smash: Ram:25,Mega:40,Drag:30
    let cancelFrameMin= if (armType==1) {40.0} else if (armType==2) {27.0} else {32.0};
    let cancelFrameMax=65.0;
    //let currentFrame = MotionModule::frame(fighter.module_accessor); 
    let currentFrame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_ATTACK_FRAME) as f32;
    let motionFrame = MotionModule::frame(fighter.module_accessor); 
    if cancelFrameMin <= currentFrame && currentFrame <= cancelFrameMax {
        let specialAttack = [
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WAIT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_RV,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_SQUAT_WAIT,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_FALL_AERIAL,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_JUMP_AERIAL,
            *FIGHTER_TANTAN_STATUS_KIND_ATTACK_WALK_BRAKE_BACK
        ].contains(&status);
        if (specialAttack){
            println!("Request cancel on frame {},motion {}",currentFrame,motionFrame);
            //There'S GOTTA be a better way than this 
            if StatusModule::situation_kind(fighter.module_accessor) != *SITUATION_KIND_AIR
            {
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                } 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI3, true);
                } 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_S4) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_S4_START, true);
                } 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_HI4) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, true);
                } 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW4) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, true);
                } 
                /* 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_LW3) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK_LW3, true);
                } 
                else if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_N) != 0 {
                    StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_ATTACK, true);
                } */
            }
            else{
                if (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_N) != 0 
                || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_F) != 0 
                || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_B) != 0
                || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_HI) != 0  
                || (ControlModule::get_command_flag_cat(boma, 0) & *FIGHTER_PAD_CMD_CAT1_FLAG_ATTACK_AIR_LW) != 0 
                || (ControlModule::get_pad_flag(boma) & *FIGHTER_PAD_FLAG_ATTACK_TRIGGER) != 0 
                {
                    println!("Aerial");
                    if (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_BACK_PUNCH_L))
                    || (WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_BACK_PUNCH_R))
                    {
                        println!("Switch");
                        PostureModule::set_lr(boma, -PostureModule::lr(boma));
                    }
                    fighter.change_status(FIGHTER_STATUS_KIND_ATTACK_AIR.into(), false.into());
                    //StatusModule::change_status_request_from_script(boma, 54, true);
                } 
            }
        }
    }
}

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        tilt_cancel(fighter,boma,status);

        if MotionModule::frame(fighter.module_accessor) < 2.0
        {
            println!("Current:{} Special {} Air {} Normal {}",
            status,
            *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL,
            *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_LANDING
            )
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        tantan_update
    );
}