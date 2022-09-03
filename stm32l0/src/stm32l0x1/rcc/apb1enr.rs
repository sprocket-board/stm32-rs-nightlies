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
#[doc = "Field `TIM2EN` reader - Timer2 clock enable bit"]
pub type TIM2EN_R = crate::BitReader<TIM2EN_A>;
#[doc = "Timer2 clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2EN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<TIM2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2EN_A {
        match self.bits {
            false => TIM2EN_A::Disabled,
            true => TIM2EN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN_A::Enabled
    }
}
#[doc = "Field `TIM2EN` writer - Timer2 clock enable bit"]
pub type TIM2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB1ENR_SPEC, TIM2EN_A, O>;
impl<'a, const O: u8> TIM2EN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2EN_A::Enabled)
    }
}
#[doc = "Field `TIM3EN` reader - Timer 3 clock enbale bit"]
pub use TIM2EN_R as TIM3EN_R;
#[doc = "Field `TIM6EN` reader - Timer 6 clock enable bit"]
pub use TIM2EN_R as TIM6EN_R;
#[doc = "Field `TIM7EN` reader - Timer 7 clock enable bit"]
pub use TIM2EN_R as TIM7EN_R;
#[doc = "Field `WWDGEN` reader - Window watchdog clock enable bit"]
pub use TIM2EN_R as WWDGEN_R;
#[doc = "Field `SPI2EN` reader - SPI2 clock enable bit"]
pub use TIM2EN_R as SPI2EN_R;
#[doc = "Field `USART2EN` reader - UART2 clock enable bit"]
pub use TIM2EN_R as USART2EN_R;
#[doc = "Field `LPUART1EN` reader - LPUART1 clock enable bit"]
pub use TIM2EN_R as LPUART1EN_R;
#[doc = "Field `USART4EN` reader - USART4 clock enable bit"]
pub use TIM2EN_R as USART4EN_R;
#[doc = "Field `USART5EN` reader - USART5 clock enable bit"]
pub use TIM2EN_R as USART5EN_R;
#[doc = "Field `I2C1EN` reader - I2C1 clock enable bit"]
pub use TIM2EN_R as I2C1EN_R;
#[doc = "Field `I2C2EN` reader - I2C2 clock enable bit"]
pub use TIM2EN_R as I2C2EN_R;
#[doc = "Field `PWREN` reader - Power interface clock enable bit"]
pub use TIM2EN_R as PWREN_R;
#[doc = "Field `I2C3EN` reader - I2C3 clock enable bit"]
pub use TIM2EN_R as I2C3EN_R;
#[doc = "Field `LPTIM1EN` reader - Low power timer clock enable bit"]
pub use TIM2EN_R as LPTIM1EN_R;
#[doc = "Field `TIM3EN` writer - Timer 3 clock enbale bit"]
pub use TIM2EN_W as TIM3EN_W;
#[doc = "Field `TIM6EN` writer - Timer 6 clock enable bit"]
pub use TIM2EN_W as TIM6EN_W;
#[doc = "Field `TIM7EN` writer - Timer 7 clock enable bit"]
pub use TIM2EN_W as TIM7EN_W;
#[doc = "Field `WWDGEN` writer - Window watchdog clock enable bit"]
pub use TIM2EN_W as WWDGEN_W;
#[doc = "Field `SPI2EN` writer - SPI2 clock enable bit"]
pub use TIM2EN_W as SPI2EN_W;
#[doc = "Field `USART2EN` writer - UART2 clock enable bit"]
pub use TIM2EN_W as USART2EN_W;
#[doc = "Field `LPUART1EN` writer - LPUART1 clock enable bit"]
pub use TIM2EN_W as LPUART1EN_W;
#[doc = "Field `USART4EN` writer - USART4 clock enable bit"]
pub use TIM2EN_W as USART4EN_W;
#[doc = "Field `USART5EN` writer - USART5 clock enable bit"]
pub use TIM2EN_W as USART5EN_W;
#[doc = "Field `I2C1EN` writer - I2C1 clock enable bit"]
pub use TIM2EN_W as I2C1EN_W;
#[doc = "Field `I2C2EN` writer - I2C2 clock enable bit"]
pub use TIM2EN_W as I2C2EN_W;
#[doc = "Field `PWREN` writer - Power interface clock enable bit"]
pub use TIM2EN_W as PWREN_W;
#[doc = "Field `I2C3EN` writer - I2C3 clock enable bit"]
pub use TIM2EN_W as I2C3EN_W;
#[doc = "Field `LPTIM1EN` writer - Low power timer clock enable bit"]
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    #[doc = "Bit 0 - Timer2 clock enable bit"]
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer 3 clock enbale bit"]
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 6 clock enable bit"]
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer 7 clock enable bit"]
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - Window watchdog clock enable bit"]
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 clock enable bit"]
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - UART2 clock enable bit"]
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - LPUART1 clock enable bit"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - USART4 clock enable bit"]
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART5 clock enable bit"]
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 clock enable bit"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 clock enable bit"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - Power interface clock enable bit"]
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - I2C3 clock enable bit"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Low power timer clock enable bit"]
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer2 clock enable bit"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<0> {
        TIM2EN_W::new(self)
    }
    #[doc = "Bit 1 - Timer 3 clock enbale bit"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<1> {
        TIM3EN_W::new(self)
    }
    #[doc = "Bit 4 - Timer 6 clock enable bit"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<4> {
        TIM6EN_W::new(self)
    }
    #[doc = "Bit 5 - Timer 7 clock enable bit"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<5> {
        TIM7EN_W::new(self)
    }
    #[doc = "Bit 11 - Window watchdog clock enable bit"]
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<11> {
        WWDGEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 clock enable bit"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<14> {
        SPI2EN_W::new(self)
    }
    #[doc = "Bit 17 - UART2 clock enable bit"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<17> {
        USART2EN_W::new(self)
    }
    #[doc = "Bit 18 - LPUART1 clock enable bit"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<18> {
        LPUART1EN_W::new(self)
    }
    #[doc = "Bit 19 - USART4 clock enable bit"]
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<19> {
        USART4EN_W::new(self)
    }
    #[doc = "Bit 20 - USART5 clock enable bit"]
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W<20> {
        USART5EN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 clock enable bit"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<21> {
        I2C1EN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 clock enable bit"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<22> {
        I2C2EN_W::new(self)
    }
    #[doc = "Bit 28 - Power interface clock enable bit"]
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<28> {
        PWREN_W::new(self)
    }
    #[doc = "Bit 30 - I2C3 clock enable bit"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<30> {
        I2C3EN_W::new(self)
    }
    #[doc = "Bit 31 - Low power timer clock enable bit"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<31> {
        LPTIM1EN_W::new(self)
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
