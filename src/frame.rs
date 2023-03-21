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
            println!("Current:{} Common {} Attack {} 100 {}",
            status,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ATTACK,
            *FIGHTER_STATUS_KIND_ATTACK_100
        )
        }
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        tantan_update
    );
}