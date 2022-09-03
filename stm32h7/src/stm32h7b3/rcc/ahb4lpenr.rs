#[doc = "Register `AHB4LPENR` reader"]
pub struct R(crate::R<AHB4LPENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB4LPENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB4LPENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB4LPENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB4LPENR` writer"]
pub struct W(crate::W<AHB4LPENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB4LPENR_SPEC>;
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
impl From<crate::W<AHB4LPENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB4LPENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOALPEN` reader - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOALPEN_R = crate::BitReader<GPIOALPEN_A>;
#[doc = "GPIOA peripheral clock enable during CSleep mode Set and reset by software.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GPIOALPEN_A {
    #[doc = "0: The selected clock is disabled during csleep mode"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled during csleep mode"]
    Enabled = 1,
}
impl From<GPIOALPEN_A> for bool {
    #[inline(always)]
    fn from(variant: GPIOALPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPIOALPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPIOALPEN_A {
        match self.bits {
            false => GPIOALPEN_A::Disabled,
            true => GPIOALPEN_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GPIOALPEN_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GPIOALPEN_A::Enabled
    }
}
#[doc = "Field `GPIOALPEN` writer - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
pub type GPIOALPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB4LPENR_SPEC, GPIOALPEN_A, O>;
impl<'a, const O: u8> GPIOALPEN_W<'a, O> {
    #[doc = "The selected clock is disabled during csleep mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::Disabled)
    }
    #[doc = "The selected clock is enabled during csleep mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GPIOALPEN_A::Enabled)
    }
}
#[doc = "Field `GPIOBLPEN` reader - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOBLPEN_R;
#[doc = "Field `GPIOCLPEN` reader - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOCLPEN_R;
#[doc = "Field `GPIODLPEN` reader - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIODLPEN_R;
#[doc = "Field `GPIOELPEN` reader - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOELPEN_R;
#[doc = "Field `GPIOFLPEN` reader - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOFLPEN_R;
#[doc = "Field `GPIOGLPEN` reader - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOGLPEN_R;
#[doc = "Field `GPIOHLPEN` reader - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOHLPEN_R;
#[doc = "Field `GPIOILPEN` reader - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOILPEN_R;
#[doc = "Field `GPIOJLPEN` reader - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOJLPEN_R;
#[doc = "Field `GPIOKLPEN` reader - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as GPIOKLPEN_R;
#[doc = "Field `BDMA2LPEN` reader - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as BDMA2LPEN_R;
#[doc = "Field `BKPRAMLPEN` reader - Backup RAM clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as BKPRAMLPEN_R;
#[doc = "Field `SRDSRAMLPEN` reader - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_R as SRDSRAMLPEN_R;
#[doc = "Field `GPIOBLPEN` writer - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOBLPEN_W;
#[doc = "Field `GPIOCLPEN` writer - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOCLPEN_W;
#[doc = "Field `GPIODLPEN` writer - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIODLPEN_W;
#[doc = "Field `GPIOELPEN` writer - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOELPEN_W;
#[doc = "Field `GPIOFLPEN` writer - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOFLPEN_W;
#[doc = "Field `GPIOGLPEN` writer - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOGLPEN_W;
#[doc = "Field `GPIOHLPEN` writer - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOHLPEN_W;
#[doc = "Field `GPIOILPEN` writer - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOILPEN_W;
#[doc = "Field `GPIOJLPEN` writer - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOJLPEN_W;
#[doc = "Field `GPIOKLPEN` writer - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as GPIOKLPEN_W;
#[doc = "Field `BDMA2LPEN` writer - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as BDMA2LPEN_W;
#[doc = "Field `BKPRAMLPEN` writer - Backup RAM clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as BKPRAMLPEN_W;
#[doc = "Field `SRDSRAMLPEN` writer - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
pub use GPIOALPEN_W as SRDSRAMLPEN_W;
impl R {
    #[doc = "Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bdma2lpen(&self) -> BDMA2LPEN_R {
        BDMA2LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BKPRAMLPEN_R {
        BKPRAMLPEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn srdsramlpen(&self) -> SRDSRAMLPEN_R {
        SRDSRAMLPEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOA peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<0> {
        GPIOALPEN_W::new(self)
    }
    #[doc = "Bit 1 - GPIOB peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<1> {
        GPIOBLPEN_W::new(self)
    }
    #[doc = "Bit 2 - GPIOC peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<2> {
        GPIOCLPEN_W::new(self)
    }
    #[doc = "Bit 3 - GPIOD peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<3> {
        GPIODLPEN_W::new(self)
    }
    #[doc = "Bit 4 - GPIOE peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<4> {
        GPIOELPEN_W::new(self)
    }
    #[doc = "Bit 5 - GPIOF peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<5> {
        GPIOFLPEN_W::new(self)
    }
    #[doc = "Bit 6 - GPIOG peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<6> {
        GPIOGLPEN_W::new(self)
    }
    #[doc = "Bit 7 - GPIOH peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<7> {
        GPIOHLPEN_W::new(self)
    }
    #[doc = "Bit 8 - GPIOI peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<8> {
        GPIOILPEN_W::new(self)
    }
    #[doc = "Bit 9 - GPIOJ peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<9> {
        GPIOJLPEN_W::new(self)
    }
    #[doc = "Bit 10 - GPIOK peripheral clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<10> {
        GPIOKLPEN_W::new(self)
    }
    #[doc = "Bit 21 - SmartRun domain DMA and DMAMUX clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bdma2lpen(&mut self) -> BDMA2LPEN_W<21> {
        BDMA2LPEN_W::new(self)
    }
    #[doc = "Bit 28 - Backup RAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BKPRAMLPEN_W<28> {
        BKPRAMLPEN_W::new(self)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM clock enable during CSleep mode Set and reset by software."]
    #[inline(always)]
    pub fn srdsramlpen(&mut self) -> SRDSRAMLPEN_W<29> {
        SRDSRAMLPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb4lpenr](index.html) module"]
pub struct AHB4LPENR_SPEC;
impl crate::RegisterSpec for AHB4LPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb4lpenr::R](R) reader structure"]
impl crate::Readable for AHB4LPENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb4lpenr::W](W) writer structure"]
impl crate::Writable for AHB4LPENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0x3020_07ff"]
impl crate::Resettable for AHB4LPENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3020_07ff
    }
}
