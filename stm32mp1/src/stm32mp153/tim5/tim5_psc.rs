#[doc = "Register `TIM5_PSC` reader"]
pub struct R(crate::R<TIM5_PSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM5_PSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM5_PSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM5_PSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM5_PSC` writer"]
pub struct W(crate::W<TIM5_PSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM5_PSC_SPEC>;
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
impl From<crate::W<TIM5_PSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM5_PSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PSC` reader - PSC"]
pub type PSC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PSC` writer - PSC"]
pub type PSC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIM5_PSC_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&self) -> PSC_R {
        PSC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - PSC"]
    #[inline(always)]
    pub fn psc(&mut self) -> PSC_W<0> {
        PSC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM5 prescaler\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim5_psc](index.html) module"]
pub struct TIM5_PSC_SPEC;
impl crate::RegisterSpec for TIM5_PSC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tim5_psc::R](R) reader structure"]
impl crate::Readable for TIM5_PSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim5_psc::W](W) writer structure"]
impl crate::Writable for TIM5_PSC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM5_PSC to value 0"]
impl crate::Resettable for TIM5_PSC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
