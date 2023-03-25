use super::*;

pub const CHECK_SPECIAL_N_UNIQ:            i32 = 0x38;
pub const CHECK_SPECIAL_S_UNIQ:            i32 = 0x39;
pub const CHECK_SPECIAL_HI_UNIQ:            i32 = 0x3A;
pub const CHECK_SPECIAL_LW_UNIQ:            i32 = 0x3B;
pub const CHECK_SPECIAL_AIR_UNIQ:          i32 = 0x26;
pub const CHECK_GROUND_CATCH_UNIQ:         i32 = 0x50;

unsafe extern "C" fn tantan_special_none_preU(fighter: &mut L2CFighterCommon) -> L2CValue {
    return false.into();
}
unsafe fn agent_start(fighter: &mut L2CFighterCommon)
{
    let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
    if fighter_kind != *FIGHTER_KIND_TANTAN {
        return;
    }
    fighter.global_table[CHECK_SPECIAL_N_UNIQ].assign(&L2CValue::Ptr(tantan_special_none_preU as *const () as _));
    fighter.global_table[CHECK_SPECIAL_S_UNIQ].assign(&L2CValue::Ptr(tantan_special_none_preU as *const () as _));
    fighter.global_table[CHECK_SPECIAL_LW_UNIQ].assign(&L2CValue::Ptr(tantan_special_none_preU as *const () as _));
    fighter.global_table[CHECK_GROUND_CATCH_UNIQ].assign(&L2CValue::Ptr(tantan_special_none_preU as *const () as _));
    println!("No more specials!");

}

#[fighter_init]
fn agent_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        agent_start(fighter);
    }
}
pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
}