#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART1SW` reader - USART1 clock source selection"]
pub type USART1SW_R = crate::FieldReader<u8, USART1SW_A>;
#[doc = "USART1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART1SW_A {
    #[doc = "0: PCLK selected as USART clock source"]
    Pclk = 0,
    #[doc = "1: SYSCLK selected as USART clock source"]
    Sysclk = 1,
    #[doc = "2: LSE selected as USART clock source"]
    Lse = 2,
    #[doc = "3: HSI selected as USART clock source"]
    Hsi = 3,
}
impl From<USART1SW_A> for u8 {
    #[inline(always)]
    fn from(variant: USART1SW_A) -> Self {
        variant as _
    }
}
impl USART1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USART1SW_A {
        match self.bits {
            0 => USART1SW_A::Pclk,
            1 => USART1SW_A::Sysclk,
            2 => USART1SW_A::Lse,
            3 => USART1SW_A::Hsi,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Pclk`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == USART1SW_A::Pclk
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == USART1SW_A::Sysclk
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART1SW_A::Lse
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == USART1SW_A::Hsi
    }
}
#[doc = "Field `USART1SW` writer - USART1 clock source selection"]
pub type USART1SW_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFGR3_SPEC, u8, USART1SW_A, 2, O>;
impl<'a, const O: u8> USART1SW_W<'a, O> {
    #[doc = "PCLK selected as USART clock source"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Pclk)
    }
    #[doc = "SYSCLK selected as USART clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(USART1SW_A::Sysclk)
    }
    #[doc = "LSE selected as USART clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART1SW_A::Lse)
    }
    #[doc = "HSI selected as USART clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(USART1SW_A::Hsi)
    }
}
#[doc = "Field `I2C1SW` reader - I2C1 clock source selection"]
pub type I2C1SW_R = crate::BitReader<I2C1SW_A>;
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C1SW_A {
    #[doc = "0: HSI clock selected as I2C clock source"]
    Hsi = 0,
    #[doc = "1: SYSCLK clock selected as I2C clock source"]
    Sysclk = 1,
}
impl From<I2C1SW_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1SW_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1SW_A {
        match self.bits {
            false => I2C1SW_A::Hsi,
            true => I2C1SW_A::Sysclk,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == I2C1SW_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Sysclk`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2C1SW_A::Sysclk
    }
}
#[doc = "Field `I2C1SW` writer - I2C1 clock source selection"]
pub type I2C1SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, I2C1SW_A, O>;
impl<'a, const O: u8> I2C1SW_W<'a, O> {
    #[doc = "HSI clock selected as I2C clock source"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(I2C1SW_A::Hsi)
    }
    #[doc = "SYSCLK clock selected as I2C clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2C1SW_A::Sysclk)
    }
}
#[doc = "Field `CECSW` reader - HDMI CEC clock source selection"]
pub type CECSW_R = crate::BitReader<CECSW_A>;
#[doc = "HDMI CEC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CECSW_A {
    #[doc = "0: HSI clock divided by 244 selected as CEC clock source"]
    HsiDiv244 = 0,
    #[doc = "1: LSE clock selected as CEC clock source"]
    Lse = 1,
}
impl From<CECSW_A> for bool {
    #[inline(always)]
    fn from(variant: CECSW_A) -> Self {
        variant as u8 != 0
    }
}
impl CECSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CECSW_A {
        match self.bits {
            false => CECSW_A::HsiDiv244,
            true => CECSW_A::Lse,
        }
    }
    #[doc = "Checks if the value of the field is `HsiDiv244`"]
    #[inline(always)]
    pub fn is_hsi_div244(&self) -> bool {
        *self == CECSW_A::HsiDiv244
    }
    #[doc = "Checks if the value of the field is `Lse`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSW_A::Lse
    }
}
#[doc = "Field `CECSW` writer - HDMI CEC clock source selection"]
pub type CECSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, CECSW_A, O>;
impl<'a, const O: u8> CECSW_W<'a, O> {
    #[doc = "HSI clock divided by 244 selected as CEC clock source"]
    #[inline(always)]
    pub fn hsi_div244(self) -> &'a mut W {
        self.variant(CECSW_A::HsiDiv244)
    }
    #[doc = "LSE clock selected as CEC clock source"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSW_A::Lse)
    }
}
#[doc = "Field `USBSW` reader - USB clock source selection"]
pub type USBSW_R = crate::BitReader<USBSW_A>;
#[doc = "USB clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USBSW_A {
    #[doc = "0: HSI48 selected as USB clock source"]
    Hsi48 = 0,
    #[doc = "1: PLL clock selected as USB clock source"]
    Pllclk = 1,
}
impl From<USBSW_A> for bool {
    #[inline(always)]
    fn from(variant: USBSW_A) -> Self {
        variant as u8 != 0
    }
}
impl USBSW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSW_A {
        match self.bits {
            false => USBSW_A::Hsi48,
            true => USBSW_A::Pllclk,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSW_A::Hsi48
    }
    #[doc = "Checks if the value of the field is `Pllclk`"]
    #[inline(always)]
    pub fn is_pllclk(&self) -> bool {
        *self == USBSW_A::Pllclk
    }
}
#[doc = "Field `USBSW` writer - USB clock source selection"]
pub type USBSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, USBSW_A, O>;
impl<'a, const O: u8> USBSW_W<'a, O> {
    #[doc = "HSI48 selected as USB clock source"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(USBSW_A::Hsi48)
    }
    #[doc = "PLL clock selected as USB clock source"]
    #[inline(always)]
    pub fn pllclk(self) -> &'a mut W {
        self.variant(USBSW_A::Pllclk)
    }
}
#[doc = "Field `ADCSW` reader - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_R = crate::BitReader<bool>;
#[doc = "Field `ADCSW` writer - ADCSW is deprecated. See ADC field in CFGR2 register."]
pub type ADCSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR3_SPEC, bool, O>;
#[doc = "Field `USART2SW` reader - USART2 clock source selection"]
pub use USART1SW_R as USART2SW_R;
#[doc = "Field `USART3SW` reader - USART3 clock source"]
pub use USART1SW_R as USART3SW_R;
#[doc = "Field `USART2SW` writer - USART2 clock source selection"]
pub use USART1SW_W as USART2SW_W;
#[doc = "Field `USART3SW` writer - USART3 clock source"]
pub use USART1SW_W as USART3SW_W;
impl R {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&self) -> USART1SW_R {
        USART1SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&self) -> I2C1SW_R {
        I2C1SW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&self) -> CECSW_R {
        CECSW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&self) -> USBSW_R {
        USBSW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&self) -> ADCSW_R {
        ADCSW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&self) -> USART2SW_R {
        USART2SW_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&self) -> USART3SW_R {
        USART3SW_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sw(&mut self) -> USART1SW_W<0> {
        USART1SW_W::new(self)
    }
    #[doc = "Bit 4 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sw(&mut self) -> I2C1SW_W<4> {
        I2C1SW_W::new(self)
    }
    #[doc = "Bit 6 - HDMI CEC clock source selection"]
    #[inline(always)]
    pub fn cecsw(&mut self) -> CECSW_W<6> {
        CECSW_W::new(self)
    }
    #[doc = "Bit 7 - USB clock source selection"]
    #[inline(always)]
    pub fn usbsw(&mut self) -> USBSW_W<7> {
        USBSW_W::new(self)
    }
    #[doc = "Bit 8 - ADCSW is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcsw(&mut self) -> ADCSW_W<8> {
        ADCSW_W::new(self)
    }
    #[doc = "Bits 16:17 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sw(&mut self) -> USART2SW_W<16> {
        USART2SW_W::new(self)
    }
    #[doc = "Bits 18:19 - USART3 clock source"]
    #[inline(always)]
    pub fn usart3sw(&mut self) -> USART3SW_W<18> {
        USART3SW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
