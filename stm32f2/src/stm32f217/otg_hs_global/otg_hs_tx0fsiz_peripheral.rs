#[doc = "Register `OTG_HS_TX0FSIZ_Peripheral` reader"]
pub struct R(crate::R<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HS_TX0FSIZ_Peripheral` writer"]
pub struct W(crate::W<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>;
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
impl From<crate::W<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HS_TX0FSIZ_PERIPHERAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HS_TX0FSIZ_PERIPHERAL_SPEC, u16, u16, 16, O>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OTG_HS_TX0FSIZ_PERIPHERAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<0> {
        TX0FSA_W::new(self)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&mut self) -> TX0FD_W<16> {
        TX0FD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint 0 transmit FIFO size (peripheral mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hs_tx0fsiz_peripheral](index.html) module"]
pub struct OTG_HS_TX0FSIZ_PERIPHERAL_SPEC;
impl crate::RegisterSpec for OTG_HS_TX0FSIZ_PERIPHERAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hs_tx0fsiz_peripheral::R](R) reader structure"]
impl crate::Readable for OTG_HS_TX0FSIZ_PERIPHERAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hs_tx0fsiz_peripheral::W](W) writer structure"]
impl crate::Writable for OTG_HS_TX0FSIZ_PERIPHERAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HS_TX0FSIZ_Peripheral to value 0x0200"]
impl crate::Resettable for OTG_HS_TX0FSIZ_PERIPHERAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}