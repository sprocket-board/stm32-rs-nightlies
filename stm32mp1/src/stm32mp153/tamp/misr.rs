#[doc = "Register `MISR` reader"]
pub struct R(crate::R<MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TAMP1MF` reader - TAMP1MF"]
pub type TAMP1MF_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2MF` reader - TAMP2MF"]
pub type TAMP2MF_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3MF` reader - TAMP3MF"]
pub type TAMP3MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP1MF` reader - ITAMP1MF"]
pub type ITAMP1MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP2MF` reader - ITAMP2MF"]
pub type ITAMP2MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP3MF` reader - ITAMP3MF"]
pub type ITAMP3MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP4MF` reader - ITAMP4MF"]
pub type ITAMP4MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP5MF` reader - ITAMP5MF"]
pub type ITAMP5MF_R = crate::BitReader<bool>;
#[doc = "Field `ITAMP8MF` reader - ITAMP8MF"]
pub type ITAMP8MF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - TAMP1MF"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> TAMP1MF_R {
        TAMP1MF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TAMP2MF"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> TAMP2MF_R {
        TAMP2MF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TAMP3MF"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> TAMP3MF_R {
        TAMP3MF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - ITAMP1MF"]
    #[inline(always)]
    pub fn itamp1mf(&self) -> ITAMP1MF_R {
        ITAMP1MF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ITAMP2MF"]
    #[inline(always)]
    pub fn itamp2mf(&self) -> ITAMP2MF_R {
        ITAMP2MF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ITAMP3MF"]
    #[inline(always)]
    pub fn itamp3mf(&self) -> ITAMP3MF_R {
        ITAMP3MF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ITAMP4MF"]
    #[inline(always)]
    pub fn itamp4mf(&self) -> ITAMP4MF_R {
        ITAMP4MF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ITAMP5MF"]
    #[inline(always)]
    pub fn itamp5mf(&self) -> ITAMP5MF_R {
        ITAMP5MF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - ITAMP8MF"]
    #[inline(always)]
    pub fn itamp8mf(&self) -> ITAMP8MF_R {
        ITAMP8MF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "TAMP non-secure masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misr](index.html) module"]
pub struct MISR_SPEC;
impl crate::RegisterSpec for MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misr::R](R) reader structure"]
impl crate::Readable for MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
