#[doc = "Register `CSGCM1R` reader"]
pub struct R(crate::R<CSGCM1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSGCM1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSGCM1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSGCM1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSGCM1R` writer"]
pub struct W(crate::W<CSGCM1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSGCM1R_SPEC>;
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
impl From<crate::W<CSGCM1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSGCM1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM1` reader - CSGCM1"]
pub type CSGCM1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCM1` writer - CSGCM1"]
pub type CSGCM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSGCM1R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    pub fn csgcm1(&self) -> CSGCM1_R {
        CSGCM1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM1"]
    #[inline(always)]
    pub fn csgcm1(&mut self) -> CSGCM1_W<0> {
        CSGCM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csgcm1r](index.html) module"]
pub struct CSGCM1R_SPEC;
impl crate::RegisterSpec for CSGCM1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csgcm1r::R](R) reader structure"]
impl crate::Readable for CSGCM1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csgcm1r::W](W) writer structure"]
impl crate::Writable for CSGCM1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSGCM1R to value 0"]
impl crate::Resettable for CSGCM1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
