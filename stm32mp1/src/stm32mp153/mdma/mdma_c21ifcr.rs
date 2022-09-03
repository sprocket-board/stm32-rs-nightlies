#[doc = "Register `MDMA_C21IFCR` writer"]
pub struct W(crate::W<MDMA_C21IFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDMA_C21IFCR_SPEC>;
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
impl From<crate::W<MDMA_C21IFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDMA_C21IFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTEIF` writer - CTEIF"]
pub type CTEIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C21IFCR_SPEC, bool, O>;
#[doc = "Field `CCTCIF` writer - CCTCIF"]
pub type CCTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C21IFCR_SPEC, bool, O>;
#[doc = "Field `CBRTIF` writer - CBRTIF"]
pub type CBRTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C21IFCR_SPEC, bool, O>;
#[doc = "Field `CBTIF` writer - CBTIF"]
pub type CBTIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C21IFCR_SPEC, bool, O>;
#[doc = "Field `CLTCIF` writer - CLTCIF"]
pub type CLTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MDMA_C21IFCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CTEIF_W<0> {
        CTEIF_W::new(self)
    }
    #[doc = "Bit 1 - CCTCIF"]
    #[inline(always)]
    pub fn cctcif(&mut self) -> CCTCIF_W<1> {
        CCTCIF_W::new(self)
    }
    #[doc = "Bit 2 - CBRTIF"]
    #[inline(always)]
    pub fn cbrtif(&mut self) -> CBRTIF_W<2> {
        CBRTIF_W::new(self)
    }
    #[doc = "Bit 3 - CBTIF"]
    #[inline(always)]
    pub fn cbtif(&mut self) -> CBTIF_W<3> {
        CBTIF_W::new(self)
    }
    #[doc = "Bit 4 - CLTCIF"]
    #[inline(always)]
    pub fn cltcif(&mut self) -> CLTCIF_W<4> {
        CLTCIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MDMA channel 21 interrupt flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdma_c21ifcr](index.html) module"]
pub struct MDMA_C21IFCR_SPEC;
impl crate::RegisterSpec for MDMA_C21IFCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mdma_c21ifcr::W](W) writer structure"]
impl crate::Writable for MDMA_C21IFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MDMA_C21IFCR to value 0"]
impl crate::Resettable for MDMA_C21IFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
