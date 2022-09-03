#[doc = "Register `TIM14_EGR` writer"]
pub struct W(crate::W<TIM14_EGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM14_EGR_SPEC>;
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
impl From<crate::W<TIM14_EGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM14_EGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UG` writer - UG"]
pub type UG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM14_EGR_SPEC, bool, O>;
#[doc = "Field `CC1G` writer - CC1G"]
pub type CC1G_W<'a, const O: u8> = crate::BitWriter<'a, u16, TIM14_EGR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - UG"]
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<0> {
        UG_W::new(self)
    }
    #[doc = "Bit 1 - CC1G"]
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<1> {
        CC1G_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM14 event generation register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim14_egr](index.html) module"]
pub struct TIM14_EGR_SPEC;
impl crate::RegisterSpec for TIM14_EGR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [tim14_egr::W](W) writer structure"]
impl crate::Writable for TIM14_EGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM14_EGR to value 0"]
impl crate::Resettable for TIM14_EGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
