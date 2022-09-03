#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PD` reader - PD"]
pub type PD_R = crate::BitReader<bool>;
#[doc = "Field `PSSC` reader - PSSC"]
pub type PSSC_R = crate::BitReader<bool>;
#[doc = "Field `UANC` reader - UANC"]
pub type UANC_R = crate::BitReader<bool>;
#[doc = "Field `PSS0` reader - PSS0"]
pub type PSS0_R = crate::BitReader<bool>;
#[doc = "Field `UAN0` reader - UAN0"]
pub type UAN0_R = crate::BitReader<bool>;
#[doc = "Field `RUE0` reader - RUE0"]
pub type RUE0_R = crate::BitReader<bool>;
#[doc = "Field `PSS1` reader - PSS1"]
pub type PSS1_R = crate::BitReader<bool>;
#[doc = "Field `UAN1` reader - UAN1"]
pub type UAN1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSSC"]
    #[inline(always)]
    pub fn pssc(&self) -> PSSC_R {
        PSSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UANC"]
    #[inline(always)]
    pub fn uanc(&self) -> UANC_R {
        UANC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSS0"]
    #[inline(always)]
    pub fn pss0(&self) -> PSS0_R {
        PSS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UAN0"]
    #[inline(always)]
    pub fn uan0(&self) -> UAN0_R {
        UAN0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RUE0"]
    #[inline(always)]
    pub fn rue0(&self) -> RUE0_R {
        RUE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSS1"]
    #[inline(always)]
    pub fn pss1(&self) -> PSS1_R {
        PSS1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UAN1"]
    #[inline(always)]
    pub fn uan1(&self) -> UAN1_R {
        UAN1_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host PHY status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSR to value 0x1528"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1528
    }
}
