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
#[doc = "Field `SYSCFGEN` reader - System configuration controller clock enable bit"]
pub type SYSCFGEN_R = crate::BitReader<SYSCFGEN_A>;
#[doc = "System configuration controller clock enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGEN_A {
    #[doc = "0: Clock disabled"]
    Disabled = 0,
    #[doc = "1: Clock enabled"]
    Enabled = 1,
}
impl From<SYSCFGEN_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSCFGEN_A {
        match self.bits {
            false => SYSCFGEN_A::Disabled,
            true => SYSCFGEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGEN_A::Enabled
    }
}
#[doc = "Field `SYSCFGEN` writer - System configuration controller clock enable bit"]
pub type SYSCFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, SYSCFGEN_A, O>;
impl<'a, const O: u8> SYSCFGEN_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Disabled)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SYSCFGEN_A::Enabled)
    }
}
#[doc = "Field `TIM21EN` reader - TIM21 timer clock enable bit"]
pub use SYSCFGEN_R as TIM21EN_R;
#[doc = "Field `TIM22EN` reader - TIM22 timer clock enable bit"]
pub use SYSCFGEN_R as TIM22EN_R;
#[doc = "Field `MIFIEN` reader - MiFaRe Firewall clock enable bit"]
pub use SYSCFGEN_R as MIFIEN_R;
#[doc = "Field `ADCEN` reader - ADC clock enable bit"]
pub use SYSCFGEN_R as ADCEN_R;
#[doc = "Field `SPI1EN` reader - SPI1 clock enable bit"]
pub use SYSCFGEN_R as SPI1EN_R;
#[doc = "Field `USART1EN` reader - USART1 clock enable bit"]
pub use SYSCFGEN_R as USART1EN_R;
#[doc = "Field `DBGEN` reader - DBG clock enable bit"]
pub use SYSCFGEN_R as DBGEN_R;
#[doc = "Field `TIM21EN` writer - TIM21 timer clock enable bit"]
pub use SYSCFGEN_W as TIM21EN_W;
#[doc = "Field `TIM22EN` writer - TIM22 timer clock enable bit"]
pub use SYSCFGEN_W as TIM22EN_W;
#[doc = "Field `MIFIEN` writer - MiFaRe Firewall clock enable bit"]
pub use SYSCFGEN_W as MIFIEN_W;
#[doc = "Field `ADCEN` writer - ADC clock enable bit"]
pub use SYSCFGEN_W as ADCEN_W;
#[doc = "Field `SPI1EN` writer - SPI1 clock enable bit"]
pub use SYSCFGEN_W as SPI1EN_W;
#[doc = "Field `USART1EN` writer - USART1 clock enable bit"]
pub use SYSCFGEN_W as USART1EN_W;
#[doc = "Field `DBGEN` writer - DBG clock enable bit"]
pub use SYSCFGEN_W as DBGEN_W;
impl R {
    #[doc = "Bit 0 - System configuration controller clock enable bit"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SYSCFGEN_R {
        SYSCFGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable bit"]
    #[inline(always)]
    pub fn tim21en(&self) -> TIM21EN_R {
        TIM21EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable bit"]
    #[inline(always)]
    pub fn tim22en(&self) -> TIM22EN_R {
        TIM22EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - MiFaRe Firewall clock enable bit"]
    #[inline(always)]
    pub fn mifien(&self) -> MIFIEN_R {
        MIFIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC clock enable bit"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable bit"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable bit"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 22 - DBG clock enable bit"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - System configuration controller clock enable bit"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SYSCFGEN_W<0> {
        SYSCFGEN_W::new(self)
    }
    #[doc = "Bit 2 - TIM21 timer clock enable bit"]
    #[inline(always)]
    pub fn tim21en(&mut self) -> TIM21EN_W<2> {
        TIM21EN_W::new(self)
    }
    #[doc = "Bit 5 - TIM22 timer clock enable bit"]
    #[inline(always)]
    pub fn tim22en(&mut self) -> TIM22EN_W<5> {
        TIM22EN_W::new(self)
    }
    #[doc = "Bit 7 - MiFaRe Firewall clock enable bit"]
    #[inline(always)]
    pub fn mifien(&mut self) -> MIFIEN_W<7> {
        MIFIEN_W::new(self)
    }
    #[doc = "Bit 9 - ADC clock enable bit"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<9> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 12 - SPI1 clock enable bit"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - USART1 clock enable bit"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 22 - DBG clock enable bit"]
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<22> {
        DBGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
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
