#[doc = "Register `C1_AHB4ENR` reader"]
pub struct R(crate::R<C1_AHB4ENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_AHB4ENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_AHB4ENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_AHB4ENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1_AHB4ENR` writer"]
pub struct W(crate::W<C1_AHB4ENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_AHB4ENR_SPEC>;
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
impl From<crate::W<C1_AHB4ENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_AHB4ENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOAEN` reader - 0GPIO peripheral clock enable"]
pub type GPIOAEN_R = crate::BitReader<GPIOAEN_A>;
#[doc = "0GPIO peripheral clock enable\n\nValue on reset: 0"]
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
#[doc = "Field `GPIOAEN` writer - 0GPIO peripheral clock enable"]
pub type GPIOAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C1_AHB4ENR_SPEC, GPIOAEN_A, O>;
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
#[doc = "Field `GPIOBEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOBEN_R;
#[doc = "Field `GPIOCEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOCEN_R;
#[doc = "Field `GPIODEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIODEN_R;
#[doc = "Field `GPIOEEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOEEN_R;
#[doc = "Field `GPIOFEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOFEN_R;
#[doc = "Field `GPIOGEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOGEN_R;
#[doc = "Field `GPIOHEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOHEN_R;
#[doc = "Field `GPIOIEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOIEN_R;
#[doc = "Field `GPIOJEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOJEN_R;
#[doc = "Field `GPIOKEN` reader - 0GPIO peripheral clock enable"]
pub use GPIOAEN_R as GPIOKEN_R;
#[doc = "Field `CRCEN` reader - CRC peripheral clock enable"]
pub use GPIOAEN_R as CRCEN_R;
#[doc = "Field `BDMAEN` reader - BDMA and DMAMUX2 Clock Enable"]
pub use GPIOAEN_R as BDMAEN_R;
#[doc = "Field `ADC3EN` reader - ADC3 Peripheral Clocks Enable"]
pub use GPIOAEN_R as ADC3EN_R;
#[doc = "Field `HSEMEN` reader - HSEM peripheral clock enable"]
pub use GPIOAEN_R as HSEMEN_R;
#[doc = "Field `BKPRAMEN` reader - Backup RAM Clock Enable"]
pub use GPIOAEN_R as BKPRAMEN_R;
#[doc = "Field `GPIOBEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOBEN_W;
#[doc = "Field `GPIOCEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOCEN_W;
#[doc = "Field `GPIODEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIODEN_W;
#[doc = "Field `GPIOEEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOEEN_W;
#[doc = "Field `GPIOFEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOFEN_W;
#[doc = "Field `GPIOGEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOGEN_W;
#[doc = "Field `GPIOHEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOHEN_W;
#[doc = "Field `GPIOIEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOIEN_W;
#[doc = "Field `GPIOJEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOJEN_W;
#[doc = "Field `GPIOKEN` writer - 0GPIO peripheral clock enable"]
pub use GPIOAEN_W as GPIOKEN_W;
#[doc = "Field `CRCEN` writer - CRC peripheral clock enable"]
pub use GPIOAEN_W as CRCEN_W;
#[doc = "Field `BDMAEN` writer - BDMA and DMAMUX2 Clock Enable"]
pub use GPIOAEN_W as BDMAEN_W;
#[doc = "Field `ADC3EN` writer - ADC3 Peripheral Clocks Enable"]
pub use GPIOAEN_W as ADC3EN_W;
#[doc = "Field `HSEMEN` writer - HSEM peripheral clock enable"]
pub use GPIOAEN_W as HSEMEN_W;
#[doc = "Field `BKPRAMEN` writer - Backup RAM Clock Enable"]
pub use GPIOAEN_W as BKPRAMEN_W;
impl R {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GPIOGEN_R {
        GPIOGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GPIOHEN_R {
        GPIOHEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GPIOIEN_R {
        GPIOIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GPIOJEN_R {
        GPIOJEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&self) -> GPIOKEN_R {
        GPIOKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&self) -> BDMAEN_R {
        BDMAEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> ADC3EN_R {
        ADC3EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HSEMEN_R {
        HSEMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BKPRAMEN_R {
        BKPRAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GPIOAEN_W<0> {
        GPIOAEN_W::new(self)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GPIOBEN_W<1> {
        GPIOBEN_W::new(self)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GPIOCEN_W<2> {
        GPIOCEN_W::new(self)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GPIODEN_W<3> {
        GPIODEN_W::new(self)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GPIOEEN_W<4> {
        GPIOEEN_W::new(self)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GPIOFEN_W<5> {
        GPIOFEN_W::new(self)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GPIOGEN_W<6> {
        GPIOGEN_W::new(self)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GPIOHEN_W<7> {
        GPIOHEN_W::new(self)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> GPIOIEN_W<8> {
        GPIOIEN_W::new(self)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GPIOJEN_W<9> {
        GPIOJEN_W::new(self)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&mut self) -> GPIOKEN_W<10> {
        GPIOKEN_W::new(self)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<19> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&mut self) -> BDMAEN_W<21> {
        BDMAEN_W::new(self)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> ADC3EN_W<24> {
        ADC3EN_W::new(self)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&mut self) -> HSEMEN_W<25> {
        HSEMEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BKPRAMEN_W<28> {
        BKPRAMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC AHB4 Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1_ahb4enr](index.html) module"]
pub struct C1_AHB4ENR_SPEC;
impl crate::RegisterSpec for C1_AHB4ENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1_ahb4enr::R](R) reader structure"]
impl crate::Readable for C1_AHB4ENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1_ahb4enr::W](W) writer structure"]
impl crate::Writable for C1_AHB4ENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1_AHB4ENR to value 0"]
impl crate::Resettable for C1_AHB4ENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
