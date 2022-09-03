#[doc = "Register `RCC_MP_APB5LPENSETR` reader"]
pub struct R(crate::R<RCC_MP_APB5LPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_APB5LPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_APB5LPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_APB5LPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_APB5LPENSETR` writer"]
pub struct W(crate::W<RCC_MP_APB5LPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_APB5LPENSETR_SPEC>;
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
impl From<crate::W<RCC_MP_APB5LPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_APB5LPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI6LPEN` reader - SPI6LPEN"]
pub type SPI6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI6LPEN` writer - SPI6LPEN"]
pub type SPI6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `I2C4LPEN` reader - I2C4LPEN"]
pub type I2C4LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C4LPEN` writer - I2C4LPEN"]
pub type I2C4LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `I2C6LPEN` reader - I2C6LPEN"]
pub type I2C6LPEN_R = crate::BitReader<bool>;
#[doc = "Field `I2C6LPEN` writer - I2C6LPEN"]
pub type I2C6LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `USART1LPEN` reader - USART1LPEN"]
pub type USART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1LPEN` writer - USART1LPEN"]
pub type USART1LPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `RTCAPBLPEN` reader - RTCAPBLPEN"]
pub type RTCAPBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCAPBLPEN` writer - RTCAPBLPEN"]
pub type RTCAPBLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `TZC1LPEN` reader - TZC1LPEN"]
pub type TZC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TZC1LPEN` writer - TZC1LPEN"]
pub type TZC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `TZC2LPEN` reader - TZC2LPEN"]
pub type TZC2LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TZC2LPEN` writer - TZC2LPEN"]
pub type TZC2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `TZPCLPEN` reader - TZPCLPEN"]
pub type TZPCLPEN_R = crate::BitReader<bool>;
#[doc = "Field `TZPCLPEN` writer - TZPCLPEN"]
pub type TZPCLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `IWDG1APBLPEN` reader - IWDG1APBLPEN"]
pub type IWDG1APBLPEN_R = crate::BitReader<bool>;
#[doc = "Field `IWDG1APBLPEN` writer - IWDG1APBLPEN"]
pub type IWDG1APBLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `BSECLPEN` reader - BSECLPEN"]
pub type BSECLPEN_R = crate::BitReader<bool>;
#[doc = "Field `BSECLPEN` writer - BSECLPEN"]
pub type BSECLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `STGENLPEN` reader - STGENLPEN"]
pub type STGENLPEN_R = crate::BitReader<bool>;
#[doc = "Field `STGENLPEN` writer - STGENLPEN"]
pub type STGENLPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
#[doc = "Field `STGENSTPEN` reader - STGENSTPEN"]
pub type STGENSTPEN_R = crate::BitReader<bool>;
#[doc = "Field `STGENSTPEN` writer - STGENSTPEN"]
pub type STGENSTPEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, RCC_MP_APB5LPENSETR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    pub fn iwdg1apblpen(&self) -> IWDG1APBLPEN_R {
        IWDG1APBLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<0> {
        SPI6LPEN_W::new(self)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<2> {
        I2C4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W<3> {
        I2C6LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<8> {
        RTCAPBLPEN_W::new(self)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W<11> {
        TZC1LPEN_W::new(self)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W<12> {
        TZC2LPEN_W::new(self)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W<13> {
        TZPCLPEN_W::new(self)
    }
    #[doc = "Bit 15 - IWDG1APBLPEN"]
    #[inline(always)]
    pub fn iwdg1apblpen(&mut self) -> IWDG1APBLPEN_W<15> {
        IWDG1APBLPEN_W::new(self)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W<16> {
        BSECLPEN_W::new(self)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W<20> {
        STGENLPEN_W::new(self)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W<21> {
        STGENSTPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used by the MCU in order to clear the PERxLPEN bits If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_apb5lpensetr](index.html) module"]
pub struct RCC_MP_APB5LPENSETR_SPEC;
impl crate::RegisterSpec for RCC_MP_APB5LPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_apb5lpensetr::R](R) reader structure"]
impl crate::Readable for RCC_MP_APB5LPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_apb5lpensetr::W](W) writer structure"]
impl crate::Writable for RCC_MP_APB5LPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_APB5LPENSETR to value 0x0011_391d"]
impl crate::Resettable for RCC_MP_APB5LPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0011_391d
    }
}
