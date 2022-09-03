#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    _reserved2: [u8; 0x34],
    #[doc = "0x3c - APB1 Low Freeze Register CPU1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x40 - APB1 Low Freeze Register CPU2"]
    pub c2ap_b1fzr1: C2AP_B1FZR1,
    #[doc = "0x44 - APB1 High Freeze Register CPU1"]
    pub apb1fzr2: APB1FZR2,
    _reserved_5_c2apb: [u8; 0x04],
    #[doc = "0x4c - APB2 Freeze Register CPU1"]
    pub apb2fzr: APB2FZR,
}
impl RegisterBlock {
    #[doc = "0x48 - APB2 Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb2fzr(&self) -> &C2APB2FZR {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const C2APB2FZR) }
    }
    #[doc = "0x48 - APB1 High Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb1fzr2(&self) -> &C2APB1FZR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const C2APB1FZR2) }
    }
}
#[doc = "IDCODE (r) register accessor: an alias for `Reg<IDCODE_SPEC>`"]
pub type IDCODE = crate::Reg<idcode::IDCODE_SPEC>;
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB1FZR1 (rw) register accessor: an alias for `Reg<APB1FZR1_SPEC>`"]
pub type APB1FZR1 = crate::Reg<apb1fzr1::APB1FZR1_SPEC>;
#[doc = "APB1 Low Freeze Register CPU1"]
pub mod apb1fzr1;
#[doc = "C2AP_B1FZR1 (rw) register accessor: an alias for `Reg<C2AP_B1FZR1_SPEC>`"]
pub type C2AP_B1FZR1 = crate::Reg<c2ap_b1fzr1::C2AP_B1FZR1_SPEC>;
#[doc = "APB1 Low Freeze Register CPU2"]
pub mod c2ap_b1fzr1;
#[doc = "APB1FZR2 (rw) register accessor: an alias for `Reg<APB1FZR2_SPEC>`"]
pub type APB1FZR2 = crate::Reg<apb1fzr2::APB1FZR2_SPEC>;
#[doc = "APB1 High Freeze Register CPU1"]
pub mod apb1fzr2;
#[doc = "C2APB1FZR2 (rw) register accessor: an alias for `Reg<C2APB1FZR2_SPEC>`"]
pub type C2APB1FZR2 = crate::Reg<c2apb1fzr2::C2APB1FZR2_SPEC>;
#[doc = "APB1 High Freeze Register CPU2"]
pub mod c2apb1fzr2;
#[doc = "APB2FZR (rw) register accessor: an alias for `Reg<APB2FZR_SPEC>`"]
pub type APB2FZR = crate::Reg<apb2fzr::APB2FZR_SPEC>;
#[doc = "APB2 Freeze Register CPU1"]
pub mod apb2fzr;
#[doc = "C2APB2FZR (rw) register accessor: an alias for `Reg<C2APB2FZR_SPEC>`"]
pub type C2APB2FZR = crate::Reg<c2apb2fzr::C2APB2FZR_SPEC>;
#[doc = "APB2 Freeze Register CPU2"]
pub mod c2apb2fzr;
