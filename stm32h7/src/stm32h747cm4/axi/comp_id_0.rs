#[doc = "Register `COMP_ID_0` reader"]
pub struct R(crate::R<COMP_ID_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP_ID_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP_ID_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP_ID_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PREAMBLE` reader - Preamble bits 0 to 7"]
pub type PREAMBLE_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Preamble bits 0 to 7"]
    #[inline(always)]
    pub fn preamble(&self) -> PREAMBLE_R {
        PREAMBLE_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXI interconnect - component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp_id_0](index.html) module"]
pub struct COMP_ID_0_SPEC;
impl crate::RegisterSpec for COMP_ID_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp_id_0::R](R) reader structure"]
impl crate::Readable for COMP_ID_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets COMP_ID_0 to value 0x04"]
impl crate::Resettable for COMP_ID_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x04
    }
}
