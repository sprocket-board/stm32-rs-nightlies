#[doc = "Register `CSGCMCCM7R` reader"]
pub struct R(crate::R<CSGCMCCM7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM7R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM7R` writer"]
pub struct W(crate::W<CSGCMCCM7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM7R_SPEC>;
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
impl From<crate::W<CSGCMCCM7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM7R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM7R` reader - CSGCMCCM7R"]
pub type CSGCMCCM7R_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM7R` writer - CSGCMCCM7R"]
pub type CSGCMCCM7R_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSGCMCCM7R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM7R"]
    #[inline(always)]
    pub fn csgcmccm7r(&self) -> CSGCMCCM7R_R {
        CSGCMCCM7R_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM7R"]
    #[inline(always)]
    pub fn csgcmccm7r(&mut self) -> CSGCMCCM7R_W<0> {
        CSGCMCCM7R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm7r](index.html) module"]
pub struct CSGCMCCM7R_SPEC;
impl crate::RegisterSpec for CSGCMCCM7R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm7r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM7R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm7r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM7R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM7R to value 0"]
impl crate::Resettable for CSGCMCCM7R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
