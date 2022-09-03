#[doc = "Register `CRYP_CSGCM5R` reader"]
pub struct R(crate::R<CRYP_CSGCM5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYP_CSGCM5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYP_CSGCM5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYP_CSGCM5R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYP_CSGCM5R` writer"]
pub struct W(crate::W<CRYP_CSGCM5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYP_CSGCM5R_SPEC>;
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
impl From<crate::W<CRYP_CSGCM5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYP_CSGCM5R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSGCM5` reader - CSGCM5"]
pub type CSGCM5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CSGCM5` writer - CSGCM5"]
pub type CSGCM5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CRYP_CSGCM5R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - CSGCM5"]
    #[inline(always)]
    pub fn csgcm5(&self) -> CSGCM5_R {
        CSGCM5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSGCM5"]
    #[inline(always)]
    pub fn csgcm5(&mut self) -> CSGCM5_W<0> {
        CSGCM5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Please refer to Section39.6.21: CRYP context swap GCM-CCM registers (CRYP_CSGCMCCMxR) for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cryp_csgcm5r](index.html) module"]
pub struct CRYP_CSGCM5R_SPEC;
impl crate::RegisterSpec for CRYP_CSGCM5R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cryp_csgcm5r::R](R) reader structure"]
impl crate::Readable for CRYP_CSGCM5R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cryp_csgcm5r::W](W) writer structure"]
impl crate::Writable for CRYP_CSGCM5R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYP_CSGCM5R to value 0"]
impl crate::Resettable for CRYP_CSGCM5R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
