#[doc = "Register `RCC_CECCKSELR` reader"]
pub struct R(crate::R<RCC_CECCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CECCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CECCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CECCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CECCKSELR` writer"]
pub struct W(crate::W<RCC_CECCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CECCKSELR_SPEC>;
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
impl From<crate::W<RCC_CECCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CECCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CECSRC` reader - CECSRC"]
pub type CECSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CECSRC` writer - CECSRC"]
pub type CECSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCC_CECCKSELR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CECSRC_R {
        CECSRC_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CECSRC"]
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CECSRC_W<0> {
        CECSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the CEC-HDMI.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cecckselr](index.html) module"]
pub struct RCC_CECCKSELR_SPEC;
impl crate::RegisterSpec for RCC_CECCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cecckselr::R](R) reader structure"]
impl crate::Readable for RCC_CECCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cecckselr::W](W) writer structure"]
impl crate::Writable for RCC_CECCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CECCKSELR to value 0"]
impl crate::Resettable for RCC_CECCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
