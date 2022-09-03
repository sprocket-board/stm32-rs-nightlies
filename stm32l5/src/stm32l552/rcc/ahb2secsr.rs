#[doc = "Register `AHB2SECSR` reader"]
pub struct R(crate::R<AHB2SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB2SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB2SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB2SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `GPIOASECF` reader - GPIOASECF"]
pub type GPIOASECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOBSECF` reader - GPIOBSECF"]
pub type GPIOBSECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOCSECF` reader - GPIOCSECF"]
pub type GPIOCSECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIODSECF` reader - GPIODSECF"]
pub type GPIODSECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOESECF` reader - GPIOESECF"]
pub type GPIOESECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOFSECF` reader - GPIOFSECF"]
pub type GPIOFSECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOGSECF` reader - GPIOGSECF"]
pub type GPIOGSECF_R = crate::BitReader<bool>;
#[doc = "Field `GPIOHSECF` reader - GPIOHSECF"]
pub type GPIOHSECF_R = crate::BitReader<bool>;
#[doc = "Field `SRAM2SECF` reader - SRAM2SECF"]
pub type SRAM2SECF_R = crate::BitReader<bool>;
#[doc = "Field `OTFDEC1SECF` reader - OTFDEC1SECF"]
pub type OTFDEC1SECF_R = crate::BitReader<bool>;
#[doc = "Field `SDMMC1SECF` reader - SDMMC1SECF"]
pub type SDMMC1SECF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - GPIOASECF"]
    #[inline(always)]
    pub fn gpioasecf(&self) -> GPIOASECF_R {
        GPIOASECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBSECF"]
    #[inline(always)]
    pub fn gpiobsecf(&self) -> GPIOBSECF_R {
        GPIOBSECF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCSECF"]
    #[inline(always)]
    pub fn gpiocsecf(&self) -> GPIOCSECF_R {
        GPIOCSECF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODSECF"]
    #[inline(always)]
    pub fn gpiodsecf(&self) -> GPIODSECF_R {
        GPIODSECF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOESECF"]
    #[inline(always)]
    pub fn gpioesecf(&self) -> GPIOESECF_R {
        GPIOESECF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFSECF"]
    #[inline(always)]
    pub fn gpiofsecf(&self) -> GPIOFSECF_R {
        GPIOFSECF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOGSECF"]
    #[inline(always)]
    pub fn gpiogsecf(&self) -> GPIOGSECF_R {
        GPIOGSECF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOHSECF"]
    #[inline(always)]
    pub fn gpiohsecf(&self) -> GPIOHSECF_R {
        GPIOHSECF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM2SECF"]
    #[inline(always)]
    pub fn sram2secf(&self) -> SRAM2SECF_R {
        SRAM2SECF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 21 - OTFDEC1SECF"]
    #[inline(always)]
    pub fn otfdec1secf(&self) -> OTFDEC1SECF_R {
        OTFDEC1SECF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDMMC1SECF"]
    #[inline(always)]
    pub fn sdmmc1secf(&self) -> SDMMC1SECF_R {
        SDMMC1SECF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "RCC AHB2 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb2secsr](index.html) module"]
pub struct AHB2SECSR_SPEC;
impl crate::RegisterSpec for AHB2SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb2secsr::R](R) reader structure"]
impl crate::Readable for AHB2SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB2SECSR to value 0x0020_02ff"]
impl crate::Resettable for AHB2SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_02ff
    }
}
