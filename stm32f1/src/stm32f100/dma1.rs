#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch: [CH; 7],
}
impl RegisterBlock {
    #[doc = "0x08..0x1c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch1(&self) -> &CH {
        &self.ch[0]
    }
    #[doc = "0x1c..0x30 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch2(&self) -> &CH {
        &self.ch[1]
    }
    #[doc = "0x30..0x44 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch3(&self) -> &CH {
        &self.ch[2]
    }
    #[doc = "0x44..0x58 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch4(&self) -> &CH {
        &self.ch[3]
    }
    #[doc = "0x58..0x6c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch5(&self) -> &CH {
        &self.ch[4]
    }
    #[doc = "0x6c..0x80 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch6(&self) -> &CH {
        &self.ch[5]
    }
    #[doc = "0x80..0x94 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    #[inline(always)]
    pub fn ch7(&self) -> &CH {
        &self.ch[6]
    }
}
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub use ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
