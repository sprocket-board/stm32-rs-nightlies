#[doc = "Register `M5SR` reader"]
pub struct R(crate::R<M5SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M5SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M5SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M5SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M5SR` writer"]
pub struct W(crate::W<M5SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M5SR_SPEC>;
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
impl From<crate::W<M5SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M5SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FEC` reader - Failing error code"]
pub type FEC_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FEC` writer - Failing error code"]
pub type FEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M5SR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FEC_R {
        FEC_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&mut self) -> FEC_W<0> {
        FEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m5sr](index.html) module"]
pub struct M5SR_SPEC;
impl crate::RegisterSpec for M5SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m5sr::R](R) reader structure"]
impl crate::Readable for M5SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m5sr::W](W) writer structure"]
impl crate::Writable for M5SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M5SR to value 0"]
impl crate::Resettable for M5SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
