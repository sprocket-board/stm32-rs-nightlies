#[doc = "Register `APB1ENR` reader"]
pub struct R(crate::R<APB1ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB1ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB1ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB1ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB1ENR` writer"]
pub struct W(crate::W<APB1ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB1ENR_SPEC>;
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
impl From<crate::W<APB1ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB1ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM5EN` reader - TIM5 clock enable"]
pub type TIM5EN_R = crate::BitReader<TIM5EN_A>;
#[doc = "TIM5 clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM5EN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<TIM5EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM5EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM5EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM5EN_A {
        match self.bits {
            false => TIM5EN_A::Disabled,
            true => TIM5EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM5EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM5EN_A::Enabled
    }
}
#[doc = "Field `TIM5EN` writer - TIM5 clock enable"]
pub type TIM5EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, TIM5EN_A, O>;
impl<'a, const O: u8> TIM5EN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM5EN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM5EN_A::Enabled)
    }
}
#[doc = "Field `TIM6EN` reader - TIM6 clock enable"]
pub use TIM5EN_R as TIM6EN_R;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 clock enable"]
pub use TIM5EN_R as LPTIM1EN_R;
#[doc = "Field `RTCAPBEN` reader - RTC APB clock enable"]
pub use TIM5EN_R as RTCAPBEN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable"]
pub use TIM5EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable"]
pub use TIM5EN_R as SPI2EN_R;
#[doc = "Field `USART2EN` reader - USART 2 clock enable"]
pub use TIM5EN_R as USART2EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable"]
pub use TIM5EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable"]
pub use TIM5EN_R as I2C2EN_R;
#[doc = "Field `FMPI2C1EN` reader - FMPI2C1 clock enable"]
pub use TIM5EN_R as FMPI2C1EN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable"]
pub use TIM5EN_R as PWREN_R;
#[doc = "Field `DACEN` reader - DAC interface clock enable"]
pub use TIM5EN_R as DACEN_R;
#[doc = "Field `TIM6EN` writer - TIM6 clock enable"]
pub use TIM5EN_W as TIM6EN_W;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 clock enable"]
pub use TIM5EN_W as LPTIM1EN_W;
#[doc = "Field `RTCAPBEN` writer - RTC APB clock enable"]
pub use TIM5EN_W as RTCAPBEN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable"]
pub use TIM5EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable"]
pub use TIM5EN_W as SPI2EN_W;
#[doc = "Field `USART2EN` writer - USART 2 clock enable"]
pub use TIM5EN_W as USART2EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable"]
pub use TIM5EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable"]
pub use TIM5EN_W as I2C2EN_W;
#[doc = "Field `FMPI2C1EN` writer - FMPI2C1 clock enable"]
pub use TIM5EN_W as FMPI2C1EN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable"]
pub use TIM5EN_W as PWREN_W;
#[doc = "Field `DACEN` writer - DAC interface clock enable"]
pub use TIM5EN_W as DACEN_W;
impl R {
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable"]
    #[inline(always)]
    pub fn fmpi2c1en(&self) -> FMPI2C1EN_R {
        FMPI2C1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - TIM5 clock enable"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<3> {
        TIM5EN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 clock enable"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1 clock enable"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<9> {
        LPTIM1EN_W::new(self)
    }
    #[doc = "Bit 10 - RTC APB clock enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<10> {
        RTCAPBEN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 17 - USART 2 clock enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 24 - FMPI2C1 clock enable"]
    #[inline(always)]
    pub fn fmpi2c1en(&mut self) -> FMPI2C1EN_W<24> {
        FMPI2C1EN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 29 - DAC interface clock enable"]
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<29> {
        DACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB1 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1enr](index.html) module"]
pub struct APB1ENR_SPEC;
impl crate::RegisterSpec for APB1ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb1enr::R](R) reader structure"]
impl crate::Readable for APB1ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb1enr::W](W) writer structure"]
impl crate::Writable for APB1ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB1ENR to value 0"]
impl crate::Resettable for APB1ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
