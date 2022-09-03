#[doc = "Register `APB2ENR` reader"]
pub struct R(crate::R<APB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB2ENR` writer"]
pub struct W(crate::W<APB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB2ENR_SPEC>;
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
impl From<crate::W<APB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AFIOEN` reader - Alternate function I/O clock enable"]
pub type AFIOEN_R = crate::BitReader<AFIOEN_A>;
#[doc = "Alternate function I/O clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AFIOEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<AFIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: AFIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AFIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AFIOEN_A {
        match self.bits {
            false => AFIOEN_A::Disabled,
            true => AFIOEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFIOEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFIOEN_A::Enabled
    }
}
#[doc = "Field `AFIOEN` writer - Alternate function I/O clock enable"]
pub type AFIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, AFIOEN_A, O>;
impl<'a, const O: u8> AFIOEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFIOEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFIOEN_A::Enabled)
    }
}
#[doc = "Field `IOPAEN` reader - I/O port A clock enable"]
pub use AFIOEN_R as IOPAEN_R;
#[doc = "Field `IOPBEN` reader - I/O port B clock enable"]
pub use AFIOEN_R as IOPBEN_R;
#[doc = "Field `IOPCEN` reader - I/O port C clock enable"]
pub use AFIOEN_R as IOPCEN_R;
#[doc = "Field `IOPDEN` reader - I/O port D clock enable"]
pub use AFIOEN_R as IOPDEN_R;
#[doc = "Field `IOPEEN` reader - I/O port E clock enable"]
pub use AFIOEN_R as IOPEEN_R;
#[doc = "Field `IOPFEN` reader - I/O port F clock enable"]
pub use AFIOEN_R as IOPFEN_R;
#[doc = "Field `IOPGEN` reader - I/O port G clock enable"]
pub use AFIOEN_R as IOPGEN_R;
#[doc = "Field `ADC1EN` reader - ADC 1 interface clock enable"]
pub use AFIOEN_R as ADC1EN_R;
#[doc = "Field `ADC2EN` reader - ADC 2 interface clock enable"]
pub use AFIOEN_R as ADC2EN_R;
#[doc = "Field `TIM1EN` reader - TIM1 Timer clock enable"]
pub use AFIOEN_R as TIM1EN_R;
#[doc = "Field `SPI1EN` reader - SPI 1 clock enable"]
pub use AFIOEN_R as SPI1EN_R;
#[doc = "Field `TIM8EN` reader - TIM8 Timer clock enable"]
pub use AFIOEN_R as TIM8EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable"]
pub use AFIOEN_R as USART1EN_R;
#[doc = "Field `ADC3EN` reader - ADC3 interface clock enable"]
pub use AFIOEN_R as ADC3EN_R;
#[doc = "Field `TIM9EN` reader - TIM9 Timer clock enable"]
pub use AFIOEN_R as TIM9EN_R;
#[doc = "Field `TIM10EN` reader - TIM10 Timer clock enable"]
pub use AFIOEN_R as TIM10EN_R;
#[doc = "Field `TIM11EN` reader - TIM11 Timer clock enable"]
pub use AFIOEN_R as TIM11EN_R;
#[doc = "Field `IOPAEN` writer - I/O port A clock enable"]
pub use AFIOEN_W as IOPAEN_W;
#[doc = "Field `IOPBEN` writer - I/O port B clock enable"]
pub use AFIOEN_W as IOPBEN_W;
#[doc = "Field `IOPCEN` writer - I/O port C clock enable"]
pub use AFIOEN_W as IOPCEN_W;
#[doc = "Field `IOPDEN` writer - I/O port D clock enable"]
pub use AFIOEN_W as IOPDEN_W;
#[doc = "Field `IOPEEN` writer - I/O port E clock enable"]
pub use AFIOEN_W as IOPEEN_W;
#[doc = "Field `IOPFEN` writer - I/O port F clock enable"]
pub use AFIOEN_W as IOPFEN_W;
#[doc = "Field `IOPGEN` writer - I/O port G clock enable"]
pub use AFIOEN_W as IOPGEN_W;
#[doc = "Field `ADC1EN` writer - ADC 1 interface clock enable"]
pub use AFIOEN_W as ADC1EN_W;
#[doc = "Field `ADC2EN` writer - ADC 2 interface clock enable"]
pub use AFIOEN_W as ADC2EN_W;
#[doc = "Field `TIM1EN` writer - TIM1 Timer clock enable"]
pub use AFIOEN_W as TIM1EN_W;
#[doc = "Field `SPI1EN` writer - SPI 1 clock enable"]
pub use AFIOEN_W as SPI1EN_W;
#[doc = "Field `TIM8EN` writer - TIM8 Timer clock enable"]
pub use AFIOEN_W as TIM8EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable"]
pub use AFIOEN_W as USART1EN_W;
#[doc = "Field `ADC3EN` writer - ADC3 interface clock enable"]
pub use AFIOEN_W as ADC3EN_W;
#[doc = "Field `TIM9EN` writer - TIM9 Timer clock enable"]
pub use AFIOEN_W as TIM9EN_W;
#[doc = "Field `TIM10EN` writer - TIM10 Timer clock enable"]
pub use AFIOEN_W as TIM10EN_W;
#[doc = "Field `TIM11EN` writer - TIM11 Timer clock enable"]
pub use AFIOEN_W as TIM11EN_W;
impl R {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&self) -> AFIOEN_R {
        AFIOEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&self) -> IOPAEN_R {
        IOPAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&self) -> IOPBEN_R {
        IOPBEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&self) -> IOPCEN_R {
        IOPCEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&self) -> IOPDEN_R {
        IOPDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&self) -> IOPEEN_R {
        IOPEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&self) -> IOPFEN_R {
        IOPFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&self) -> IOPGEN_R {
        IOPGEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adc1en(&self) -> ADC1EN_R {
        ADC1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&self) -> ADC2EN_R {
        ADC2EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> TIM8EN_R {
        TIM8EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC3 interface clock enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9en(&self) -> TIM9EN_R {
        TIM9EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10en(&self) -> TIM10EN_R {
        TIM10EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    pub fn tim11en(&self) -> TIM11EN_R {
        TIM11EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Alternate function I/O clock enable"]
    #[inline(always)]
    pub fn afioen(&mut self) -> AFIOEN_W<0> {
        AFIOEN_W::new(self)
    }
    #[doc = "Bit 2 - I/O port A clock enable"]
    #[inline(always)]
    pub fn iopaen(&mut self) -> IOPAEN_W<2> {
        IOPAEN_W::new(self)
    }
    #[doc = "Bit 3 - I/O port B clock enable"]
    #[inline(always)]
    pub fn iopben(&mut self) -> IOPBEN_W<3> {
        IOPBEN_W::new(self)
    }
    #[doc = "Bit 4 - I/O port C clock enable"]
    #[inline(always)]
    pub fn iopcen(&mut self) -> IOPCEN_W<4> {
        IOPCEN_W::new(self)
    }
    #[doc = "Bit 5 - I/O port D clock enable"]
    #[inline(always)]
    pub fn iopden(&mut self) -> IOPDEN_W<5> {
        IOPDEN_W::new(self)
    }
    #[doc = "Bit 6 - I/O port E clock enable"]
    #[inline(always)]
    pub fn iopeen(&mut self) -> IOPEEN_W<6> {
        IOPEEN_W::new(self)
    }
    #[doc = "Bit 7 - I/O port F clock enable"]
    #[inline(always)]
    pub fn iopfen(&mut self) -> IOPFEN_W<7> {
        IOPFEN_W::new(self)
    }
    #[doc = "Bit 8 - I/O port G clock enable"]
    #[inline(always)]
    pub fn iopgen(&mut self) -> IOPGEN_W<8> {
        IOPGEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC 1 interface clock enable"]
    #[inline(always)]
    pub fn adc1en(&mut self) -> ADC1EN_W<9> {
        ADC1EN_W::new(self)
    }
    #[doc = "Bit 10 - ADC 2 interface clock enable"]
    #[inline(always)]
    pub fn adc2en(&mut self) -> ADC2EN_W<10> {
        ADC2EN_W::new(self)
    }
    #[doc = "Bit 11 - TIM1 Timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - SPI 1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 13 - TIM8 Timer clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> TIM8EN_W<13> {
        TIM8EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 15 - ADC3 interface clock enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<15> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 19 - TIM9 Timer clock enable"]
    #[inline(always)]
    pub fn tim9en(&mut self) -> TIM9EN_W<19> {
        TIM9EN_W::new(self)
    }
    #[doc = "Bit 20 - TIM10 Timer clock enable"]
    #[inline(always)]
    pub fn tim10en(&mut self) -> TIM10EN_W<20> {
        TIM10EN_W::new(self)
    }
    #[doc = "Bit 21 - TIM11 Timer clock enable"]
    #[inline(always)]
    pub fn tim11en(&mut self) -> TIM11EN_W<21> {
        TIM11EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register (RCC_APB2ENR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
pub struct APB2ENR_SPEC;
impl crate::RegisterSpec for APB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb2enr::R](R) reader structure"]
impl crate::Readable for APB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb2enr::W](W) writer structure"]
impl crate::Writable for APB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB2ENR to value 0"]
impl crate::Resettable for APB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
