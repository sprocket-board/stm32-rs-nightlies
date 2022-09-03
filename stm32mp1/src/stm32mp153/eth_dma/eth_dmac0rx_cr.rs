#[doc = "Register `ETH_DMAC0RxCR` reader"]
pub struct R(crate::R<ETH_DMAC0RX_CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETH_DMAC0RX_CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETH_DMAC0RX_CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETH_DMAC0RX_CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETH_DMAC0RxCR` writer"]
pub struct W(crate::W<ETH_DMAC0RX_CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETH_DMAC0RX_CR_SPEC>;
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
impl From<crate::W<ETH_DMAC0RX_CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETH_DMAC0RX_CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SR` reader - Start or Stop Receive Command"]
pub type SR_R = crate::BitReader<bool>;
#[doc = "Field `SR` writer - Start or Stop Receive Command"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, bool, O>;
#[doc = "Field `RBSZ` reader - Receive Buffer size"]
pub type RBSZ_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RBSZ` writer - Receive Buffer size"]
pub type RBSZ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u16, u16, 14, O>;
#[doc = "Field `RXPBL` reader - RXPBL"]
pub type RXPBL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPBL` writer - RXPBL"]
pub type RXPBL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u8, u8, 6, O>;
#[doc = "Field `RQOS` reader - RQOS"]
pub type RQOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RQOS` writer - RQOS"]
pub type RQOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RPF` reader - DMA Rx Channel Packet Flush"]
pub type RPF_R = crate::BitReader<bool>;
#[doc = "Field `RPF` writer - DMA Rx Channel Packet Flush"]
pub type RPF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETH_DMAC0RX_CR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:27 - RQOS"]
    #[inline(always)]
    pub fn rqos(&self) -> RQOS_R {
        RQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<0> {
        SR_W::new(self)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W<1> {
        RBSZ_W::new(self)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W<16> {
        RXPBL_W::new(self)
    }
    #[doc = "Bits 24:27 - RQOS"]
    #[inline(always)]
    pub fn rqos(&mut self) -> RQOS_W<24> {
        RQOS_W::new(self)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W<31> {
        RPF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel receive control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eth_dmac0rx_cr](index.html) module"]
pub struct ETH_DMAC0RX_CR_SPEC;
impl crate::RegisterSpec for ETH_DMAC0RX_CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eth_dmac0rx_cr::R](R) reader structure"]
impl crate::Readable for ETH_DMAC0RX_CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eth_dmac0rx_cr::W](W) writer structure"]
impl crate::Writable for ETH_DMAC0RX_CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ETH_DMAC0RxCR to value 0x8000"]
impl crate::Resettable for ETH_DMAC0RX_CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
