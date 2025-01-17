#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timdcr: TIMDCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timdisr: TIMDISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timdicr: TIMDICR,
    #[doc = "0x0c - TIMxDIER5"]
    pub timddier: TIMDDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntdr: CNTDR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perdr: PERDR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repdr: REPDR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1dr: CMP1DR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cdr: CMP1CDR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2dr: CMP2DR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3dr: CMP3DR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4dr: CMP4DR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1dr: CPT1DR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2dr: CPT2DR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtdr: DTDR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setd1r: SETD1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstd1r: RSTD1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setd2r: SETD2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstd2r: RSTD2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefdr1: EEFDR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefdr2: EEFDR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstdr: RSTDR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpdr: CHPDR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1dcr: CPT1DCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2dcr: CPT2DCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outdr: OUTDR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltdr: FLTDR,
}
#[doc = "TIMDCR (rw) register accessor: an alias for `Reg<TIMDCR_SPEC>`"]
pub type TIMDCR = crate::Reg<timdcr::TIMDCR_SPEC>;
#[doc = "Timerx Control Register"]
pub mod timdcr;
#[doc = "TIMDISR (r) register accessor: an alias for `Reg<TIMDISR_SPEC>`"]
pub type TIMDISR = crate::Reg<timdisr::TIMDISR_SPEC>;
#[doc = "Timerx Interrupt Status Register"]
pub mod timdisr;
#[doc = "TIMDICR (w) register accessor: an alias for `Reg<TIMDICR_SPEC>`"]
pub type TIMDICR = crate::Reg<timdicr::TIMDICR_SPEC>;
#[doc = "Timerx Interrupt Clear Register"]
pub mod timdicr;
#[doc = "TIMDDIER (rw) register accessor: an alias for `Reg<TIMDDIER_SPEC>`"]
pub type TIMDDIER = crate::Reg<timddier::TIMDDIER_SPEC>;
#[doc = "TIMxDIER5"]
pub mod timddier;
#[doc = "CNTDR (rw) register accessor: an alias for `Reg<CNTDR_SPEC>`"]
pub type CNTDR = crate::Reg<cntdr::CNTDR_SPEC>;
#[doc = "Timerx Counter Register"]
pub mod cntdr;
#[doc = "PERDR (rw) register accessor: an alias for `Reg<PERDR_SPEC>`"]
pub type PERDR = crate::Reg<perdr::PERDR_SPEC>;
#[doc = "Timerx Period Register"]
pub mod perdr;
#[doc = "REPDR (rw) register accessor: an alias for `Reg<REPDR_SPEC>`"]
pub type REPDR = crate::Reg<repdr::REPDR_SPEC>;
#[doc = "Timerx Repetition Register"]
pub mod repdr;
#[doc = "CMP1DR (rw) register accessor: an alias for `Reg<CMP1DR_SPEC>`"]
pub type CMP1DR = crate::Reg<cmp1dr::CMP1DR_SPEC>;
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1dr;
#[doc = "CMP1CDR (rw) register accessor: an alias for `Reg<CMP1CDR_SPEC>`"]
pub type CMP1CDR = crate::Reg<cmp1cdr::CMP1CDR_SPEC>;
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cdr;
#[doc = "CMP2DR (rw) register accessor: an alias for `Reg<CMP2DR_SPEC>`"]
pub type CMP2DR = crate::Reg<cmp2dr::CMP2DR_SPEC>;
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2dr;
#[doc = "CMP3DR (rw) register accessor: an alias for `Reg<CMP3DR_SPEC>`"]
pub type CMP3DR = crate::Reg<cmp3dr::CMP3DR_SPEC>;
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3dr;
#[doc = "CMP4DR (rw) register accessor: an alias for `Reg<CMP4DR_SPEC>`"]
pub type CMP4DR = crate::Reg<cmp4dr::CMP4DR_SPEC>;
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4dr;
#[doc = "CPT1DR (r) register accessor: an alias for `Reg<CPT1DR_SPEC>`"]
pub type CPT1DR = crate::Reg<cpt1dr::CPT1DR_SPEC>;
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1dr;
#[doc = "CPT2DR (r) register accessor: an alias for `Reg<CPT2DR_SPEC>`"]
pub type CPT2DR = crate::Reg<cpt2dr::CPT2DR_SPEC>;
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2dr;
#[doc = "DTDR (rw) register accessor: an alias for `Reg<DTDR_SPEC>`"]
pub type DTDR = crate::Reg<dtdr::DTDR_SPEC>;
#[doc = "Timerx Deadtime Register"]
pub mod dtdr;
#[doc = "SETD1R (rw) register accessor: an alias for `Reg<SETD1R_SPEC>`"]
pub type SETD1R = crate::Reg<setd1r::SETD1R_SPEC>;
#[doc = "Timerx Output1 Set Register"]
pub mod setd1r;
#[doc = "RSTD1R (rw) register accessor: an alias for `Reg<RSTD1R_SPEC>`"]
pub type RSTD1R = crate::Reg<rstd1r::RSTD1R_SPEC>;
#[doc = "Timerx Output1 Reset Register"]
pub mod rstd1r;
#[doc = "SETD2R (rw) register accessor: an alias for `Reg<SETD2R_SPEC>`"]
pub type SETD2R = crate::Reg<setd2r::SETD2R_SPEC>;
#[doc = "Timerx Output2 Set Register"]
pub mod setd2r;
#[doc = "RSTD2R (rw) register accessor: an alias for `Reg<RSTD2R_SPEC>`"]
pub type RSTD2R = crate::Reg<rstd2r::RSTD2R_SPEC>;
#[doc = "Timerx Output2 Reset Register"]
pub mod rstd2r;
#[doc = "EEFDR1 (rw) register accessor: an alias for `Reg<EEFDR1_SPEC>`"]
pub type EEFDR1 = crate::Reg<eefdr1::EEFDR1_SPEC>;
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefdr1;
#[doc = "EEFDR2 (rw) register accessor: an alias for `Reg<EEFDR2_SPEC>`"]
pub type EEFDR2 = crate::Reg<eefdr2::EEFDR2_SPEC>;
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefdr2;
#[doc = "RSTDR (rw) register accessor: an alias for `Reg<RSTDR_SPEC>`"]
pub type RSTDR = crate::Reg<rstdr::RSTDR_SPEC>;
#[doc = "TimerA Reset Register"]
pub mod rstdr;
#[doc = "CHPDR (rw) register accessor: an alias for `Reg<CHPDR_SPEC>`"]
pub type CHPDR = crate::Reg<chpdr::CHPDR_SPEC>;
#[doc = "Timerx Chopper Register"]
pub mod chpdr;
#[doc = "CPT1DCR (rw) register accessor: an alias for `Reg<CPT1DCR_SPEC>`"]
pub type CPT1DCR = crate::Reg<cpt1dcr::CPT1DCR_SPEC>;
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1dcr;
#[doc = "CPT2DCR (rw) register accessor: an alias for `Reg<CPT2DCR_SPEC>`"]
pub type CPT2DCR = crate::Reg<cpt2dcr::CPT2DCR_SPEC>;
#[doc = "CPT2xCR"]
pub mod cpt2dcr;
#[doc = "OUTDR (rw) register accessor: an alias for `Reg<OUTDR_SPEC>`"]
pub type OUTDR = crate::Reg<outdr::OUTDR_SPEC>;
#[doc = "Timerx Output Register"]
pub mod outdr;
#[doc = "FLTDR (rw) register accessor: an alias for `Reg<FLTDR_SPEC>`"]
pub type FLTDR = crate::Reg<fltdr::FLTDR_SPEC>;
#[doc = "Timerx Fault Register"]
pub mod fltdr;
