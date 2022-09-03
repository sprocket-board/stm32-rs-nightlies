#[doc = "Register `TIM15_RCR` reader"]
pub struct R(crate::R<TIM15_RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM15_RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM15_RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM15_RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM15_RCR` writer"]
pub struct W(crate::W<TIM15_RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM15_RCR_SPEC>;
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
impl From<crate::W<TIM15_RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM15_RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REP` reader - REP"]
pub type REP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REP` writer - REP"]
pub type REP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM15_RCR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - REP"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - REP"]
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
#[doc = "TIM15 repetition counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim15_rcr](index.html) module"]
pub struct TIM15_RCR_SPEC;
impl crate::RegisterSpec for TIM15_RCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim15_rcr::R](R) reader structure"]
impl crate::Readable for TIM15_RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim15_rcr::W](W) writer structure"]
impl crate::Writable for TIM15_RCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM15_RCR to value 0"]
impl crate::Resettable for TIM15_RCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
