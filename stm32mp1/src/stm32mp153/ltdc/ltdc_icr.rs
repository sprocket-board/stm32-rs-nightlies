#[doc = "Register `LTDC_ICR` writer"]
pub struct W(crate::W<LTDC_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_ICR_SPEC>;
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
impl From<crate::W<LTDC_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIF` writer - CLIF"]
pub type CLIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_ICR_SPEC, bool, O>;
#[doc = "Field `CFUIF` writer - CFUIF"]
pub type CFUIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_ICR_SPEC, bool, O>;
#[doc = "Field `CTERRIF` writer - CTERRIF"]
pub type CTERRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_ICR_SPEC, bool, O>;
#[doc = "Field `CRRIF` writer - CRRIF"]
pub type CRRIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, LTDC_ICR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - CLIF"]
    #[inline(always)]
    pub fn clif(&mut self) -> CLIF_W<0> {
        CLIF_W::new(self)
    }
    #[doc = "Bit 1 - CFUIF"]
    #[inline(always)]
    pub fn cfuif(&mut self) -> CFUIF_W<1> {
        CFUIF_W::new(self)
    }
    #[doc = "Bit 2 - CTERRIF"]
    #[inline(always)]
    pub fn cterrif(&mut self) -> CTERRIF_W<2> {
        CTERRIF_W::new(self)
    }
    #[doc = "Bit 3 - CRRIF"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CRRIF_W<3> {
        CRRIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_icr](index.html) module"]
pub struct LTDC_ICR_SPEC;
impl crate::RegisterSpec for LTDC_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ltdc_icr::W](W) writer structure"]
impl crate::Writable for LTDC_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_ICR to value 0"]
impl crate::Resettable for LTDC_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
