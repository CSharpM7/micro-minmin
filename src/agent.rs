use super::*;

pub const CHECK_SPECIAL_N_UNIQ:            i32 = 0x38;
pub const CHECK_SPECIAL_S_UNIQ:            i32 = 0x39;
pub const CHECK_SPECIAL_HI_UNIQ:            i32 = 0x3A;
pub const CHECK_SPECIAL_LW_UNIQ:            i32 = 0x3B;
pub const CHECK_AIR_SPECIAL_UNIQ:          i32 = 0x26;

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

#[skyline::hook(replace=WorkModule::is_enable_transition_term)]
unsafe fn is_enable_transition_term_hook(boma: &mut BattleObjectModuleAccessor, flag: i32) -> bool {
    if smash::app::utility::get_category(boma) == *BATTLE_OBJECT_CATEGORY_FIGHTER {

        let fighter_kind = utility::get_kind(boma);
        let status_kind = StatusModule::status_kind(boma);

        if fighter_kind == *FIGHTER_KIND_TANTAN {
            if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_N{
                return false;
            }
            if flag == *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SPECIAL_S{
                return false;
            }
        }
    }   
    original!()(boma, flag)
}

pub fn install() {
    install_agent_init_callbacks!(
        agent_init
    );
    install_agent_resets!(
        agent_reset
    );
    skyline::install_hooks!(
        is_enable_transition_term_hook
    );
}