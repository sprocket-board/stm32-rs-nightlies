#[doc = "Register `CSGCMCCM1R` reader"]
pub struct R(crate::R<CSGCMCCM1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM1R` writer"]
pub struct W(crate::W<CSGCMCCM1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM1R_SPEC>;
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
impl From<crate::W<CSGCMCCM1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM1` reader - CSGCMCCM1"]
pub type CSGCMCCM1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM1` writer - CSGCMCCM1"]
pub type CSGCMCCM1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSGCMCCM1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    pub fn csgcmccm1(&self) -> CSGCMCCM1_R {
        CSGCMCCM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM1"]
    #[inline(always)]
    pub fn csgcmccm1(&mut self) -> CSGCMCCM1_W<0> {
        CSGCMCCM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm1r](index.html) module"]
pub struct CSGCMCCM1R_SPEC;
impl crate::RegisterSpec for CSGCMCCM1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm1r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm1r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM1R to value 0"]
impl crate::Resettable for CSGCMCCM1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
