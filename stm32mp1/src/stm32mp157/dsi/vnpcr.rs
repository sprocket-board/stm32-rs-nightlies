#[doc = "Register `VNPCR` reader"]
pub struct R(crate::R<VNPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VNPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VNPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VNPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VNPCR` writer"]
pub struct W(crate::W<VNPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VNPCR_SPEC>;
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
impl From<crate::W<VNPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VNPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub type NPSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `NPSIZE` writer - NPSIZE"]
pub type NPSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VNPCR_SPEC, u16, u16, 13, O>;
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&mut self) -> NPSIZE_W<0> {
        NPSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host video null packet configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vnpcr](index.html) module"]
pub struct VNPCR_SPEC;
impl crate::RegisterSpec for VNPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vnpcr::R](R) reader structure"]
impl crate::Readable for VNPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vnpcr::W](W) writer structure"]
impl crate::Writable for VNPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets VNPCR to value 0"]
impl crate::Resettable for VNPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
