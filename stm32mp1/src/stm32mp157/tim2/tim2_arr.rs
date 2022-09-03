#[doc = "Register `TIM2_ARR` reader"]
pub struct R(crate::R<TIM2_ARR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM2_ARR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM2_ARR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM2_ARR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM2_ARR` writer"]
pub struct W(crate::W<TIM2_ARR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM2_ARR_SPEC>;
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
impl From<crate::W<TIM2_ARR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM2_ARR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARR` reader - ARR"]
pub type ARR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ARR` writer - ARR"]
pub type ARR_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM2_ARR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&self) -> ARR_R {
        ARR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - ARR"]
    #[inline(always)]
    pub fn arr(&mut self) -> ARR_W<0> {
        ARR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM2 auto-reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim2_arr](index.html) module"]
pub struct TIM2_ARR_SPEC;
impl crate::RegisterSpec for TIM2_ARR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim2_arr::R](R) reader structure"]
impl crate::Readable for TIM2_ARR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim2_arr::W](W) writer structure"]
impl crate::Writable for TIM2_ARR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM2_ARR to value 0xffff"]
impl crate::Resettable for TIM2_ARR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
