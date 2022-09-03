#[doc = "Register `TIMx_EGR` writer"]
pub struct W(crate::W<TIMX_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_EGR_SPEC>;
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
impl From<crate::W<TIMX_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` writer - Update generation"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMX_EGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Update generation"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_egr](index.html) module"]
pub struct TIMX_EGR_SPEC;
impl crate::RegisterSpec for TIMX_EGR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [timx_egr::W](W) writer structure"]
impl crate::Writable for TIMX_EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_EGR to value 0"]
impl crate::Resettable for TIMX_EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}