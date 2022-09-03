#[doc = "Register `OTG_HS_HPTXFSIZ` reader"]
pub struct R(crate::R<OTG_HS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_HPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_HPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_HPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_HPTXFSIZ` writer"]
pub struct W(crate::W<OTG_HS_HPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_HPTXFSIZ_SPEC>;
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
impl From<crate::W<OTG_HS_HPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_HPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PTXSA` reader - Host periodic TxFIFO start address"]
pub type PTXSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTXSA` writer - Host periodic TxFIFO start address"]
pub type PTXSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HS_HPTXFSIZ_SPEC, u16, u16, 16, O>;
#[doc = "Field `PTXFD` reader - Host periodic TxFIFO depth"]
pub type PTXFD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PTXFD` writer - Host periodic TxFIFO depth"]
pub type PTXFD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HS_HPTXFSIZ_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&self) -> PTXSA_R {
        PTXSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&self) -> PTXFD_R {
        PTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Host periodic TxFIFO start address"]
    #[inline(always)]
    pub fn ptxsa(&mut self) -> PTXSA_W<0> {
        PTXSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Host periodic TxFIFO depth"]
    #[inline(always)]
    pub fn ptxfd(&mut self) -> PTXFD_W<16> {
        PTXFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_HS Host periodic transmit FIFO size register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_hptxfsiz](index.html) module"]
pub struct OTG_HS_HPTXFSIZ_SPEC;
impl crate::RegisterSpec for OTG_HS_HPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_hptxfsiz::R](R) reader structure"]
impl crate::Readable for OTG_HS_HPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_hptxfsiz::W](W) writer structure"]
impl crate::Writable for OTG_HS_HPTXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_HPTXFSIZ to value 0x0200_0600"]
impl crate::Resettable for OTG_HS_HPTXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0600
    }
}
