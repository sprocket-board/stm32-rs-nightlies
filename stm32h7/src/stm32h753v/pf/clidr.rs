#[doc = "Register `CLIDR` reader"]
pub struct R(crate::R<CLIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CL1` reader - CL1"]
pub type CL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL2` reader - CL2"]
pub type CL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL3` reader - CL3"]
pub type CL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL4` reader - CL4"]
pub type CL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL5` reader - CL5"]
pub type CL5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL6` reader - CL6"]
pub type CL6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CL7` reader - CL7"]
pub type CL7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LoUIS` reader - LoUIS"]
pub type LO_UIS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LoC` reader - LoC"]
pub type LO_C_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LoU` reader - LoU"]
pub type LO_U_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - CL1"]
    #[inline(always)]
    pub fn cl1(&self) -> CL1_R {
        CL1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - CL2"]
    #[inline(always)]
    pub fn cl2(&self) -> CL2_R {
        CL2_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - CL3"]
    #[inline(always)]
    pub fn cl3(&self) -> CL3_R {
        CL3_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - CL4"]
    #[inline(always)]
    pub fn cl4(&self) -> CL4_R {
        CL4_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - CL5"]
    #[inline(always)]
    pub fn cl5(&self) -> CL5_R {
        CL5_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - CL6"]
    #[inline(always)]
    pub fn cl6(&self) -> CL6_R {
        CL6_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - CL7"]
    #[inline(always)]
    pub fn cl7(&self) -> CL7_R {
        CL7_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - LoUIS"]
    #[inline(always)]
    pub fn lo_uis(&self) -> LO_UIS_R {
        LO_UIS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - LoC"]
    #[inline(always)]
    pub fn lo_c(&self) -> LO_C_R {
        LO_C_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - LoU"]
    #[inline(always)]
    pub fn lo_u(&self) -> LO_U_R {
        LO_U_R::new(((self.bits >> 27) & 7) as u8)
    }
}
#[doc = "Cache Level ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clidr](index.html) module"]
pub struct CLIDR_SPEC;
impl crate::RegisterSpec for CLIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clidr::R](R) reader structure"]
impl crate::Readable for CLIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLIDR to value 0x0900_0003"]
impl crate::Resettable for CLIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0900_0003
    }
}
