#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLVEN` reader - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SLVEN_R = crate::BitReader<bool>;
#[doc = "Field `SLVEN` writer - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type SLVEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `DIS_NSS` reader - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DIS_NSS_R = crate::BitReader<bool>;
#[doc = "Field `DIS_NSS` writer - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type DIS_NSS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type ADDM7_R = crate::BitReader<bool>;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type ADDM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `LBDL` reader - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDL_R = crate::BitReader<bool>;
#[doc = "Field `LBDL` writer - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `LBDIE` reader - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDIE_R = crate::BitReader<bool>;
#[doc = "Field `LBDIE` writer - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `LBCL` reader - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBCL_R = crate::BitReader<bool>;
#[doc = "Field `LBCL` writer - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LBCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CPHA` reader - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPHA_R = crate::BitReader<bool>;
#[doc = "Field `CPHA` writer - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CPOL` reader - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPOL_R = crate::BitReader<bool>;
#[doc = "Field `CPOL` writer - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `CLKEN` reader - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type CLKEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKEN` writer - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
pub type CLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `STOP` reader - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LINEN` reader - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LINEN_R = crate::BitReader<bool>;
#[doc = "Field `LINEN` writer - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
pub type LINEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type DATAINV_R = crate::BitReader<bool>;
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type DATAINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type MSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ABREN` reader - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABREN_R = crate::BitReader<bool>;
#[doc = "Field `ABREN` writer - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ABRMOD` reader - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UEÂ =Â 0). Note: If DATAINVÂ =Â 1 and/or MSBFIRSTÂ =Â 1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABRMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ABRMOD` writer - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UEÂ =Â 0). Note: If DATAINVÂ =Â 1 and/or MSBFIRSTÂ =Â 1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type ABRMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `RTOEN` reader - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTOEN_R = crate::BitReader<bool>;
#[doc = "Field `RTOEN` writer - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
pub type RTOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ADD` reader - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0)."]
pub type ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD` writer - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0)."]
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn slven(&self) -> SLVEN_R {
        SLVEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dis_nss(&self) -> DIS_NSS_R {
        DIS_NSS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdl(&self) -> LBDL_R {
        LBDL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdie(&self) -> LBDIE_R {
        LBDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbcl(&self) -> LBCL_R {
        LBCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    pub fn clken(&self) -> CLKEN_R {
        CLKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn linen(&self) -> LINEN_R {
        LINEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abren(&self) -> ABREN_R {
        ABREN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UEÂ =Â 0). Note: If DATAINVÂ =Â 1 and/or MSBFIRSTÂ =Â 1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abrmod(&self) -> ABRMOD_R {
        ABRMOD_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtoen(&self) -> RTOEN_R {
        RTOEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Synchronous Slave mode enable When the SLVEN bit is set, the synchronous slave mode is enabled. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn slven(&mut self) -> SLVEN_W<0> {
        SLVEN_W::new(self)
    }
    #[doc = "Bit 3 - When the DIS_NSS bit is set, the NSS pin input is ignored. Note: When SPI slave mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn dis_nss(&mut self) -> DIS_NSS_W<3> {
        DIS_NSS_W::new(self)
    }
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the USART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<4> {
        ADDM7_W::new(self)
    }
    #[doc = "Bit 5 - LIN break detection length This bit is for selection between 11 bit or 10 bit break detection. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdl(&mut self) -> LBDL_W<5> {
        LBDL_W::new(self)
    }
    #[doc = "Bit 6 - LIN break detection interrupt enable Break interrupt mask (break detection using break delimiter). Note: If LIN mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbdie(&mut self) -> LBDIE_W<6> {
        LBDIE_W::new(self)
    }
    #[doc = "Bit 8 - Last bit clock pulse This bit is used to select whether the clock pulse associated with the last data bit transmitted (MSB) has to be output on the SCLK pin in synchronous mode. The last bit is the 7th or 8th or 9th data bit transmitted depending on the 7 or 8 or 9 bit format selected by the M bit in the USART_CR1 register. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn lbcl(&mut self) -> LBCL_W<8> {
        LBCL_W::new(self)
    }
    #[doc = "Bit 9 - Clock phase This bit is used to select the phase of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPOL bit to produce the desired clock/data relationship (see and ) This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<9> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 10 - Clock polarity This bit enables the user to select the polarity of the clock output on the SCLK pin in synchronous mode. It works in conjunction with the CPHA bit to produce the desired clock/data relationship This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If synchronous mode is not supported, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<10> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 11 - Clock enable This bit enables the user to enable the SCLK pin. This bit can only be written when the USART is disabled (UEÂ =Â 0). Note: If neither synchronous mode nor Smartcard mode is supported, this bit is reserved and must be kept at reset value. Refer to . In Smartcard mode, in order to provide correctly the SCLK clock to the smartcard, the steps below must be respected: UE = 0 SCEN = 1 GTPR configuration CLKEN= 1 UE = 1"]
    #[inline(always)]
    pub fn clken(&mut self) -> CLKEN_W<11> {
        CLKEN_W::new(self)
    }
    #[doc = "Bits 12:13 - stop bits These bits are used for programming the stop bits. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<12> {
        STOP_W::new(self)
    }
    #[doc = "Bit 14 - LIN mode enable This bit is set and cleared by software. The LIN mode enables the capability to send LIN synchronous breaks (13 low bits) using the SBKRQ bit in the USART_CR1 register, and to detect LIN Sync breaks. This bitfield can only be written when the USART is disabled (UEÂ =Â 0). Note: If the USART does not support LIN mode, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn linen(&mut self) -> LINEN_W<14> {
        LINEN_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<16> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<17> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<18> {
        DATAINV_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<19> {
        MSBFIRST_W::new(self)
    }
    #[doc = "Bit 20 - Auto baud rate enable This bit is set and cleared by software. Note: If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abren(&mut self) -> ABREN_W<20> {
        ABREN_W::new(self)
    }
    #[doc = "Bits 21:22 - Auto baud rate mode These bits are set and cleared by software. This bitfield can only be written when ABREN = 0 or the USART is disabled (UEÂ =Â 0). Note: If DATAINVÂ =Â 1 and/or MSBFIRSTÂ =Â 1 the patterns must be the same on the line, for example 0xAA for MSBFIRST) If the USART does not support the auto baud rate feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn abrmod(&mut self) -> ABRMOD_W<21> {
        ABRMOD_W::new(self)
    }
    #[doc = "Bit 23 - Receiver timeout enable This bit is set and cleared by software. When this feature is enabled, the RTOF flag in the USART_ISR register is set if the RX line is idle (no reception) for the duration programmed in the RTOR (receiver timeout register). Note: If the USART does not support the Receiver timeout feature, this bit is reserved and must be kept at reset value. Refer to ."]
    #[inline(always)]
    pub fn rtoen(&mut self) -> RTOEN_W<23> {
        RTOEN_W::new(self)
    }
    #[doc = "Bits 24:31 - Address of the USART node ADD\\[7:4\\]: These bits give the address of the USART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or low-power mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0). ADD\\[3:0\\]: These bits give the address of the USART node or a character code to be recognized. They are used for wakeup with address mark detection, in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the USART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn add(&mut self) -> ADD_W<24> {
        ADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
