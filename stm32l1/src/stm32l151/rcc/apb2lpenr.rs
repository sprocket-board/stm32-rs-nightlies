#[doc = "Register `APB2LPENR` reader"]
pub struct R(crate::R<APB2LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2LPENR` writer"]
pub struct W(crate::W<APB2LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2LPENR_SPEC>;
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
impl From<crate::W<APB2LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGLPEN` reader - System configuration controller clock enable during Sleep mode"]
pub type SYSCFGLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SYSCFGLPEN` writer - System configuration controller clock enable during Sleep mode"]
pub type SYSCFGLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM9LPEN` reader - TIM9 timer clock enable during Sleep mode"]
pub type TIM9LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM9LPEN` writer - TIM9 timer clock enable during Sleep mode"]
pub type TIM9LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM10LPEN` reader - TIM10 timer clock enable during Sleep mode"]
pub type TIM10LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM10LPEN` writer - TIM10 timer clock enable during Sleep mode"]
pub type TIM10LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `TIM11LPEN` reader - TIM11 timer clock enable during Sleep mode"]
pub type TIM11LPEN_R = crate::BitReader<bool>;
#[doc = "Field `TIM11LPEN` writer - TIM11 timer clock enable during Sleep mode"]
pub type TIM11LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `ADC1LPEN` reader - ADC1 interface clock enable during Sleep mode"]
pub type ADC1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `ADC1LPEN` writer - ADC1 interface clock enable during Sleep mode"]
pub type ADC1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SDIOLPEN` reader - SDIO clock enable during Sleep mode"]
pub type SDIOLPEN_R = crate::BitReader<bool>;
#[doc = "Field `SDIOLPEN` writer - SDIO clock enable during Sleep mode"]
pub type SDIOLPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `SPI1LPEN` reader - SPI 1 clock enable during Sleep mode"]
pub type SPI1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1LPEN` writer - SPI 1 clock enable during Sleep mode"]
pub type SPI1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
#[doc = "Field `USART1LPEN` reader - USART1 clock enable during Sleep mode"]
pub type USART1LPEN_R = crate::BitReader<bool>;
#[doc = "Field `USART1LPEN` writer - USART1 clock enable during Sleep mode"]
pub type USART1LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2LPENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM9 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM10 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&self) -> TIM10LPEN_R {
        TIM10LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM11 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&self) -> TIM11LPEN_R {
        TIM11LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&self) -> ADC1LPEN_R {
        ADC1LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&self) -> SDIOLPEN_R {
        SDIOLPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller clock enable during Sleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<0> {
        SYSCFGLPEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM9 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<2> {
        TIM9LPEN_W::new(self)
    }
    #[doc = "Bit 3 - TIM10 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim10lpen(&mut self) -> TIM10LPEN_W<3> {
        TIM10LPEN_W::new(self)
    }
    #[doc = "Bit 4 - TIM11 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn tim11lpen(&mut self) -> TIM11LPEN_W<4> {
        TIM11LPEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC1 interface clock enable during Sleep mode"]
    #[inline(always)]
    pub fn adc1lpen(&mut self) -> ADC1LPEN_W<9> {
        ADC1LPEN_W::new(self)
    }
    #[doc = "Bit 11 - SDIO clock enable during Sleep mode"]
    #[inline(always)]
    pub fn sdiolpen(&mut self) -> SDIOLPEN_W<11> {
        SDIOLPEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<12> {
        SPI1LPEN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<14> {
        USART1LPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable in low power mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2lpenr](index.html) module"]
pub struct APB2LPENR_SPEC;
impl crate::RegisterSpec for APB2LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2lpenr::R](R) reader structure"]
impl crate::Readable for APB2LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2lpenr::W](W) writer structure"]
impl crate::Writable for APB2LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2LPENR to value 0"]
impl crate::Resettable for APB2LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
