#[doc = "Register `OTG_DTXFSTS7` reader"]
pub struct R(crate::R<OTG_DTXFSTS7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_DTXFSTS7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_DTXFSTS7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_DTXFSTS7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPTFSAV` reader - INEPTFSAV"]
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - INEPTFSAV"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This read-only register contains the free space information for the device IN endpoint Tx FIFO.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_dtxfsts7](index.html) module"]
pub struct OTG_DTXFSTS7_SPEC;
impl crate::RegisterSpec for OTG_DTXFSTS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_dtxfsts7::R](R) reader structure"]
impl crate::Readable for OTG_DTXFSTS7_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_DTXFSTS7 to value 0x0200"]
impl crate::Resettable for OTG_DTXFSTS7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200
    }
}
