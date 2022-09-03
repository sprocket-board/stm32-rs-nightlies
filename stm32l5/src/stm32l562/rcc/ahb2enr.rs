#[doc = "Register `AHB2ENR` reader"]
pub struct R(crate::R<AHB2ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB2ENR` writer"]
pub struct W(crate::W<AHB2ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB2ENR_SPEC>;
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
impl From<crate::W<AHB2ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB2ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - IO port A clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
#[doc = "IO port A clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOAEN_A {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<GPIOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOAEN_A {
        match self.bits {
            false => GPIOAEN_A::Disabled,
            true => GPIOAEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOAEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOAEN_A::Enabled
    }
}
#[doc = "Field `GPIOAEN` writer - IO port A clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB2ENR_SPEC, GPIOAEN_A, O>;
impl<'a, const O: u8> GPIOAEN_W<'a, O> {
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOAEN_A::Enabled)
    }
}
#[doc = "Field `GPIOBEN` reader - IO port B clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - IO port C clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - IO port D clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - IO port E clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - IO port F clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - IO port G clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - IO port H clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `ADCEN` reader - ADC clock enable"]
pub use GPIOAEN_R as ADCEN_R;
#[doc = "Field `AESEN` reader - AES accelerator clock enable"]
pub use GPIOAEN_R as AESEN_R;
#[doc = "Field `HASHEN` reader - HASH clock enable"]
pub use GPIOAEN_R as HASHEN_R;
#[doc = "Field `RNGEN` reader - Random Number Generator clock enable"]
pub use GPIOAEN_R as RNGEN_R;
#[doc = "Field `PKAEN` reader - PKAEN"]
pub use GPIOAEN_R as PKAEN_R;
#[doc = "Field `OTFDEC1EN` reader - OTFDEC1EN"]
pub use GPIOAEN_R as OTFDEC1EN_R;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 clock enable"]
pub use GPIOAEN_R as SDMMC1EN_R;
#[doc = "Field `GPIOBEN` writer - IO port B clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - IO port C clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - IO port D clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - IO port E clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - IO port F clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - IO port G clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - IO port H clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `ADCEN` writer - ADC clock enable"]
pub use GPIOAEN_W as ADCEN_W;
#[doc = "Field `AESEN` writer - AES accelerator clock enable"]
pub use GPIOAEN_W as AESEN_W;
#[doc = "Field `HASHEN` writer - HASH clock enable"]
pub use GPIOAEN_W as HASHEN_W;
#[doc = "Field `RNGEN` writer - Random Number Generator clock enable"]
pub use GPIOAEN_W as RNGEN_W;
#[doc = "Field `PKAEN` writer - PKAEN"]
pub use GPIOAEN_W as PKAEN_W;
#[doc = "Field `OTFDEC1EN` writer - OTFDEC1EN"]
pub use GPIOAEN_W as OTFDEC1EN_W;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 clock enable"]
pub use GPIOAEN_W as SDMMC1EN_W;
impl R {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&self) -> AESEN_R {
        AESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HASHEN_R {
        HASHEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RNGEN_R {
        RNGEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PKAEN"]
    #[inline(always)]
    pub fn pkaen(&self) -> PKAEN_R {
        PKAEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - OTFDEC1EN"]
    #[inline(always)]
    pub fn otfdec1en(&self) -> OTFDEC1EN_R {
        OTFDEC1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> SDMMC1EN_R {
        SDMMC1EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO port A clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - IO port B clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - IO port C clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - IO port D clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - IO port E clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - IO port F clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - IO port G clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - IO port H clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 13 - ADC clock enable"]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W<13> {
        ADCEN_W::new(self)
    }
    #[doc = "Bit 16 - AES accelerator clock enable"]
    #[inline(always)]
    pub fn aesen(&mut self) -> AESEN_W<16> {
        AESEN_W::new(self)
    }
    #[doc = "Bit 17 - HASH clock enable"]
    #[inline(always)]
    pub fn hashen(&mut self) -> HASHEN_W<17> {
        HASHEN_W::new(self)
    }
    #[doc = "Bit 18 - Random Number Generator clock enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RNGEN_W<18> {
        RNGEN_W::new(self)
    }
    #[doc = "Bit 19 - PKAEN"]
    #[inline(always)]
    pub fn pkaen(&mut self) -> PKAEN_W<19> {
        PKAEN_W::new(self)
    }
    #[doc = "Bit 21 - OTFDEC1EN"]
    #[inline(always)]
    pub fn otfdec1en(&mut self) -> OTFDEC1EN_W<21> {
        OTFDEC1EN_W::new(self)
    }
    #[doc = "Bit 22 - SDMMC1 clock enable"]
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> SDMMC1EN_W<22> {
        SDMMC1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB2 peripheral clock enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2enr](index.html) module"]
pub struct AHB2ENR_SPEC;
impl crate::RegisterSpec for AHB2ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2enr::R](R) reader structure"]
impl crate::Readable for AHB2ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb2enr::W](W) writer structure"]
impl crate::Writable for AHB2ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for AHB2ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
