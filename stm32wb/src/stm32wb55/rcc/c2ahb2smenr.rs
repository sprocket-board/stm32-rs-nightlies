#[doc = "Register `C2AHB2SMENR` reader"]
pub struct R(crate::R<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AHB2SMENR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AHB2SMENR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AHB2SMENR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2AHB2SMENR` writer"]
pub struct W(crate::W<C2AHB2SMENR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AHB2SMENR_SPEC>;
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
impl From<crate::W<C2AHB2SMENR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AHB2SMENR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPIOASMEN` reader - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOASMEN` writer - CPU2 IO port A clocks enable during Sleep and Stop modes"]
pub type GPIOASMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOBSMEN` reader - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBSMEN` writer - CPU2 IO port B clocks enable during Sleep and Stop modes"]
pub type GPIOBSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOCSMEN` reader - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCSMEN` writer - CPU2 IO port C clocks enable during Sleep and Stop modes"]
pub type GPIOCSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIODSMEN` reader - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIODSMEN` writer - CPU2 IO port D clocks enable during Sleep and Stop modes"]
pub type GPIODSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOESMEN` reader - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOESMEN` writer - CPU2 IO port E clocks enable during Sleep and Stop modes"]
pub type GPIOESMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `GPIOHSMEN` reader - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHSMEN` writer - CPU2 IO port H clocks enable during Sleep and Stop modes"]
pub type GPIOHSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `ADCFSSMEN` reader - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_R = crate::BitReader<bool>;
#[doc = "Field `ADCFSSMEN` writer - CPU2 ADC clocks enable during Sleep and Stop modes"]
pub type ADCFSSMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
#[doc = "Field `AES1SMEN` reader - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub type AES1SMEN_R = crate::BitReader<bool>;
#[doc = "Field `AES1SMEN` writer - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
pub type AES1SMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AHB2SMENR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&self) -> GPIOHSMEN_R {
        GPIOHSMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&self) -> ADCFSSMEN_R {
        ADCFSSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aes1smen(&self) -> AES1SMEN_R {
        AES1SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU2 IO port A clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioasmen(&mut self) -> GPIOASMEN_W<0> {
        GPIOASMEN_W::new(self)
    }
    #[doc = "Bit 1 - CPU2 IO port B clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiobsmen(&mut self) -> GPIOBSMEN_W<1> {
        GPIOBSMEN_W::new(self)
    }
    #[doc = "Bit 2 - CPU2 IO port C clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiocsmen(&mut self) -> GPIOCSMEN_W<2> {
        GPIOCSMEN_W::new(self)
    }
    #[doc = "Bit 3 - CPU2 IO port D clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiodsmen(&mut self) -> GPIODSMEN_W<3> {
        GPIODSMEN_W::new(self)
    }
    #[doc = "Bit 4 - CPU2 IO port E clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpioesmen(&mut self) -> GPIOESMEN_W<4> {
        GPIOESMEN_W::new(self)
    }
    #[doc = "Bit 7 - CPU2 IO port H clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn gpiohsmen(&mut self) -> GPIOHSMEN_W<7> {
        GPIOHSMEN_W::new(self)
    }
    #[doc = "Bit 13 - CPU2 ADC clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn adcfssmen(&mut self) -> ADCFSSMEN_W<13> {
        ADCFSSMEN_W::new(self)
    }
    #[doc = "Bit 16 - CPU2 AES1 accelerator clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn aes1smen(&mut self) -> AES1SMEN_W<16> {
        AES1SMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ahb2smenr](index.html) module"]
pub struct C2AHB2SMENR_SPEC;
impl crate::RegisterSpec for C2AHB2SMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2ahb2smenr::R](R) reader structure"]
impl crate::Readable for C2AHB2SMENR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2ahb2smenr::W](W) writer structure"]
impl crate::Writable for C2AHB2SMENR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2AHB2SMENR to value 0x0001_209f"]
impl crate::Resettable for C2AHB2SMENR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_209f
    }
}
