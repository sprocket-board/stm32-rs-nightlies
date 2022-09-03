#[doc = "Register `CSGCMCCM0R` reader"]
pub struct R(crate::R<CSGCMCCM0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCMCCM0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCMCCM0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCMCCM0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCMCCM0R` writer"]
pub struct W(crate::W<CSGCMCCM0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCMCCM0R_SPEC>;
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
impl From<crate::W<CSGCMCCM0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCMCCM0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCMCCM0` reader - CSGCMCCM0"]
pub type CSGCMCCM0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCMCCM0` writer - CSGCMCCM0"]
pub type CSGCMCCM0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CSGCMCCM0R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&self) -> CSGCMCCM0_R {
        CSGCMCCM0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCMCCM0"]
    #[inline(always)]
    pub fn csgcmccm0(&mut self) -> CSGCMCCM0_W<0> {
        CSGCMCCM0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcmccm0r](index.html) module"]
pub struct CSGCMCCM0R_SPEC;
impl crate::RegisterSpec for CSGCMCCM0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcmccm0r::R](R) reader structure"]
impl crate::Readable for CSGCMCCM0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcmccm0r::W](W) writer structure"]
impl crate::Writable for CSGCMCCM0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCMCCM0R to value 0"]
impl crate::Resettable for CSGCMCCM0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
