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
#[doc = "Field `TIM1EN` reader - CPU1 TIM1 timer clock enable"]
pub type TIM1EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM1EN` writer - CPU1 TIM1 timer clock enable"]
pub type TIM1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SPI1EN` reader - CPU1 SPI1 clock enable"]
pub type SPI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI1EN` writer - CPU1 SPI1 clock enable"]
pub type SPI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `USART1EN` reader - CPU1 USART1clock enable"]
pub type USART1EN_R = crate::BitReader<bool>;
#[doc = "Field `USART1EN` writer - CPU1 USART1clock enable"]
pub type USART1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM16EN` reader - CPU1 TIM16 timer clock enable"]
pub type TIM16EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM16EN` writer - CPU1 TIM16 timer clock enable"]
pub type TIM16EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `TIM17EN` reader - CPU1 TIM17 timer clock enable"]
pub type TIM17EN_R = crate::BitReader<bool>;
#[doc = "Field `TIM17EN` writer - CPU1 TIM17 timer clock enable"]
pub type TIM17EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
#[doc = "Field `SAI1EN` reader - CPU1 SAI1 clock enable"]
pub type SAI1EN_R = crate::BitReader<bool>;
#[doc = "Field `SAI1EN` writer - CPU1 SAI1 clock enable"]
pub type SAI1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, APB2ENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 11 - CPU1 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> TIM1EN_R {
        TIM1EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU1 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> SPI1EN_R {
        SPI1EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU1 USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU1 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> TIM16EN_R {
        TIM16EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU1 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> TIM17EN_R {
        TIM17EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU1 SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> SAI1EN_R {
        SAI1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - CPU1 TIM1 timer clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> TIM1EN_W<11> {
        TIM1EN_W::new(self)
    }
    #[doc = "Bit 12 - CPU1 SPI1 clock enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> SPI1EN_W<12> {
        SPI1EN_W::new(self)
    }
    #[doc = "Bit 14 - CPU1 USART1clock enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W<14> {
        USART1EN_W::new(self)
    }
    #[doc = "Bit 17 - CPU1 TIM16 timer clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> TIM16EN_W<17> {
        TIM16EN_W::new(self)
    }
    #[doc = "Bit 18 - CPU1 TIM17 timer clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> TIM17EN_W<18> {
        TIM17EN_W::new(self)
    }
    #[doc = "Bit 21 - CPU1 SAI1 clock enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> SAI1EN_W<21> {
        SAI1EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB2ENR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2enr](index.html) module"]
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
