use super::*;

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_update(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        if [
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_100
        ].contains(&status)
        && false{
            println!("Current:{} Start {} Attack {} Hold {}",
            status,
            *FIGHTER_STATUS_KIND_ATTACK_S4_START,
            *FIGHTER_STATUS_KIND_ATTACK_S4,
            *FIGHTER_STATUS_KIND_ATTACK_S4_HOLD
            )
        }
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