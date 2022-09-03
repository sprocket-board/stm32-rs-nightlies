#[doc = "Register `TIM5_RCR` reader"]
pub struct R(crate::R<TIM5_RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_RCR` writer"]
pub struct W(crate::W<TIM5_RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_RCR_SPEC>;
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
impl From<crate::W<TIM5_RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - REP"]
pub type REP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REP` writer - REP"]
pub type REP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_RCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - REP"]
    #[inline(always)]
    pub fn rep(&mut self) -> REP_W<0> {
        REP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_rcr](index.html) module"]
pub struct TIM5_RCR_SPEC;
impl crate::RegisterSpec for TIM5_RCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim5_rcr::R](R) reader structure"]
impl crate::Readable for TIM5_RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_rcr::W](W) writer structure"]
impl crate::Writable for TIM5_RCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM5_RCR to value 0"]
impl crate::Resettable for TIM5_RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
