#[doc = "Register `TIMx_TISEL` reader"]
pub struct R(crate::R<TIMX_TISEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMX_TISEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMX_TISEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMX_TISEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMx_TISEL` writer"]
pub struct W(crate::W<TIMX_TISEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMX_TISEL_SPEC>;
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
impl From<crate::W<TIMX_TISEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMX_TISEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMX_TISEL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<0> {
        TI1SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM17 input selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timx_tisel](index.html) module"]
pub struct TIMX_TISEL_SPEC;
impl crate::RegisterSpec for TIMX_TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timx_tisel::R](R) reader structure"]
impl crate::Readable for TIMX_TISEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timx_tisel::W](W) writer structure"]
impl crate::Writable for TIMX_TISEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMx_TISEL to value 0"]
impl crate::Resettable for TIMX_TISEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
