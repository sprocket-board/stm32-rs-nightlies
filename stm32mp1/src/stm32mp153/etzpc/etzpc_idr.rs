#[doc = "Register `ETZPC_IDR` reader"]
pub struct R(crate::R<ETZPC_IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETZPC_IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETZPC_IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETZPC_IDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "ETZPC IP version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etzpc_idr](index.html) module"]
pub struct ETZPC_IDR_SPEC;
impl crate::RegisterSpec for ETZPC_IDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etzpc_idr::R](R) reader structure"]
impl crate::Readable for ETZPC_IDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETZPC_IDR to value 0x0010_0061"]
impl crate::Resettable for ETZPC_IDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0010_0061
    }
}
