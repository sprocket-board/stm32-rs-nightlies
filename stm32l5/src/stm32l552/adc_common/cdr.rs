#[doc = "Register `CDR` reader"]
pub struct R(crate::R<CDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA_MST` reader - RDATA_MST"]
pub type RDATA_MST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RDATA_SLV` reader - RDATA_SLV"]
pub type RDATA_SLV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - RDATA_MST"]
    #[inline(always)]
    pub fn rdata_mst(&self) -> RDATA_MST_R {
        RDATA_MST_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - RDATA_SLV"]
    #[inline(always)]
    pub fn rdata_slv(&self) -> RDATA_SLV_R {
        RDATA_SLV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Common regular data register for dual mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdr](index.html) module"]
pub struct CDR_SPEC;
impl crate::RegisterSpec for CDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdr::R](R) reader structure"]
impl crate::Readable for CDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CDR to value 0"]
impl crate::Resettable for CDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
