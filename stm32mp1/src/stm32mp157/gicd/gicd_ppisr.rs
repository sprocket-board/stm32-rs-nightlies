#[doc = "Register `GICD_PPISR` reader"]
pub struct R(crate::R<GICD_PPISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_PPISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_PPISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_PPISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PPI6` reader - PPI6"]
pub type PPI6_R = crate::BitReader<bool>;
#[doc = "Field `PPI5` reader - PPI5"]
pub type PPI5_R = crate::BitReader<bool>;
#[doc = "Field `PPI4` reader - PPI4"]
pub type PPI4_R = crate::BitReader<bool>;
#[doc = "Field `PPI0` reader - PPI0"]
pub type PPI0_R = crate::BitReader<bool>;
#[doc = "Field `PPI1` reader - PPI1"]
pub type PPI1_R = crate::BitReader<bool>;
#[doc = "Field `PPI2` reader - PPI2"]
pub type PPI2_R = crate::BitReader<bool>;
#[doc = "Field `PPI3` reader - PPI3"]
pub type PPI3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 9 - PPI6"]
    #[inline(always)]
    pub fn ppi6(&self) -> PPI6_R {
        PPI6_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PPI5"]
    #[inline(always)]
    pub fn ppi5(&self) -> PPI5_R {
        PPI5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PPI4"]
    #[inline(always)]
    pub fn ppi4(&self) -> PPI4_R {
        PPI4_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PPI0"]
    #[inline(always)]
    pub fn ppi0(&self) -> PPI0_R {
        PPI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PPI1"]
    #[inline(always)]
    pub fn ppi1(&self) -> PPI1_R {
        PPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PPI2"]
    #[inline(always)]
    pub fn ppi2(&self) -> PPI2_R {
        PPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PPI3"]
    #[inline(always)]
    pub fn ppi3(&self) -> PPI3_R {
        PPI3_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "GICD private peripheral interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_ppisr](index.html) module"]
pub struct GICD_PPISR_SPEC;
impl crate::RegisterSpec for GICD_PPISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_ppisr::R](R) reader structure"]
impl crate::Readable for GICD_PPISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_PPISR to value 0"]
impl crate::Resettable for GICD_PPISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
