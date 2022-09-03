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
#[doc = "Field `ADDM7` reader - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type ADDM7_R = crate::BitReader<bool>;
#[doc = "Field `ADDM7` writer - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
pub type ADDM7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `STOP` reader - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type STOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STOP` writer - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type STOP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 2, O>;
#[doc = "Field `SWAP` reader - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type SWAP_R = crate::BitReader<bool>;
#[doc = "Field `SWAP` writer - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type SWAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `RXINV` reader - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type RXINV_R = crate::BitReader<bool>;
#[doc = "Field `RXINV` writer - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `TXINV` reader - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type TXINV_R = crate::BitReader<bool>;
#[doc = "Field `TXINV` writer - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `DATAINV` reader - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DATAINV_R = crate::BitReader<bool>;
#[doc = "Field `DATAINV` writer - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type DATAINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `MSBFIRST` reader - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type MSBFIRST_R = crate::BitReader<bool>;
#[doc = "Field `MSBFIRST` writer - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
pub type MSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
#[doc = "Field `ADD` reader - Address of the LPUART node ADD\\[7:4\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or Stop mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0) ADD\\[3:0\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used for wakeup with address mark detection in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0)"]
pub type ADD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADD` writer - Address of the LPUART node ADD\\[7:4\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or Stop mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0) ADD\\[3:0\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used for wakeup with address mark detection in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0)"]
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&self) -> ADDM7_R {
        ADDM7_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn swap(&self) -> SWAP_R {
        SWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn datainv(&self) -> DATAINV_R {
        DATAINV_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn msbfirst(&self) -> MSBFIRST_R {
        MSBFIRST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node ADD\\[7:4\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or Stop mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0) ADD\\[3:0\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used for wakeup with address mark detection in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0)"]
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - 7-bit Address Detection/4-bit Address Detection This bit is for selection between 4-bit address detection or 7-bit address detection. This bit can only be written when the LPUART is disabled (UEÂ =Â 0) Note: In 7-bit and 9-bit data modes, the address detection is done on 6-bit and 8-bit address (ADD\\[5:0\\]
and ADD\\[7:0\\]) respectively."]
    #[inline(always)]
    pub fn addm7(&mut self) -> ADDM7_W<4> {
        ADDM7_W::new(self)
    }
    #[doc = "Bits 12:13 - STOP bits These bits are used for programming the stop bits. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<12> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - Swap TX/RX pins This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn swap(&mut self) -> SWAP_W<15> {
        SWAP_W::new(self)
    }
    #[doc = "Bit 16 - RX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the RX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn rxinv(&mut self) -> RXINV_W<16> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 17 - TX pin active level inversion This bit is set and cleared by software. This enables the use of an external inverter on the TX line. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn txinv(&mut self) -> TXINV_W<17> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 18 - Binary data inversion This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn datainv(&mut self) -> DATAINV_W<18> {
        DATAINV_W::new(self)
    }
    #[doc = "Bit 19 - Most significant bit first This bit is set and cleared by software. This bitfield can only be written when the LPUART is disabled (UEÂ =Â 0)."]
    #[inline(always)]
    pub fn msbfirst(&mut self) -> MSBFIRST_W<19> {
        MSBFIRST_W::new(self)
    }
    #[doc = "Bits 24:31 - Address of the LPUART node ADD\\[7:4\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used to wake up the MCU with 7-bit address mark detection in multiprocessor communication during Mute mode or Stop mode. The MSB of the character sent by the transmitter should be equal to 1. They can also be used for character detection during normal reception, Mute mode inactive (for example, end of block detection in ModBus protocol). In this case, the whole received character (8-bit) is compared to the ADD\\[7:0\\]
value and CMF flag is set on match. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0) ADD\\[3:0\\]: These bits give the address of the LPUART node or a character code to be recognized. They are used for wakeup with address mark detection in multiprocessor communication during Mute mode or low-power mode. These bits can only be written when reception is disabled (RE = 0) or the LPUART is disabled (UEÂ =Â 0)"]
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
#[doc = "LPUART control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
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
