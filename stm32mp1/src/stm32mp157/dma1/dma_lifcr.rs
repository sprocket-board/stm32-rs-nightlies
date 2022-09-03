#[doc = "Register `DMA_LIFCR` writer"]
pub struct W(crate::W<DMA_LIFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_LIFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_LIFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_LIFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFEIF0` writer - CFEIF0"]
pub type CFEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF0` writer - CDMEIF0"]
pub type CDMEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF0` writer - CTEIF0"]
pub type CTEIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF0` writer - CHTIF0"]
pub type CHTIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF0` writer - CTCIF0"]
pub type CTCIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF1` writer - CFEIF1"]
pub type CFEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF1` writer - CDMEIF1"]
pub type CDMEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF1` writer - CTEIF1"]
pub type CTEIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF1` writer - CHTIF1"]
pub type CHTIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF1` writer - CTCIF1"]
pub type CTCIF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF2` writer - CFEIF2"]
pub type CFEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF2` writer - CDMEIF2"]
pub type CDMEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF2` writer - CTEIF2"]
pub type CTEIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF2` writer - CHTIF2"]
pub type CHTIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF2` writer - CTCIF2"]
pub type CTCIF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CFEIF3` writer - CFEIF3"]
pub type CFEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CDMEIF3` writer - CDMEIF3"]
pub type CDMEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTEIF3` writer - CTEIF3"]
pub type CTEIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CHTIF3` writer - CHTIF3"]
pub type CHTIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
#[doc = "Field `CTCIF3` writer - CTCIF3"]
pub type CTCIF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, DMA_LIFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CFEIF0"]
    #[inline(always)]
    pub fn cfeif0(&mut self) -> CFEIF0_W<0> {
        CFEIF0_W::new(self)
    }
    #[doc = "Bit 2 - CDMEIF0"]
    #[inline(always)]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<2> {
        CDMEIF0_W::new(self)
    }
    #[doc = "Bit 3 - CTEIF0"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> CTEIF0_W<3> {
        CTEIF0_W::new(self)
    }
    #[doc = "Bit 4 - CHTIF0"]
    #[inline(always)]
    pub fn chtif0(&mut self) -> CHTIF0_W<4> {
        CHTIF0_W::new(self)
    }
    #[doc = "Bit 5 - CTCIF0"]
    #[inline(always)]
    pub fn ctcif0(&mut self) -> CTCIF0_W<5> {
        CTCIF0_W::new(self)
    }
    #[doc = "Bit 6 - CFEIF1"]
    #[inline(always)]
    pub fn cfeif1(&mut self) -> CFEIF1_W<6> {
        CFEIF1_W::new(self)
    }
    #[doc = "Bit 8 - CDMEIF1"]
    #[inline(always)]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<8> {
        CDMEIF1_W::new(self)
    }
    #[doc = "Bit 9 - CTEIF1"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> CTEIF1_W<9> {
        CTEIF1_W::new(self)
    }
    #[doc = "Bit 10 - CHTIF1"]
    #[inline(always)]
    pub fn chtif1(&mut self) -> CHTIF1_W<10> {
        CHTIF1_W::new(self)
    }
    #[doc = "Bit 11 - CTCIF1"]
    #[inline(always)]
    pub fn ctcif1(&mut self) -> CTCIF1_W<11> {
        CTCIF1_W::new(self)
    }
    #[doc = "Bit 16 - CFEIF2"]
    #[inline(always)]
    pub fn cfeif2(&mut self) -> CFEIF2_W<16> {
        CFEIF2_W::new(self)
    }
    #[doc = "Bit 18 - CDMEIF2"]
    #[inline(always)]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<18> {
        CDMEIF2_W::new(self)
    }
    #[doc = "Bit 19 - CTEIF2"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> CTEIF2_W<19> {
        CTEIF2_W::new(self)
    }
    #[doc = "Bit 20 - CHTIF2"]
    #[inline(always)]
    pub fn chtif2(&mut self) -> CHTIF2_W<20> {
        CHTIF2_W::new(self)
    }
    #[doc = "Bit 21 - CTCIF2"]
    #[inline(always)]
    pub fn ctcif2(&mut self) -> CTCIF2_W<21> {
        CTCIF2_W::new(self)
    }
    #[doc = "Bit 22 - CFEIF3"]
    #[inline(always)]
    pub fn cfeif3(&mut self) -> CFEIF3_W<22> {
        CFEIF3_W::new(self)
    }
    #[doc = "Bit 24 - CDMEIF3"]
    #[inline(always)]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<24> {
        CDMEIF3_W::new(self)
    }
    #[doc = "Bit 25 - CTEIF3"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> CTEIF3_W<25> {
        CTEIF3_W::new(self)
    }
    #[doc = "Bit 26 - CHTIF3"]
    #[inline(always)]
    pub fn chtif3(&mut self) -> CHTIF3_W<26> {
        CHTIF3_W::new(self)
    }
    #[doc = "Bit 27 - CTCIF3"]
    #[inline(always)]
    pub fn ctcif3(&mut self) -> CTCIF3_W<27> {
        CTCIF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA low interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_lifcr](index.html) module"]
pub struct DMA_LIFCR_SPEC;
impl crate::RegisterSpec for DMA_LIFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_lifcr::W](W) writer structure"]
impl crate::Writable for DMA_LIFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_LIFCR to value 0"]
impl crate::Resettable for DMA_LIFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
