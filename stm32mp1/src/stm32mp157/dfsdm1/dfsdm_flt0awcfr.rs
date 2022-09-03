#[doc = "Register `DFSDM_FLT0AWCFR` reader"]
pub struct R(crate::R<DFSDM_FLT0AWCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT0AWCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT0AWCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT0AWCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSDM_FLT0AWCFR` writer"]
pub struct W(crate::W<DFSDM_FLT0AWCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSDM_FLT0AWCFR_SPEC>;
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
impl From<crate::W<DFSDM_FLT0AWCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSDM_FLT0AWCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRAWLTF` reader - CLRAWLTF"]
pub type CLRAWLTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRAWLTF` writer - CLRAWLTF"]
pub type CLRAWLTF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT0AWCFR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLRAWHTF` reader - CLRAWHTF"]
pub type CLRAWHTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRAWHTF` writer - CLRAWHTF"]
pub type CLRAWHTF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DFSDM_FLT0AWCFR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - CLRAWLTF"]
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CLRAWHTF"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CLRAWLTF"]
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<0> {
        CLRAWLTF_W::new(self)
    }
    #[doc = "Bits 8:15 - CLRAWHTF"]
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> CLRAWHTF_W<8> {
        CLRAWHTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DFSDM filter 0 analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt0awcfr](index.html) module"]
pub struct DFSDM_FLT0AWCFR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT0AWCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt0awcfr::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT0AWCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsdm_flt0awcfr::W](W) writer structure"]
impl crate::Writable for DFSDM_FLT0AWCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DFSDM_FLT0AWCFR to value 0"]
impl crate::Resettable for DFSDM_FLT0AWCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
