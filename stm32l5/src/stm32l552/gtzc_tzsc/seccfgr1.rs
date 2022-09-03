#[doc = "Register `SECCFGR1` reader"]
pub struct R(crate::R<SECCFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR1` writer"]
pub struct W(crate::W<SECCFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR1_SPEC>;
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
impl From<crate::W<SECCFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIM2SEC` reader - TIM2SEC"]
pub type TIM2SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM2SEC` writer - TIM2SEC"]
pub type TIM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM3SEC` reader - TIM3SEC"]
pub type TIM3SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM3SEC` writer - TIM3SEC"]
pub type TIM3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM4SEC` reader - TIM4SEC"]
pub type TIM4SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM4SEC` writer - TIM4SEC"]
pub type TIM4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM5SEC` reader - TIM5SEC"]
pub type TIM5SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM5SEC` writer - TIM5SEC"]
pub type TIM5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM6SEC` reader - TIM6SEC"]
pub type TIM6SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM6SEC` writer - TIM6SEC"]
pub type TIM6SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM7SEC` reader - TIM7SEC"]
pub type TIM7SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM7SEC` writer - TIM7SEC"]
pub type TIM7SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `WWDGSEC` reader - WWDGSEC"]
pub type WWDGSEC_R = crate::BitReader<bool>;
#[doc = "Field `WWDGSEC` writer - WWDGSEC"]
pub type WWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `IWDGSEC` reader - IWDGSEC"]
pub type IWDGSEC_R = crate::BitReader<bool>;
#[doc = "Field `IWDGSEC` writer - IWDGSEC"]
pub type IWDGSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI2SEC` reader - SPI2SEC"]
pub type SPI2SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI2SEC` writer - SPI2SEC"]
pub type SPI2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI3SEC` reader - SPI3SEC"]
pub type SPI3SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI3SEC` writer - SPI3SEC"]
pub type SPI3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART2SEC` reader - USART2SEC"]
pub type USART2SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART2SEC` writer - USART2SEC"]
pub type USART2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USART3SEC` reader - USART3SEC"]
pub type USART3SEC_R = crate::BitReader<bool>;
#[doc = "Field `USART3SEC` writer - USART3SEC"]
pub type USART3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART4SEC` reader - UART4SEC"]
pub type UART4SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART4SEC` writer - UART4SEC"]
pub type UART4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UART5SEC` reader - UART5SEC"]
pub type UART5SEC_R = crate::BitReader<bool>;
#[doc = "Field `UART5SEC` writer - UART5SEC"]
pub type UART5SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C1SEC` reader - I2C1SEC"]
pub type I2C1SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C1SEC` writer - I2C1SEC"]
pub type I2C1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C2SEC` reader - I2C2SEC"]
pub type I2C2SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C2SEC` writer - I2C2SEC"]
pub type I2C2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C3SEC` reader - I2C3SEC"]
pub type I2C3SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C3SEC` writer - I2C3SEC"]
pub type I2C3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `CRSSEC` reader - CRSSEC"]
pub type CRSSEC_R = crate::BitReader<bool>;
#[doc = "Field `CRSSEC` writer - CRSSEC"]
pub type CRSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `DACSEC` reader - DACSEC"]
pub type DACSEC_R = crate::BitReader<bool>;
#[doc = "Field `DACSEC` writer - DACSEC"]
pub type DACSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `OPAMPSEC` reader - OPAMPSEC"]
pub type OPAMPSEC_R = crate::BitReader<bool>;
#[doc = "Field `OPAMPSEC` writer - OPAMPSEC"]
pub type OPAMPSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM1SEC` reader - LPTIM1SEC"]
pub type LPTIM1SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM1SEC` writer - LPTIM1SEC"]
pub type LPTIM1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `LPUART1SEC` reader - LPUART1SEC"]
pub type LPUART1SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPUART1SEC` writer - LPUART1SEC"]
pub type LPUART1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `I2C4SEC` reader - I2C4SEC"]
pub type I2C4SEC_R = crate::BitReader<bool>;
#[doc = "Field `I2C4SEC` writer - I2C4SEC"]
pub type I2C4SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM2SEC` reader - LPTIM2SEC"]
pub type LPTIM2SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM2SEC` writer - LPTIM2SEC"]
pub type LPTIM2SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `LPTIM3SEC` reader - LPTIM3SEC"]
pub type LPTIM3SEC_R = crate::BitReader<bool>;
#[doc = "Field `LPTIM3SEC` writer - LPTIM3SEC"]
pub type LPTIM3SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `FDCAN1SEC` reader - FDCAN1SEC"]
pub type FDCAN1SEC_R = crate::BitReader<bool>;
#[doc = "Field `FDCAN1SEC` writer - FDCAN1SEC"]
pub type FDCAN1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `USBFSSEC` reader - USBFSSEC"]
pub type USBFSSEC_R = crate::BitReader<bool>;
#[doc = "Field `USBFSSEC` writer - USBFSSEC"]
pub type USBFSSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `UCPD1SEC` reader - UCPD1SEC"]
pub type UCPD1SEC_R = crate::BitReader<bool>;
#[doc = "Field `UCPD1SEC` writer - UCPD1SEC"]
pub type UCPD1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `VREFBUFSEC` reader - VREFBUFSEC"]
pub type VREFBUFSEC_R = crate::BitReader<bool>;
#[doc = "Field `VREFBUFSEC` writer - VREFBUFSEC"]
pub type VREFBUFSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `COMPSEC` reader - COMPSEC"]
pub type COMPSEC_R = crate::BitReader<bool>;
#[doc = "Field `COMPSEC` writer - COMPSEC"]
pub type COMPSEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `TIM1SEC` reader - TIM1SEC"]
pub type TIM1SEC_R = crate::BitReader<bool>;
#[doc = "Field `TIM1SEC` writer - TIM1SEC"]
pub type TIM1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
#[doc = "Field `SPI1SEC` reader - SPI1SEC"]
pub type SPI1SEC_R = crate::BitReader<bool>;
#[doc = "Field `SPI1SEC` writer - SPI1SEC"]
pub type SPI1SEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SECCFGR1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TIM2SEC"]
    #[inline(always)]
    pub fn tim2sec(&self) -> TIM2SEC_R {
        TIM2SEC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3SEC"]
    #[inline(always)]
    pub fn tim3sec(&self) -> TIM3SEC_R {
        TIM3SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4SEC"]
    #[inline(always)]
    pub fn tim4sec(&self) -> TIM4SEC_R {
        TIM4SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5SEC"]
    #[inline(always)]
    pub fn tim5sec(&self) -> TIM5SEC_R {
        TIM5SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6SEC"]
    #[inline(always)]
    pub fn tim6sec(&self) -> TIM6SEC_R {
        TIM6SEC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM7SEC"]
    #[inline(always)]
    pub fn tim7sec(&self) -> TIM7SEC_R {
        TIM7SEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDGSEC"]
    #[inline(always)]
    pub fn wwdgsec(&self) -> WWDGSEC_R {
        WWDGSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IWDGSEC"]
    #[inline(always)]
    pub fn iwdgsec(&self) -> IWDGSEC_R {
        IWDGSEC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI2SEC"]
    #[inline(always)]
    pub fn spi2sec(&self) -> SPI2SEC_R {
        SPI2SEC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI3SEC"]
    #[inline(always)]
    pub fn spi3sec(&self) -> SPI3SEC_R {
        SPI3SEC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USART2SEC"]
    #[inline(always)]
    pub fn usart2sec(&self) -> USART2SEC_R {
        USART2SEC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USART3SEC"]
    #[inline(always)]
    pub fn usart3sec(&self) -> USART3SEC_R {
        USART3SEC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART4SEC"]
    #[inline(always)]
    pub fn uart4sec(&self) -> UART4SEC_R {
        UART4SEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UART5SEC"]
    #[inline(always)]
    pub fn uart5sec(&self) -> UART5SEC_R {
        UART5SEC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - I2C1SEC"]
    #[inline(always)]
    pub fn i2c1sec(&self) -> I2C1SEC_R {
        I2C1SEC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - I2C2SEC"]
    #[inline(always)]
    pub fn i2c2sec(&self) -> I2C2SEC_R {
        I2C2SEC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - I2C3SEC"]
    #[inline(always)]
    pub fn i2c3sec(&self) -> I2C3SEC_R {
        I2C3SEC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CRSSEC"]
    #[inline(always)]
    pub fn crssec(&self) -> CRSSEC_R {
        CRSSEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DACSEC"]
    #[inline(always)]
    pub fn dacsec(&self) -> DACSEC_R {
        DACSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPAMPSEC"]
    #[inline(always)]
    pub fn opampsec(&self) -> OPAMPSEC_R {
        OPAMPSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPTIM1SEC"]
    #[inline(always)]
    pub fn lptim1sec(&self) -> LPTIM1SEC_R {
        LPTIM1SEC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPUART1SEC"]
    #[inline(always)]
    pub fn lpuart1sec(&self) -> LPUART1SEC_R {
        LPUART1SEC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C4SEC"]
    #[inline(always)]
    pub fn i2c4sec(&self) -> I2C4SEC_R {
        I2C4SEC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LPTIM2SEC"]
    #[inline(always)]
    pub fn lptim2sec(&self) -> LPTIM2SEC_R {
        LPTIM2SEC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - LPTIM3SEC"]
    #[inline(always)]
    pub fn lptim3sec(&self) -> LPTIM3SEC_R {
        LPTIM3SEC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FDCAN1SEC"]
    #[inline(always)]
    pub fn fdcan1sec(&self) -> FDCAN1SEC_R {
        FDCAN1SEC_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USBFSSEC"]
    #[inline(always)]
    pub fn usbfssec(&self) -> USBFSSEC_R {
        USBFSSEC_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - UCPD1SEC"]
    #[inline(always)]
    pub fn ucpd1sec(&self) -> UCPD1SEC_R {
        UCPD1SEC_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - VREFBUFSEC"]
    #[inline(always)]
    pub fn vrefbufsec(&self) -> VREFBUFSEC_R {
        VREFBUFSEC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - COMPSEC"]
    #[inline(always)]
    pub fn compsec(&self) -> COMPSEC_R {
        COMPSEC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TIM1SEC"]
    #[inline(always)]
    pub fn tim1sec(&self) -> TIM1SEC_R {
        TIM1SEC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SPI1SEC"]
    #[inline(always)]
    pub fn spi1sec(&self) -> SPI1SEC_R {
        SPI1SEC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2SEC"]
    #[inline(always)]
    pub fn tim2sec(&mut self) -> TIM2SEC_W<0> {
        TIM2SEC_W::new(self)
    }
    #[doc = "Bit 1 - TIM3SEC"]
    #[inline(always)]
    pub fn tim3sec(&mut self) -> TIM3SEC_W<1> {
        TIM3SEC_W::new(self)
    }
    #[doc = "Bit 2 - TIM4SEC"]
    #[inline(always)]
    pub fn tim4sec(&mut self) -> TIM4SEC_W<2> {
        TIM4SEC_W::new(self)
    }
    #[doc = "Bit 3 - TIM5SEC"]
    #[inline(always)]
    pub fn tim5sec(&mut self) -> TIM5SEC_W<3> {
        TIM5SEC_W::new(self)
    }
    #[doc = "Bit 4 - TIM6SEC"]
    #[inline(always)]
    pub fn tim6sec(&mut self) -> TIM6SEC_W<4> {
        TIM6SEC_W::new(self)
    }
    #[doc = "Bit 5 - TIM7SEC"]
    #[inline(always)]
    pub fn tim7sec(&mut self) -> TIM7SEC_W<5> {
        TIM7SEC_W::new(self)
    }
    #[doc = "Bit 6 - WWDGSEC"]
    #[inline(always)]
    pub fn wwdgsec(&mut self) -> WWDGSEC_W<6> {
        WWDGSEC_W::new(self)
    }
    #[doc = "Bit 7 - IWDGSEC"]
    #[inline(always)]
    pub fn iwdgsec(&mut self) -> IWDGSEC_W<7> {
        IWDGSEC_W::new(self)
    }
    #[doc = "Bit 8 - SPI2SEC"]
    #[inline(always)]
    pub fn spi2sec(&mut self) -> SPI2SEC_W<8> {
        SPI2SEC_W::new(self)
    }
    #[doc = "Bit 9 - SPI3SEC"]
    #[inline(always)]
    pub fn spi3sec(&mut self) -> SPI3SEC_W<9> {
        SPI3SEC_W::new(self)
    }
    #[doc = "Bit 10 - USART2SEC"]
    #[inline(always)]
    pub fn usart2sec(&mut self) -> USART2SEC_W<10> {
        USART2SEC_W::new(self)
    }
    #[doc = "Bit 11 - USART3SEC"]
    #[inline(always)]
    pub fn usart3sec(&mut self) -> USART3SEC_W<11> {
        USART3SEC_W::new(self)
    }
    #[doc = "Bit 12 - UART4SEC"]
    #[inline(always)]
    pub fn uart4sec(&mut self) -> UART4SEC_W<12> {
        UART4SEC_W::new(self)
    }
    #[doc = "Bit 13 - UART5SEC"]
    #[inline(always)]
    pub fn uart5sec(&mut self) -> UART5SEC_W<13> {
        UART5SEC_W::new(self)
    }
    #[doc = "Bit 14 - I2C1SEC"]
    #[inline(always)]
    pub fn i2c1sec(&mut self) -> I2C1SEC_W<14> {
        I2C1SEC_W::new(self)
    }
    #[doc = "Bit 15 - I2C2SEC"]
    #[inline(always)]
    pub fn i2c2sec(&mut self) -> I2C2SEC_W<15> {
        I2C2SEC_W::new(self)
    }
    #[doc = "Bit 16 - I2C3SEC"]
    #[inline(always)]
    pub fn i2c3sec(&mut self) -> I2C3SEC_W<16> {
        I2C3SEC_W::new(self)
    }
    #[doc = "Bit 17 - CRSSEC"]
    #[inline(always)]
    pub fn crssec(&mut self) -> CRSSEC_W<17> {
        CRSSEC_W::new(self)
    }
    #[doc = "Bit 18 - DACSEC"]
    #[inline(always)]
    pub fn dacsec(&mut self) -> DACSEC_W<18> {
        DACSEC_W::new(self)
    }
    #[doc = "Bit 19 - OPAMPSEC"]
    #[inline(always)]
    pub fn opampsec(&mut self) -> OPAMPSEC_W<19> {
        OPAMPSEC_W::new(self)
    }
    #[doc = "Bit 20 - LPTIM1SEC"]
    #[inline(always)]
    pub fn lptim1sec(&mut self) -> LPTIM1SEC_W<20> {
        LPTIM1SEC_W::new(self)
    }
    #[doc = "Bit 21 - LPUART1SEC"]
    #[inline(always)]
    pub fn lpuart1sec(&mut self) -> LPUART1SEC_W<21> {
        LPUART1SEC_W::new(self)
    }
    #[doc = "Bit 22 - I2C4SEC"]
    #[inline(always)]
    pub fn i2c4sec(&mut self) -> I2C4SEC_W<22> {
        I2C4SEC_W::new(self)
    }
    #[doc = "Bit 23 - LPTIM2SEC"]
    #[inline(always)]
    pub fn lptim2sec(&mut self) -> LPTIM2SEC_W<23> {
        LPTIM2SEC_W::new(self)
    }
    #[doc = "Bit 24 - LPTIM3SEC"]
    #[inline(always)]
    pub fn lptim3sec(&mut self) -> LPTIM3SEC_W<24> {
        LPTIM3SEC_W::new(self)
    }
    #[doc = "Bit 25 - FDCAN1SEC"]
    #[inline(always)]
    pub fn fdcan1sec(&mut self) -> FDCAN1SEC_W<25> {
        FDCAN1SEC_W::new(self)
    }
    #[doc = "Bit 26 - USBFSSEC"]
    #[inline(always)]
    pub fn usbfssec(&mut self) -> USBFSSEC_W<26> {
        USBFSSEC_W::new(self)
    }
    #[doc = "Bit 27 - UCPD1SEC"]
    #[inline(always)]
    pub fn ucpd1sec(&mut self) -> UCPD1SEC_W<27> {
        UCPD1SEC_W::new(self)
    }
    #[doc = "Bit 28 - VREFBUFSEC"]
    #[inline(always)]
    pub fn vrefbufsec(&mut self) -> VREFBUFSEC_W<28> {
        VREFBUFSEC_W::new(self)
    }
    #[doc = "Bit 29 - COMPSEC"]
    #[inline(always)]
    pub fn compsec(&mut self) -> COMPSEC_W<29> {
        COMPSEC_W::new(self)
    }
    #[doc = "Bit 30 - TIM1SEC"]
    #[inline(always)]
    pub fn tim1sec(&mut self) -> TIM1SEC_W<30> {
        TIM1SEC_W::new(self)
    }
    #[doc = "Bit 31 - SPI1SEC"]
    #[inline(always)]
    pub fn spi1sec(&mut self) -> SPI1SEC_W<31> {
        SPI1SEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC secure configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr1](index.html) module"]
pub struct SECCFGR1_SPEC;
impl crate::RegisterSpec for SECCFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr1::R](R) reader structure"]
impl crate::Readable for SECCFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr1::W](W) writer structure"]
impl crate::Writable for SECCFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR1 to value 0"]
impl crate::Resettable for SECCFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
