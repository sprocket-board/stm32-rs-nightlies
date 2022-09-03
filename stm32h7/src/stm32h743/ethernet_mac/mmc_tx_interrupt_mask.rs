#[doc = "Register `MMC_TX_INTERRUPT_MASK` reader"]
pub struct R(crate::R<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMC_TX_INTERRUPT_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMC_TX_INTERRUPT_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMC_TX_INTERRUPT_MASK` writer"]
pub struct W(crate::W<MMC_TX_INTERRUPT_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMC_TX_INTERRUPT_MASK_SPEC>;
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
impl From<crate::W<MMC_TX_INTERRUPT_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMC_TX_INTERRUPT_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSCOLGPIM` reader - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
pub type TXSCOLGPIM_R = crate::BitReader<bool>;
#[doc = "Field `TXSCOLGPIM` writer - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
pub type TXSCOLGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `TXMCOLGPIM` reader - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
pub type TXMCOLGPIM_R = crate::BitReader<bool>;
#[doc = "Field `TXMCOLGPIM` writer - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
pub type TXMCOLGPIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `TXGPKTIM` reader - MMC Transmit Good Packet Counter Interrupt Mask"]
pub type TXGPKTIM_R = crate::BitReader<bool>;
#[doc = "Field `TXGPKTIM` writer - MMC Transmit Good Packet Counter Interrupt Mask"]
pub type TXGPKTIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `TXLPIUSCIM` reader - MMC Transmit LPI microsecond counter interrupt Mask"]
pub type TXLPIUSCIM_R = crate::BitReader<bool>;
#[doc = "Field `TXLPIUSCIM` writer - MMC Transmit LPI microsecond counter interrupt Mask"]
pub type TXLPIUSCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
#[doc = "Field `TXLPITRCIM` reader - MMC Transmit LPI transition counter interrupt Mask"]
pub type TXLPITRCIM_R = crate::BitReader<bool>;
#[doc = "Field `TXLPITRCIM` writer - MMC Transmit LPI transition counter interrupt Mask"]
pub type TXLPITRCIM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MMC_TX_INTERRUPT_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W<14> {
        TXSCOLGPIM_W::new(self)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W<15> {
        TXMCOLGPIM_W::new(self)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W<21> {
        TXGPKTIM_W::new(self)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W<26> {
        TXLPIUSCIM_W::new(self)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpitrcim(&mut self) -> TXLPITRCIM_W<27> {
        TXLPITRCIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Tx interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmc_tx_interrupt_mask](index.html) module"]
pub struct MMC_TX_INTERRUPT_MASK_SPEC;
impl crate::RegisterSpec for MMC_TX_INTERRUPT_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmc_tx_interrupt_mask::R](R) reader structure"]
impl crate::Readable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmc_tx_interrupt_mask::W](W) writer structure"]
impl crate::Writable for MMC_TX_INTERRUPT_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MMC_TX_INTERRUPT_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
