#[doc = "Register `APB4RSTR` reader"]
pub struct R(crate::R<APB4RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB4RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB4RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB4RSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB4RSTR` writer"]
pub struct W(crate::W<APB4RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB4RSTR_SPEC>;
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
impl From<crate::W<APB4RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB4RSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSCFGRST` reader - SYSCFG block reset"]
pub type SYSCFGRST_R = crate::BitReader<SYSCFGRST_A>;
#[doc = "SYSCFG block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCFGRST_A {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<SYSCFGRST_A> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SYSCFGRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCFGRST_A> {
        match self.bits {
            true => Some(SYSCFGRST_A::Reset),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Reset`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SYSCFGRST_A::Reset
    }
}
#[doc = "Field `SYSCFGRST` writer - SYSCFG block reset"]
pub type SYSCFGRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB4RSTR_SPEC, SYSCFGRST_A, O>;
impl<'a, const O: u8> SYSCFGRST_W<'a, O> {
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(SYSCFGRST_A::Reset)
    }
}
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset"]
pub use SYSCFGRST_R as LPUART1RST_R;
#[doc = "Field `SPI6RST` reader - SPI6 block reset"]
pub use SYSCFGRST_R as SPI6RST_R;
#[doc = "Field `I2C4RST` reader - I2C4 block reset"]
pub use SYSCFGRST_R as I2C4RST_R;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset"]
pub use SYSCFGRST_R as LPTIM2RST_R;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset"]
pub use SYSCFGRST_R as LPTIM3RST_R;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 block reset"]
pub use SYSCFGRST_R as LPTIM4RST_R;
#[doc = "Field `LPTIM5RST` reader - LPTIM5 block reset"]
pub use SYSCFGRST_R as LPTIM5RST_R;
#[doc = "Field `COMP12RST` reader - COMP12 Blocks Reset"]
pub use SYSCFGRST_R as COMP12RST_R;
#[doc = "Field `VREFRST` reader - VREF block reset"]
pub use SYSCFGRST_R as VREFRST_R;
#[doc = "Field `SAI4RST` reader - SAI4 block reset"]
pub use SYSCFGRST_R as SAI4RST_R;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset"]
pub use SYSCFGRST_W as LPUART1RST_W;
#[doc = "Field `SPI6RST` writer - SPI6 block reset"]
pub use SYSCFGRST_W as SPI6RST_W;
#[doc = "Field `I2C4RST` writer - I2C4 block reset"]
pub use SYSCFGRST_W as I2C4RST_W;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset"]
pub use SYSCFGRST_W as LPTIM2RST_W;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset"]
pub use SYSCFGRST_W as LPTIM3RST_W;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 block reset"]
pub use SYSCFGRST_W as LPTIM4RST_W;
#[doc = "Field `LPTIM5RST` writer - LPTIM5 block reset"]
pub use SYSCFGRST_W as LPTIM5RST_W;
#[doc = "Field `COMP12RST` writer - COMP12 Blocks Reset"]
pub use SYSCFGRST_W as COMP12RST_W;
#[doc = "Field `VREFRST` writer - VREF block reset"]
pub use SYSCFGRST_W as VREFRST_W;
#[doc = "Field `SAI4RST` writer - SAI4 block reset"]
pub use SYSCFGRST_W as SAI4RST_W;
impl R {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> LPTIM4RST_R {
        LPTIM4RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> LPTIM5RST_R {
        LPTIM5RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    pub fn comp12rst(&self) -> COMP12RST_R {
        COMP12RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VREFRST_R {
        VREFRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    pub fn sai4rst(&self) -> SAI4RST_R {
        SAI4RST_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<1> {
        SYSCFGRST_W::new(self)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<3> {
        LPUART1RST_W::new(self)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W<5> {
        SPI6RST_W::new(self)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W<7> {
        I2C4RST_W::new(self)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<9> {
        LPTIM2RST_W::new(self)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<10> {
        LPTIM3RST_W::new(self)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> LPTIM4RST_W<11> {
        LPTIM4RST_W::new(self)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> LPTIM5RST_W<12> {
        LPTIM5RST_W::new(self)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    pub fn comp12rst(&mut self) -> COMP12RST_W<14> {
        COMP12RST_W::new(self)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VREFRST_W<15> {
        VREFRST_W::new(self)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    pub fn sai4rst(&mut self) -> SAI4RST_W<21> {
        SAI4RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC APB4 Peripheral Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4rstr](index.html) module"]
pub struct APB4RSTR_SPEC;
impl crate::RegisterSpec for APB4RSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb4rstr::R](R) reader structure"]
impl crate::Readable for APB4RSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb4rstr::W](W) writer structure"]
impl crate::Writable for APB4RSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets APB4RSTR to value 0"]
impl crate::Resettable for APB4RSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
