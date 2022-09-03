#[doc = "Register `RXCRCR` reader"]
pub struct R(crate::R<RXCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCRC` reader - Rx CRC register When CRC calculation is enabled, the RXCRC\\[15:0\\]
bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used in I2S mode."]
pub type RXCRC_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Rx CRC register When CRC calculation is enabled, the RXCRC\\[15:0\\]
bits contain the computed CRC value of the subsequently received bytes. This register is reset when the CRCEN bit in SPI_CR1 register is written to 1. The CRC is calculated serially using the polynomial programmed in the SPI_CRCPR register. Only the 8 LSB bits are considered when the CRC frame format is set to be 8-bit length (CRCL bit in the SPI_CR1 is cleared). CRC calculation is done based on any CRC8 standard. The entire 16-bits of this register are considered when a 16-bit CRC frame format is selected (CRCL bit in the SPI_CR1 register is set). CRC calculation is done based on any CRC16 standard. Note: A read to this register when the BSY Flag is set could return an incorrect value. These bits are not used in I2S mode."]
    #[inline(always)]
    pub fn rxcrc(&self) -> RXCRC_R {
        RXCRC_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "SPI Rx CRC register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxcrcr](index.html) module"]
pub struct RXCRCR_SPEC;
impl crate::RegisterSpec for RXCRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxcrcr::R](R) reader structure"]
impl crate::Readable for RXCRCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXCRCR to value 0"]
impl crate::Resettable for RXCRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
