#[doc = "Register `TIM6_CCR1` reader"]
pub struct R(crate::R<TIM6_CCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM6_CCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM6_CCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM6_CCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM6_CCR1` writer"]
pub struct W(crate::W<TIM6_CCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM6_CCR1_SPEC>;
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
impl From<crate::W<TIM6_CCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM6_CCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR1` reader - CCR1"]
pub type CCR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CCR1` writer - CCR1"]
pub type CCR1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM6_CCR1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    pub fn ccr1(&self) -> CCR1_R {
        CCR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR1"]
    #[inline(always)]
    pub fn ccr1(&mut self) -> CCR1_W<0> {
        CCR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM6 capture/compare register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim6_ccr1](index.html) module"]
pub struct TIM6_CCR1_SPEC;
impl crate::RegisterSpec for TIM6_CCR1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim6_ccr1::R](R) reader structure"]
impl crate::Readable for TIM6_CCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim6_ccr1::W](W) writer structure"]
impl crate::Writable for TIM6_CCR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM6_CCR1 to value 0"]
impl crate::Resettable for TIM6_CCR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
