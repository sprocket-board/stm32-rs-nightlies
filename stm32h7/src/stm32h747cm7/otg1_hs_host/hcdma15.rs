#[doc = "Register `HCDMA15` reader"]
pub struct R(crate::R<HCDMA15_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA15_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA15_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA15_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA15` writer"]
pub struct W(crate::W<HCDMA15_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA15_SPEC>;
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
impl From<crate::W<HCDMA15_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA15_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA15_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<0> {
        DMAADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS host channel-15 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma15](index.html) module"]
pub struct HCDMA15_SPEC;
impl crate::RegisterSpec for HCDMA15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma15::R](R) reader structure"]
impl crate::Readable for HCDMA15_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma15::W](W) writer structure"]
impl crate::Writable for HCDMA15_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA15 to value 0"]
impl crate::Resettable for HCDMA15_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}