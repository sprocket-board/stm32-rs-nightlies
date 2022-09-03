#[doc = "Register `FLT1AWCFR` reader"]
pub struct R(crate::R<FLT1AWCFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1AWCFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1AWCFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1AWCFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT1AWCFR` writer"]
pub struct W(crate::W<FLT1AWCFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT1AWCFR_SPEC>;
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
impl From<crate::W<FLT1AWCFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT1AWCFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRAWLTF` reader - Clear the analog watchdog low threshold flag"]
pub type CLRAWLTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRAWLTF` writer - Clear the analog watchdog low threshold flag"]
pub type CLRAWLTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1AWCFR_SPEC, u8, u8, 8, O>;
#[doc = "Field `CLRAWHTF` reader - Clear the analog watchdog high threshold flag"]
pub type CLRAWHTF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLRAWHTF` writer - Clear the analog watchdog high threshold flag"]
pub type CLRAWHTF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1AWCFR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&self) -> CLRAWLTF_R {
        CLRAWLTF_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> CLRAWHTF_R {
        CLRAWHTF_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&mut self) -> CLRAWLTF_W<0> {
        CLRAWLTF_W::new(self)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
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
#[doc = "analog watchdog clear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awcfr](index.html) module"]
pub struct FLT1AWCFR_SPEC;
impl crate::RegisterSpec for FLT1AWCFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt1awcfr::R](R) reader structure"]
impl crate::Readable for FLT1AWCFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt1awcfr::W](W) writer structure"]
impl crate::Writable for FLT1AWCFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT1AWCFR to value 0"]
impl crate::Resettable for FLT1AWCFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
