#[doc = "Register `FMC_BCHPBR2` reader"]
pub struct R(crate::R<FMC_BCHPBR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_BCHPBR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_BCHPBR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_BCHPBR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BCHPB` reader - BCHPB"]
pub type BCHPB_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - BCHPB"]
    #[inline(always)]
    pub fn bchpb(&self) -> BCHPB_R {
        BCHPB_R::new(self.bits)
    }
}
#[doc = "FMC BCH Parity Bits Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_bchpbr2](index.html) module"]
pub struct FMC_BCHPBR2_SPEC;
impl crate::RegisterSpec for FMC_BCHPBR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_bchpbr2::R](R) reader structure"]
impl crate::Readable for FMC_BCHPBR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_BCHPBR2 to value 0"]
impl crate::Resettable for FMC_BCHPBR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
