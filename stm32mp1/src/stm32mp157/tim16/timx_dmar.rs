#[doc = "Register `TIMx_DMAR` reader"]
pub struct R(crate::R<TIMX_DMAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_DMAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_DMAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_DMAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_DMAR` writer"]
pub struct W(crate::W<TIMX_DMAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_DMAR_SPEC>;
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
impl From<crate::W<TIMX_DMAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_DMAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAB` reader - DMAB"]
pub type DMAB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DMAB` writer - DMAB"]
pub type DMAB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TIMX_DMAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DMAB"]
    #[inline(always)]
    pub fn dmab(&self) -> DMAB_R {
        DMAB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DMAB"]
    #[inline(always)]
    pub fn dmab(&mut self) -> DMAB_W<0> {
        DMAB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM16/TIM17 DMA address for full transfer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_dmar](index.html) module"]
pub struct TIMX_DMAR_SPEC;
impl crate::RegisterSpec for TIMX_DMAR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [timx_dmar::R](R) reader structure"]
impl crate::Readable for TIMX_DMAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_dmar::W](W) writer structure"]
impl crate::Writable for TIMX_DMAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_DMAR to value 0"]
impl crate::Resettable for TIMX_DMAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
