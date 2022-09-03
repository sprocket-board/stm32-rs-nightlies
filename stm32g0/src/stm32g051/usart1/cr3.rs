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
#[doc = "Field `EIE` reader - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `IREN` reader - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IREN_R = crate::BitReader<bool>;
#[doc = "Field `IREN` writer - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `IRLP` reader - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IRLP_R = crate::BitReader<bool>;
#[doc = "Field `IRLP` writer - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `HDSEL` reader - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type HDSEL_R = crate::BitReader<bool>;
#[doc = "Field `HDSEL` writer - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `NACK` reader - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `NACK` writer - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `SCEN` reader - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCEN_R = crate::BitReader<bool>;
#[doc = "Field `SCEN` writer - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAR` reader - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_R = crate::BitReader<bool>;
#[doc = "Field `DMAR` writer - DMA enable receiver This bit is set/reset by software"]
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DMAT` reader - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_R = crate::BitReader<bool>;
#[doc = "Field `DMAT` writer - DMA enable transmitter This bit is set/reset by software"]
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `RTSE` reader - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTSE_R = crate::BitReader<bool>;
#[doc = "Field `RTSE` writer - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSE` reader - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSE_R = crate::BitReader<bool>;
#[doc = "Field `CTSE` writer - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSIE_R = crate::BitReader<bool>;
#[doc = "Field `CTSIE` writer - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `ONEBIT` reader - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type ONEBIT_R = crate::BitReader<bool>;
#[doc = "Field `ONEBIT` writer - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `OVRDIS` reader - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OVRDIS_R = crate::BitReader<bool>;
#[doc = "Field `OVRDIS` writer - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_R = crate::BitReader<bool>;
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DEM` reader - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub type DEM_R = crate::BitReader<bool>;
#[doc = "Field `DEM` writer - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `DEP` reader - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEP_R = crate::BitReader<bool>;
#[doc = "Field `DEP` writer - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCARCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SCARCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `WUS` reader - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub type WUS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WUS` writer - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 2, O>;
#[doc = "Field `WUFIE` reader - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub type WUFIE_R = crate::BitReader<bool>;
#[doc = "Field `WUFIE` writer - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `TXFTIE` reader - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_R = crate::BitReader<bool>;
#[doc = "Field `TXFTIE` writer - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type TXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `TCBGTIE` reader - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type TCBGTIE_R = crate::BitReader<bool>;
#[doc = "Field `TCBGTIE` writer - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type TCBGTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RXFTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration Remaining combinations: Reserved"]
pub type RXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_R = crate::BitReader<bool>;
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
pub type RXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TXFTCFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration Remaining combinations: Reserved"]
pub type TXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable Error Interrupt Enable Bit is required to enable interrupt generation in case of a framing error, overrun error noise flag or SPI slave underrun error (FEÂ =Â 1 or OREÂ =Â 1 or NEÂ =Â 1 or UDR = 1 in the USART_ISR register)."]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable This bit is set and cleared by software. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power This bit is used for selecting between normal and low-power IrDA modes This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If IrDA mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection Selection of Single-wire Half-duplex mode This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn nack(&mut self) -> NACK_W<4> {
        NACK_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable This bit is used for enabling Smartcard mode. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
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
    #[doc = "Bit 8 - RTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 10 - CTS interrupt enable Note: If the hardware flow control feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 11 - One sample bit method enable This bit enables the user to select the sample method. When the one sample bit method is selected the noise detection flag (NE) is disabled. This bit can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn onebit(&mut self) -> ONEBIT_W<11> {
        ONEBIT_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Disable This bit is used to disable the receive overrun detection. the ORE flag is not set and the new received data overwrites the previous content of the USART_RDR register. When FIFO mode is enabled, the RXFIFO is bypassed and data is written directly in USART_RDR register. Even when FIFO management is enabled, the RXNE flag is to be used. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: This control bit enables checking the communication flow w/o reading the data"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error This bit can only be written when the USART is disabled (UE=0). Note: The reception errors are: parity error, framing error or noise error."]
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    #[doc = "Bit 14 - Driver enable mode This bit enables the user to activate the external transceiver control, through the DE signal. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. ."]
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    #[doc = "Bit 15 - Driver enable polarity selection This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If the Driver Enable feature is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count This bitfield specifies the number of retries for transmission and reception in Smartcard mode. In transmission mode, it specifies the number of automatic retransmission retries, before generating a transmission error (FE bit set). In reception mode, it specifies the number or erroneous reception trials, before generating a reception error (RXNE/RXFNE and PE bits set). This bitfield must be programmed only when the USART is disabled (UEÂ =Â 0). When the USART is enabled (UEÂ =Â 1), this bitfield may only be written to 0x0, in order to stop retransmission. Note: If Smartcard mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn scarcnt(&mut self) -> SCARCNT_W<17> {
        SCARCNT_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup from low-power mode interrupt flag selection This bitfield specifies the event which activates the WUF (Wakeup from low-power mode flag). This bitfield can only be written when the USART is disabled (UEÂ =Â 0). If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup from low-power mode interrupt enable This bit is set and cleared by software. Note: WUFIE must be set before entering in low-power mode. If the USART does not support the wakeup from Stop feature, this bit is reserved and must be kept at reset value. Refer to pageÂ 835."]
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    #[doc = "Bit 23 - TXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn txftie(&mut self) -> TXFTIE_W<23> {
        TXFTIE_W::new(self)
    }
    #[doc = "Bit 24 - Transmission Complete before guard time, interrupt enable This bit is set and cleared by software. Note: If the USART does not support the Smartcard mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<24> {
        TCBGTIE_W::new(self)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration Remaining combinations: Reserved"]
    #[inline(always)]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<25> {
        RXFTCFG_W::new(self)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxftie(&mut self) -> RXFTIE_W<28> {
        RXFTIE_W::new(self)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration Remaining combinations: Reserved"]
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
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
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
