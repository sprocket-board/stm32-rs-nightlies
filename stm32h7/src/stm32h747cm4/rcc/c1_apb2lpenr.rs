#[doc = "Register `C1_APB2LPENR` reader"]
pub struct R(crate::R<C1_APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_APB2LPENR` writer"]
pub struct W(crate::W<C1_APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_APB2LPENR_SPEC>;
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
impl From<crate::W<C1_APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode"]
pub type TIM1LPEN_R = crate::BitReader<TIM1LPEN_A>;
#[doc = "TIM1 peripheral clock enable during CSleep mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIM1LPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<TIM1LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: TIM1LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TIM1LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIM1LPEN_A {
        match self.bits {
            false => TIM1LPEN_A::Disabled,
            true => TIM1LPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM1LPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM1LPEN_A::Enabled
    }
}
#[doc = "Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode"]
pub type TIM1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_APB2LPENR_SPEC, TIM1LPEN_A, O>;
impl<'a, const O: u8> TIM1LPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIM1LPEN_A::Enabled)
    }
}
#[doc = "Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_R as TIM8LPEN_R;
#[doc = "Field `USART1LPEN` reader - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as USART1LPEN_R;
#[doc = "Field `USART6LPEN` reader - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as USART6LPEN_R;
#[doc = "Field `SPI1LPEN` reader - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SPI1LPEN_R;
#[doc = "Field `SPI4LPEN` reader - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SPI4LPEN_R;
#[doc = "Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_R as TIM15LPEN_R;
#[doc = "Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_R as TIM16LPEN_R;
#[doc = "Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_R as TIM17LPEN_R;
#[doc = "Field `SPI5LPEN` reader - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SPI5LPEN_R;
#[doc = "Field `SAI1LPEN` reader - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SAI1LPEN_R;
#[doc = "Field `SAI2LPEN` reader - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SAI2LPEN_R;
#[doc = "Field `SAI3LPEN` reader - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as SAI3LPEN_R;
#[doc = "Field `DFSDM1LPEN` reader - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_R as DFSDM1LPEN_R;
#[doc = "Field `HRTIMLPEN` reader - HRTIM peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_R as HRTIMLPEN_R;
#[doc = "Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_W as TIM8LPEN_W;
#[doc = "Field `USART1LPEN` writer - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as USART1LPEN_W;
#[doc = "Field `USART6LPEN` writer - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as USART6LPEN_W;
#[doc = "Field `SPI1LPEN` writer - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SPI1LPEN_W;
#[doc = "Field `SPI4LPEN` writer - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SPI4LPEN_W;
#[doc = "Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_W as TIM15LPEN_W;
#[doc = "Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_W as TIM16LPEN_W;
#[doc = "Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_W as TIM17LPEN_W;
#[doc = "Field `SPI5LPEN` writer - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SPI5LPEN_W;
#[doc = "Field `SAI1LPEN` writer - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SAI1LPEN_W;
#[doc = "Field `SAI2LPEN` writer - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SAI2LPEN_W;
#[doc = "Field `SAI3LPEN` writer - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as SAI3LPEN_W;
#[doc = "Field `DFSDM1LPEN` writer - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub use TIM1LPEN_W as DFSDM1LPEN_W;
#[doc = "Field `HRTIMLPEN` writer - HRTIM peripheral clock enable during CSleep mode"]
pub use TIM1LPEN_W as HRTIMLPEN_W;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> SAI3LPEN_R {
        SAI3LPEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> DFSDM1LPEN_R {
        DFSDM1LPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hrtimlpen(&self) -> HRTIMLPEN_R {
        HRTIMLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<0> {
        TIM1LPEN_W::new(self)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<1> {
        TIM8LPEN_W::new(self)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<4> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<5> {
        USART6LPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<13> {
        SPI4LPEN_W::new(self)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<16> {
        TIM15LPEN_W::new(self)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<17> {
        TIM16LPEN_W::new(self)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<18> {
        TIM17LPEN_W::new(self)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<20> {
        SPI5LPEN_W::new(self)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<22> {
        SAI1LPEN_W::new(self)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<23> {
        SAI2LPEN_W::new(self)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai3lpen(&mut self) -> SAI3LPEN_W<24> {
        SAI3LPEN_W::new(self)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dfsdm1lpen(&mut self) -> DFSDM1LPEN_W<28> {
        DFSDM1LPEN_W::new(self)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hrtimlpen(&mut self) -> HRTIMLPEN_W<29> {
        HRTIMLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB2 Sleep Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_apb2lpenr](index.html) module"]
pub struct C1_APB2LPENR_SPEC;
impl crate::RegisterSpec for C1_APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_apb2lpenr::R](R) reader structure"]
impl crate::Readable for C1_APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_apb2lpenr::W](W) writer structure"]
impl crate::Writable for C1_APB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_APB2LPENR to value 0"]
impl crate::Resettable for C1_APB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
