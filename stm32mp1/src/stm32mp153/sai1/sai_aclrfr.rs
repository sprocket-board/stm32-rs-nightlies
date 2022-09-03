#[doc = "Register `SAI_ACLRFR` writer"]
pub struct W(crate::W<SAI_ACLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACLRFR_SPEC>;
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
impl From<crate::W<SAI_ACLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COVRUDR` writer - COVRUDR"]
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
#[doc = "Field `CMUTEDET` writer - CMUTEDET"]
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
#[doc = "Field `CWCKCFG` writer - CWCKCFG"]
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
#[doc = "Field `CCNRDY` writer - CCNRDY"]
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
#[doc = "Field `CAFSDET` writer - CAFSDET"]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
#[doc = "Field `CLFSDET` writer - CLFSDET"]
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, SAI_ACLRFR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - COVRUDR"]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - CMUTEDET"]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - CWCKCFG"]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - CCNRDY"]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    #[doc = "Bit 5 - CAFSDET"]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - CLFSDET"]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aclrfr](index.html) module"]
pub struct SAI_ACLRFR_SPEC;
impl crate::RegisterSpec for SAI_ACLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sai_aclrfr::W](W) writer structure"]
impl crate::Writable for SAI_ACLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_ACLRFR to value 0"]
impl crate::Resettable for SAI_ACLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
