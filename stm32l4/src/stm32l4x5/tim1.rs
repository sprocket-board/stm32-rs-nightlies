#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - control register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - control register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - slave mode control register"]
    pub smcr: SMCR,
    #[doc = "0x0c - DMA/Interrupt enable register"]
    pub dier: DIER,
    #[doc = "0x10 - status register"]
    pub sr: SR,
    #[doc = "0x14 - event generation register"]
    pub egr: EGR,
    _reserved_6_ccmr1: [u8; 0x04],
    _reserved_7_ccmr2: [u8; 0x04],
    #[doc = "0x20 - capture/compare enable register"]
    pub ccer: CCER,
    #[doc = "0x24 - counter"]
    pub cnt: CNT,
    #[doc = "0x28 - prescaler"]
    pub psc: PSC,
    #[doc = "0x2c - auto-reload register"]
    pub arr: ARR,
    #[doc = "0x30 - repetition counter register"]
    pub rcr: RCR,
    #[doc = "0x34..0x44 - capture/compare register"]
    pub ccr: [CCR; 4],
    #[doc = "0x44 - break and dead-time register"]
    pub bdtr: BDTR,
    #[doc = "0x48 - DMA control register"]
    pub dcr: DCR,
    #[doc = "0x4c - DMA address for full transfer"]
    pub dmar: DMAR,
    #[doc = "0x50 - DMA address for full transfer"]
    pub or1: OR1,
    #[doc = "0x54 - capture/compare mode register 2 (output mode)"]
    pub ccmr3_output: CCMR3_OUTPUT,
    #[doc = "0x58 - capture/compare register"]
    pub ccr5: CCR5,
    #[doc = "0x5c - capture/compare register"]
    pub ccr6: CCR6,
    #[doc = "0x60 - DMA address for full transfer"]
    pub or2: OR2,
    #[doc = "0x64 - DMA address for full transfer"]
    pub or3: OR3,
}
impl RegisterBlock {
    #[doc = "0x18 - capture/compare mode register 1 (input mode)"]
    #[inline(always)]
    pub fn ccmr1_input(&self) -> &CCMR1_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_INPUT) }
    }
    #[doc = "0x18 - capture/compare mode register 1 (output mode)"]
    #[inline(always)]
    pub fn ccmr1_output(&self) -> &CCMR1_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(24usize) as *const CCMR1_OUTPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (input mode)"]
    #[inline(always)]
    pub fn ccmr2_input(&self) -> &CCMR2_INPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_INPUT) }
    }
    #[doc = "0x1c - capture/compare mode register 2 (output mode)"]
    #[inline(always)]
    pub fn ccmr2_output(&self) -> &CCMR2_OUTPUT {
        unsafe { &*(((self as *const Self) as *const u8).add(28usize) as *const CCMR2_OUTPUT) }
    }
    #[doc = "0x34 - capture/compare register"]
    #[inline(always)]
    pub fn ccr1(&self) -> &CCR {
        &self.ccr[0]
    }
    #[doc = "0x38 - capture/compare register"]
    #[inline(always)]
    pub fn ccr2(&self) -> &CCR {
        &self.ccr[1]
    }
    #[doc = "0x3c - capture/compare register"]
    #[inline(always)]
    pub fn ccr3(&self) -> &CCR {
        &self.ccr[2]
    }
    #[doc = "0x40 - capture/compare register"]
    #[inline(always)]
    pub fn ccr4(&self) -> &CCR {
        &self.ccr[3]
    }
}
#[doc = "CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "control register 2"]
pub mod cr2;
#[doc = "SMCR (rw) register accessor: an alias for `Reg<SMCR_SPEC>`"]
pub type SMCR = crate::Reg<smcr::SMCR_SPEC>;
#[doc = "slave mode control register"]
pub mod smcr;
#[doc = "DIER (rw) register accessor: an alias for `Reg<DIER_SPEC>`"]
pub type DIER = crate::Reg<dier::DIER_SPEC>;
#[doc = "DMA/Interrupt enable register"]
pub mod dier;
#[doc = "SR (rw) register accessor: an alias for `Reg<SR_SPEC>`"]
pub type SR = crate::Reg<sr::SR_SPEC>;
#[doc = "status register"]
pub mod sr;
#[doc = "EGR (w) register accessor: an alias for `Reg<EGR_SPEC>`"]
pub type EGR = crate::Reg<egr::EGR_SPEC>;
#[doc = "event generation register"]
pub mod egr;
#[doc = "CCMR1_Output (rw) register accessor: an alias for `Reg<CCMR1_OUTPUT_SPEC>`"]
pub type CCMR1_OUTPUT = crate::Reg<ccmr1_output::CCMR1_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 1 (output mode)"]
pub mod ccmr1_output;
#[doc = "CCMR1_Input (rw) register accessor: an alias for `Reg<CCMR1_INPUT_SPEC>`"]
pub type CCMR1_INPUT = crate::Reg<ccmr1_input::CCMR1_INPUT_SPEC>;
#[doc = "capture/compare mode register 1 (input mode)"]
pub mod ccmr1_input;
#[doc = "CCMR2_Output (rw) register accessor: an alias for `Reg<CCMR2_OUTPUT_SPEC>`"]
pub type CCMR2_OUTPUT = crate::Reg<ccmr2_output::CCMR2_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr2_output;
#[doc = "CCMR2_Input (rw) register accessor: an alias for `Reg<CCMR2_INPUT_SPEC>`"]
pub type CCMR2_INPUT = crate::Reg<ccmr2_input::CCMR2_INPUT_SPEC>;
#[doc = "capture/compare mode register 2 (input mode)"]
pub mod ccmr2_input;
#[doc = "CCER (rw) register accessor: an alias for `Reg<CCER_SPEC>`"]
pub type CCER = crate::Reg<ccer::CCER_SPEC>;
#[doc = "capture/compare enable register"]
pub mod ccer;
#[doc = "CNT (rw) register accessor: an alias for `Reg<CNT_SPEC>`"]
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
#[doc = "counter"]
pub mod cnt;
#[doc = "PSC (rw) register accessor: an alias for `Reg<PSC_SPEC>`"]
pub type PSC = crate::Reg<psc::PSC_SPEC>;
#[doc = "prescaler"]
pub mod psc;
#[doc = "ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`"]
pub type ARR = crate::Reg<arr::ARR_SPEC>;
#[doc = "auto-reload register"]
pub mod arr;
#[doc = "RCR (rw) register accessor: an alias for `Reg<RCR_SPEC>`"]
pub type RCR = crate::Reg<rcr::RCR_SPEC>;
#[doc = "repetition counter register"]
pub mod rcr;
#[doc = "CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "capture/compare register"]
pub mod ccr;
#[doc = "BDTR (rw) register accessor: an alias for `Reg<BDTR_SPEC>`"]
pub type BDTR = crate::Reg<bdtr::BDTR_SPEC>;
#[doc = "break and dead-time register"]
pub mod bdtr;
#[doc = "DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`"]
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
#[doc = "DMA control register"]
pub mod dcr;
#[doc = "DMAR (rw) register accessor: an alias for `Reg<DMAR_SPEC>`"]
pub type DMAR = crate::Reg<dmar::DMAR_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod dmar;
#[doc = "OR1 (rw) register accessor: an alias for `Reg<OR1_SPEC>`"]
pub type OR1 = crate::Reg<or1::OR1_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod or1;
#[doc = "CCMR3_Output (rw) register accessor: an alias for `Reg<CCMR3_OUTPUT_SPEC>`"]
pub type CCMR3_OUTPUT = crate::Reg<ccmr3_output::CCMR3_OUTPUT_SPEC>;
#[doc = "capture/compare mode register 2 (output mode)"]
pub mod ccmr3_output;
#[doc = "CCR5 (rw) register accessor: an alias for `Reg<CCR5_SPEC>`"]
pub type CCR5 = crate::Reg<ccr5::CCR5_SPEC>;
#[doc = "capture/compare register"]
pub mod ccr5;
#[doc = "CCR6 (rw) register accessor: an alias for `Reg<CCR6_SPEC>`"]
pub type CCR6 = crate::Reg<ccr6::CCR6_SPEC>;
#[doc = "capture/compare register"]
pub mod ccr6;
#[doc = "OR2 (rw) register accessor: an alias for `Reg<OR2_SPEC>`"]
pub type OR2 = crate::Reg<or2::OR2_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod or2;
#[doc = "OR3 (rw) register accessor: an alias for `Reg<OR3_SPEC>`"]
pub type OR3 = crate::Reg<or3::OR3_SPEC>;
#[doc = "DMA address for full transfer"]
pub mod or3;
