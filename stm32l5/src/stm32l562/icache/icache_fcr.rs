#[doc = "Register `ICACHE_FCR` writer"]
pub struct W(crate::W<ICACHE_FCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_FCR_SPEC>;
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
impl From<crate::W<ICACHE_FCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_FCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CBSYENDF` writer - CBSYENDF"]
pub type CBSYENDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_FCR_SPEC, bool, O>;
#[doc = "Field `CERRF` writer - CERRF"]
pub type CERRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICACHE_FCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 1 - CBSYENDF"]
    #[inline(always)]
    pub fn cbsyendf(&mut self) -> CBSYENDF_W<1> {
        CBSYENDF_W::new(self)
    }
    #[doc = "Bit 2 - CERRF"]
    #[inline(always)]
    pub fn cerrf(&mut self) -> CERRF_W<2> {
        CERRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ICACHE flag clear register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_fcr](index.html) module"]
pub struct ICACHE_FCR_SPEC;
impl crate::RegisterSpec for ICACHE_FCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [icache_fcr::W](W) writer structure"]
impl crate::Writable for ICACHE_FCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ICACHE_FCR to value 0"]
impl crate::Resettable for ICACHE_FCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
