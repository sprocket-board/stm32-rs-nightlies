#[doc = "Register `HCDMA11` reader"]
pub struct R(crate::R<HCDMA11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCDMA11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCDMA11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCDMA11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HCDMA11` writer"]
pub struct W(crate::W<HCDMA11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HCDMA11_SPEC>;
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
impl From<crate::W<HCDMA11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HCDMA11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HCDMA11_SPEC, u32, u32, 32, O>;
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
#[doc = "OTG_HS host channel-11 DMA address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcdma11](index.html) module"]
pub struct HCDMA11_SPEC;
impl crate::RegisterSpec for HCDMA11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcdma11::R](R) reader structure"]
impl crate::Readable for HCDMA11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hcdma11::W](W) writer structure"]
impl crate::Writable for HCDMA11_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HCDMA11 to value 0"]
impl crate::Resettable for HCDMA11_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
