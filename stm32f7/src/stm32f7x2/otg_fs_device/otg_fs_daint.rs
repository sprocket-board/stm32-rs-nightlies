#[doc = "Register `OTG_FS_DAINT` reader"]
pub struct R(crate::R<OTG_FS_DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_DAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IEPINT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OEPINT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_daint](index.html) module"]
pub struct OTG_FS_DAINT_SPEC;
impl crate::RegisterSpec for OTG_FS_DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_daint::R](R) reader structure"]
impl crate::Readable for OTG_FS_DAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OTG_FS_DAINT to value 0"]
impl crate::Resettable for OTG_FS_DAINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
