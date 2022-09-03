#[doc = "Register `OTG_HCDMA7` reader"]
pub struct R(crate::R<OTG_HCDMA7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMA7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMA7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMA7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCDMA7` writer"]
pub struct W(crate::W<OTG_HCDMA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCDMA7_SPEC>;
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
impl From<crate::W<OTG_HCDMA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCDMA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMAADDR"]
pub type DMAADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMAADDR` writer - DMAADDR"]
pub type DMAADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OTG_HCDMA7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAADDR"]
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
#[doc = "OTG host channel 7 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma7](index.html) module"]
pub struct OTG_HCDMA7_SPEC;
impl crate::RegisterSpec for OTG_HCDMA7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcdma7::R](R) reader structure"]
impl crate::Readable for OTG_HCDMA7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcdma7::W](W) writer structure"]
impl crate::Writable for OTG_HCDMA7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HCDMA7 to value 0"]
impl crate::Resettable for OTG_HCDMA7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
