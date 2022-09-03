#[doc = "Register `RDATA` reader"]
pub struct R(crate::R<RDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDATA` writer"]
pub struct W(crate::W<RDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDATA_SPEC>;
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
impl From<crate::W<RDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RES` reader - Function result"]
pub type RES_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RES` writer - Function result"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RDATA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Function result"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Function result"]
    #[inline(always)]
    pub fn res(&mut self) -> RES_W<0> {
        RES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u32) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Result register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](index.html) module"]
pub struct RDATA_SPEC;
impl crate::RegisterSpec for RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdata::R](R) reader structure"]
impl crate::Readable for RDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdata::W](W) writer structure"]
impl crate::Writable for RDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RDATA to value 0"]
impl crate::Resettable for RDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
