
mod ftilt;
mod fsmash;

mod nair;
mod bair;
mod fair;

mod specialn;

pub fn install() {
    ftilt::install();
    fsmash::install();

    nair::install();
    bair::install();
    fair::install();

    specialn::install();
}