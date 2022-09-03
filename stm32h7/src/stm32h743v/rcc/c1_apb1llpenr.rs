#[doc = "Register `C1_APB1LLPENR` reader"]
pub struct R(crate::R<C1_APB1LLPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB1LLPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB1LLPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB1LLPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB1LLPENR` writer"]
pub struct W(crate::W<C1_APB1LLPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB1LLPENR_SPEC>;
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
impl From<crate::W<C1_APB1LLPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB1LLPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2LPEN` reader - TIM2 peripheral clock enable during CSleep mode"]
pub type TIM2LPEN_R = crate::BitReader<TIM2LPEN_A>;
#[doc = "TIM2 peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM2LPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<TIM2LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM2LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM2LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM2LPEN_A {
        match self.bits {
            false => TIM2LPEN_A::Disabled,
            true => TIM2LPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2LPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2LPEN_A::Enabled
    }
}
#[doc = "Field `TIM2LPEN` writer - TIM2 peripheral clock enable during CSleep mode"]
pub type TIM2LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_APB1LLPENR_SPEC, TIM2LPEN_A, O>;
impl<'a, const O: u8> TIM2LPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM2LPEN_A::Enabled)
    }
}
#[doc = "Field `TIM3LPEN` reader - TIM3 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM3LPEN_R;
#[doc = "Field `TIM4LPEN` reader - TIM4 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM4LPEN_R;
#[doc = "Field `TIM5LPEN` reader - TIM5 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM5LPEN_R;
#[doc = "Field `TIM6LPEN` reader - TIM6 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM6LPEN_R;
#[doc = "Field `TIM7LPEN` reader - TIM7 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM7LPEN_R;
#[doc = "Field `TIM12LPEN` reader - TIM12 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM12LPEN_R;
#[doc = "Field `TIM13LPEN` reader - TIM13 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM13LPEN_R;
#[doc = "Field `TIM14LPEN` reader - TIM14 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as TIM14LPEN_R;
#[doc = "Field `LPTIM1LPEN` reader - LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as LPTIM1LPEN_R;
#[doc = "Field `SPI2LPEN` reader - SPI2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as SPI2LPEN_R;
#[doc = "Field `SPI3LPEN` reader - SPI3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as SPI3LPEN_R;
#[doc = "Field `SPDIFRXLPEN` reader - SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as SPDIFRXLPEN_R;
#[doc = "Field `USART2LPEN` reader - USART2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as USART2LPEN_R;
#[doc = "Field `USART3LPEN` reader - USART3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as USART3LPEN_R;
#[doc = "Field `UART4LPEN` reader - UART4 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as UART4LPEN_R;
#[doc = "Field `UART5LPEN` reader - UART5 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as UART5LPEN_R;
#[doc = "Field `I2C1LPEN` reader - I2C1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as I2C1LPEN_R;
#[doc = "Field `I2C2LPEN` reader - I2C2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as I2C2LPEN_R;
#[doc = "Field `I2C3LPEN` reader - I2C3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as I2C3LPEN_R;
#[doc = "Field `CECLPEN` reader - HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as CECLPEN_R;
#[doc = "Field `DAC12LPEN` reader - DAC1/2 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_R as DAC12LPEN_R;
#[doc = "Field `UART7LPEN` reader - UART7 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as UART7LPEN_R;
#[doc = "Field `UART8LPEN` reader - UART8 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_R as UART8LPEN_R;
#[doc = "Field `TIM3LPEN` writer - TIM3 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM3LPEN_W;
#[doc = "Field `TIM4LPEN` writer - TIM4 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM4LPEN_W;
#[doc = "Field `TIM5LPEN` writer - TIM5 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM5LPEN_W;
#[doc = "Field `TIM6LPEN` writer - TIM6 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM6LPEN_W;
#[doc = "Field `TIM7LPEN` writer - TIM7 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM7LPEN_W;
#[doc = "Field `TIM12LPEN` writer - TIM12 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM12LPEN_W;
#[doc = "Field `TIM13LPEN` writer - TIM13 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM13LPEN_W;
#[doc = "Field `TIM14LPEN` writer - TIM14 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as TIM14LPEN_W;
#[doc = "Field `LPTIM1LPEN` writer - LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as LPTIM1LPEN_W;
#[doc = "Field `SPI2LPEN` writer - SPI2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as SPI2LPEN_W;
#[doc = "Field `SPI3LPEN` writer - SPI3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as SPI3LPEN_W;
#[doc = "Field `SPDIFRXLPEN` writer - SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as SPDIFRXLPEN_W;
#[doc = "Field `USART2LPEN` writer - USART2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as USART2LPEN_W;
#[doc = "Field `USART3LPEN` writer - USART3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as USART3LPEN_W;
#[doc = "Field `UART4LPEN` writer - UART4 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as UART4LPEN_W;
#[doc = "Field `UART5LPEN` writer - UART5 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as UART5LPEN_W;
#[doc = "Field `I2C1LPEN` writer - I2C1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as I2C1LPEN_W;
#[doc = "Field `I2C2LPEN` writer - I2C2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as I2C2LPEN_W;
#[doc = "Field `I2C3LPEN` writer - I2C3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as I2C3LPEN_W;
#[doc = "Field `CECLPEN` writer - HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as CECLPEN_W;
#[doc = "Field `DAC12LPEN` writer - DAC1/2 peripheral clock enable during CSleep mode"]
pub use TIM2LPEN_W as DAC12LPEN_W;
#[doc = "Field `UART7LPEN` writer - UART7 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as UART7LPEN_W;
#[doc = "Field `UART8LPEN` writer - UART8 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM2LPEN_W as UART8LPEN_W;
impl R {
    #[doc = "Bit 0 - TIM2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&self) -> TIM2LPEN_R {
        TIM2LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&self) -> TIM3LPEN_R {
        TIM3LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&self) -> TIM4LPEN_R {
        TIM4LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&self) -> TIM5LPEN_R {
        TIM5LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&self) -> TIM6LPEN_R {
        TIM6LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&self) -> TIM7LPEN_R {
        TIM7LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&self) -> TIM12LPEN_R {
        TIM12LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&self) -> TIM13LPEN_R {
        TIM13LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&self) -> TIM14LPEN_R {
        TIM14LPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim1lpen(&self) -> LPTIM1LPEN_R {
        LPTIM1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi2lpen(&self) -> SPI2LPEN_R {
        SPI2LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi3lpen(&self) -> SPI3LPEN_R {
        SPI3LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spdifrxlpen(&self) -> SPDIFRXLPEN_R {
        SPDIFRXLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart2lpen(&self) -> USART2LPEN_R {
        USART2LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart3lpen(&self) -> USART3LPEN_R {
        USART3LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart4lpen(&self) -> UART4LPEN_R {
        UART4LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart5lpen(&self) -> UART5LPEN_R {
        UART5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c1lpen(&self) -> I2C1LPEN_R {
        I2C1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c2lpen(&self) -> I2C2LPEN_R {
        I2C2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c3lpen(&self) -> I2C3LPEN_R {
        I2C3LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn ceclpen(&self) -> CECLPEN_R {
        CECLPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn dac12lpen(&self) -> DAC12LPEN_R {
        DAC12LPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - UART7 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart7lpen(&self) -> UART7LPEN_R {
        UART7LPEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - UART8 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart8lpen(&self) -> UART8LPEN_R {
        UART8LPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim2lpen(&mut self) -> TIM2LPEN_W<0> {
        TIM2LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM3 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim3lpen(&mut self) -> TIM3LPEN_W<1> {
        TIM3LPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM4 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim4lpen(&mut self) -> TIM4LPEN_W<2> {
        TIM4LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM5 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim5lpen(&mut self) -> TIM5LPEN_W<3> {
        TIM5LPEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM6 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim6lpen(&mut self) -> TIM6LPEN_W<4> {
        TIM6LPEN_W::new(self)
    }
    #[doc = "Bit 5 - TIM7 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim7lpen(&mut self) -> TIM7LPEN_W<5> {
        TIM7LPEN_W::new(self)
    }
    #[doc = "Bit 6 - TIM12 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim12lpen(&mut self) -> TIM12LPEN_W<6> {
        TIM12LPEN_W::new(self)
    }
    #[doc = "Bit 7 - TIM13 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim13lpen(&mut self) -> TIM13LPEN_W<7> {
        TIM13LPEN_W::new(self)
    }
    #[doc = "Bit 8 - TIM14 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim14lpen(&mut self) -> TIM14LPEN_W<8> {
        TIM14LPEN_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim1lpen(&mut self) -> LPTIM1LPEN_W<9> {
        LPTIM1LPEN_W::new(self)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi2lpen(&mut self) -> SPI2LPEN_W<14> {
        SPI2LPEN_W::new(self)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi3lpen(&mut self) -> SPI3LPEN_W<15> {
        SPI3LPEN_W::new(self)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spdifrxlpen(&mut self) -> SPDIFRXLPEN_W<16> {
        SPDIFRXLPEN_W::new(self)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart2lpen(&mut self) -> USART2LPEN_W<17> {
        USART2LPEN_W::new(self)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart3lpen(&mut self) -> USART3LPEN_W<18> {
        USART3LPEN_W::new(self)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart4lpen(&mut self) -> UART4LPEN_W<19> {
        UART4LPEN_W::new(self)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart5lpen(&mut self) -> UART5LPEN_W<20> {
        UART5LPEN_W::new(self)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c1lpen(&mut self) -> I2C1LPEN_W<21> {
        I2C1LPEN_W::new(self)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c2lpen(&mut self) -> I2C2LPEN_W<22> {
        I2C2LPEN_W::new(self)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c3lpen(&mut self) -> I2C3LPEN_W<23> {
        I2C3LPEN_W::new(self)
    }
    #[doc = "Bit 27 - HDMI-CEC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn ceclpen(&mut self) -> CECLPEN_W<27> {
        CECLPEN_W::new(self)
    }
    #[doc = "Bit 29 - DAC1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn dac12lpen(&mut self) -> DAC12LPEN_W<29> {
        DAC12LPEN_W::new(self)
    }
    #[doc = "Bit 30 - UART7 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart7lpen(&mut self) -> UART7LPEN_W<30> {
        UART7LPEN_W::new(self)
    }
    #[doc = "Bit 31 - UART8 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn uart8lpen(&mut self) -> UART8LPEN_W<31> {
        UART8LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB1 Low Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb1llpenr](index.html) module"]
pub struct C1_APB1LLPENR_SPEC;
impl crate::RegisterSpec for C1_APB1LLPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb1llpenr::R](R) reader structure"]
impl crate::Readable for C1_APB1LLPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb1llpenr::W](W) writer structure"]
impl crate::Writable for C1_APB1LLPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB1LLPENR to value 0"]
impl crate::Resettable for C1_APB1LLPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
