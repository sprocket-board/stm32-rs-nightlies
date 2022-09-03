#[doc = "Register `PERCR` reader"]
pub struct R(crate::R<PERCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERCR` writer"]
pub struct W(crate::W<PERCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERCR_SPEC>;
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
impl From<crate::W<PERCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERx` reader - Timerx Period value"]
pub type PERX_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PERx` writer - Timerx Period value"]
pub type PERX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PERX_R {
        PERX_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&mut self) -> PERX_W<0> {
        PERX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [percr](index.html) module"]
pub struct PERCR_SPEC;
impl crate::RegisterSpec for PERCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [percr::R](R) reader structure"]
impl crate::Readable for PERCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [percr::W](W) writer structure"]
impl crate::Writable for PERCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERCR to value 0xffff"]
impl crate::Resettable for PERCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
