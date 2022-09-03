#[doc = "Register `DFSDM_FLT1JDATAR` reader"]
pub struct R(crate::R<DFSDM_FLT1JDATAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSDM_FLT1JDATAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSDM_FLT1JDATAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSDM_FLT1JDATAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JDATACH` reader - JDATACH"]
pub type JDATACH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `JDATA` reader - JDATA"]
pub type JDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - JDATACH"]
    #[inline(always)]
    pub fn jdatach(&self) -> JDATACH_R {
        JDATACH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - JDATA"]
    #[inline(always)]
    pub fn jdata(&self) -> JDATA_R {
        JDATA_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
#[doc = "DFSDM filter 1 data register for injected group\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsdm_flt1jdatar](index.html) module"]
pub struct DFSDM_FLT1JDATAR_SPEC;
impl crate::RegisterSpec for DFSDM_FLT1JDATAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsdm_flt1jdatar::R](R) reader structure"]
impl crate::Readable for DFSDM_FLT1JDATAR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DFSDM_FLT1JDATAR to value 0"]
impl crate::Resettable for DFSDM_FLT1JDATAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
