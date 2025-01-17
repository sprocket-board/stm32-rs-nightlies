#[doc = "Register `OTG_FS_DTXFSTS2` reader"]
pub struct R(crate::R<OTG_FS_DTXFSTS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DTXFSTS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DTXFSTS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DTXFSTS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space available"]
pub type INEPTFSAV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space available"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device IN endpoint transmit FIFO status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_dtxfsts2](index.html) module"]
pub struct OTG_FS_DTXFSTS2_SPEC;
impl crate::RegisterSpec for OTG_FS_DTXFSTS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_dtxfsts2::R](R) reader structure"]
impl crate::Readable for OTG_FS_DTXFSTS2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_FS_DTXFSTS2 to value 0"]
impl crate::Resettable for OTG_FS_DTXFSTS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
