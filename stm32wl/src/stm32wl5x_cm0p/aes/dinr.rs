#[doc = "Register `DINR` reader"]
pub struct R(crate::R<DINR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DINR` writer"]
pub struct W(crate::W<DINR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DINR_SPEC>;
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
impl From<crate::W<DINR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DINR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIN` reader - Data Input Register"]
pub type DIN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIN` writer - Data Input Register"]
pub type DIN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, DINR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    pub fn din(&self) -> DIN_R {
        DIN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Input Register"]
    #[inline(always)]
    pub fn din(&mut self) -> DIN_W<0> {
        DIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "data input register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr](index.html) module"]
pub struct DINR_SPEC;
impl crate::RegisterSpec for DINR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr::R](R) reader structure"]
impl crate::Readable for DINR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dinr::W](W) writer structure"]
impl crate::Writable for DINR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DINR to value 0"]
impl crate::Resettable for DINR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
