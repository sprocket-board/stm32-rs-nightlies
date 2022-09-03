#[doc = "Register `VREFBUF_CCR` reader"]
pub struct R(crate::R<VREFBUF_CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VREFBUF_CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VREFBUF_CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VREFBUF_CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VREFBUF_CCR` writer"]
pub struct W(crate::W<VREFBUF_CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VREFBUF_CCR_SPEC>;
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
impl From<crate::W<VREFBUF_CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VREFBUF_CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIM` reader - TRIM"]
pub type TRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIM` writer - TRIM"]
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VREFBUF_CCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - TRIM"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - TRIM"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W<0> {
        TRIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREFBUF calibration control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vrefbuf_ccr](index.html) module"]
pub struct VREFBUF_CCR_SPEC;
impl crate::RegisterSpec for VREFBUF_CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vrefbuf_ccr::R](R) reader structure"]
impl crate::Readable for VREFBUF_CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vrefbuf_ccr::W](W) writer structure"]
impl crate::Writable for VREFBUF_CCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VREFBUF_CCR to value 0"]
impl crate::Resettable for VREFBUF_CCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
