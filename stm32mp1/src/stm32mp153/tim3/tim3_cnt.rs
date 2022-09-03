#[doc = "Register `TIM3_CNT` reader"]
pub struct R(crate::R<TIM3_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIM3_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIM3_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIM3_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIM3_CNT` writer"]
pub struct W(crate::W<TIM3_CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIM3_CNT_SPEC>;
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
impl From<crate::W<TIM3_CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIM3_CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT` reader - CNT"]
pub type CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT` writer - CNT"]
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIM3_CNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `UIFCPY` reader - UIFCPY"]
pub type UIFCPY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - UIFCPY"]
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CNT"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIM3 counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tim3_cnt](index.html) module"]
pub struct TIM3_CNT_SPEC;
impl crate::RegisterSpec for TIM3_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tim3_cnt::R](R) reader structure"]
impl crate::Readable for TIM3_CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tim3_cnt::W](W) writer structure"]
impl crate::Writable for TIM3_CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIM3_CNT to value 0"]
impl crate::Resettable for TIM3_CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
