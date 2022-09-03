#[doc = "Register `DMA_S2PAR` reader"]
pub struct R(crate::R<DMA_S2PAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_S2PAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_S2PAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_S2PAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_S2PAR` writer"]
pub struct W(crate::W<DMA_S2PAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_S2PAR_SPEC>;
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
impl From<crate::W<DMA_S2PAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_S2PAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAR` reader - PAR"]
pub type PAR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PAR` writer - PAR"]
pub type PAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DMA_S2PAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    pub fn par(&self) -> PAR_R {
        PAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PAR"]
    #[inline(always)]
    pub fn par(&mut self) -> PAR_W<0> {
        PAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA stream 2 peripheral address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_s2par](index.html) module"]
pub struct DMA_S2PAR_SPEC;
impl crate::RegisterSpec for DMA_S2PAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_s2par::R](R) reader structure"]
impl crate::Readable for DMA_S2PAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_s2par::W](W) writer structure"]
impl crate::Writable for DMA_S2PAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_S2PAR to value 0"]
impl crate::Resettable for DMA_S2PAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
