use super::*;

#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_update(fighter: &mut L2CFighterCommon) {
    unsafe {
    }
}


pub fn install() {
    smashline::install_agent_frames!(
        tantan_update
    );
}