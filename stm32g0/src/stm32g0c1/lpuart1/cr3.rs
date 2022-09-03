#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register)."]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register)."]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAR` reader - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_R = crate::BitReader<bool>;
#[doc = "Field `DMAR` writer - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_R = crate::BitReader<bool>;
#[doc = "Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `RTSE` reader - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type RTSE_R = crate::BitReader<bool>;
#[doc = "Field `RTSE` writer - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSE` reader - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)"]
pub type CTSE_R = crate::BitReader<bool>;
#[doc = "Field `CTSE` writer - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data."]
pub type OVRDIS_R = crate::BitReader<bool>;
#[doc = "Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data."]
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_R = crate::BitReader<bool>;
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DEM_R = crate::BitReader<bool>;
#[doc = "Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DEP` reader - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DEP_R = crate::BitReader<bool>;
#[doc = "Field `DEP` writer - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `WUS` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type WUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WUS` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type WUFIE_R = crate::BitReader<bool>;
#[doc = "Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved."]
pub type RXFTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved."]
pub type RXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved."]
pub type TXFTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved."]
pub type TXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register)."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data."]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error or noise flag (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 in the LPUART_ISR register)."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable receiver This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable transmitter This bit is set/reset by software"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the LPUART is disabled (UEÂ =Â 0)"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the LPUART_RDR register. This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data."]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the LPUART is disabled (UEÂ =Â 0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0). Note: If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the LPUART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<23> {
        TXFTIE_W::new(self)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<25> {
        RXFTCFG_W::new(self)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<28> {
        RXFTIE_W::new(self)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved."]
    #[inline(always)]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<29> {
        TXFTCFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
