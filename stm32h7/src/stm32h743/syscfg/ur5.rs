#[doc = "Register `UR5` reader"]
pub struct R(crate::R<UR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MESAD_1` reader - Mass erase secured area disabled for bank 1"]
pub type MESAD_1_R = crate::BitReader<bool>;
#[doc = "Field `WRPN_1` reader - Write protection for flash bank 1"]
pub type WRPN_1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Mass erase secured area disabled for bank 1"]
    #[inline(always)]
    pub fn mesad_1(&self) -> MESAD_1_R {
        MESAD_1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Write protection for flash bank 1"]
    #[inline(always)]
    pub fn wrpn_1(&self) -> WRPN_1_R {
        WRPN_1_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ur5](index.html) module"]
pub struct UR5_SPEC;
impl crate::RegisterSpec for UR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ur5::R](R) reader structure"]
impl crate::Readable for UR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UR5 to value 0"]
impl crate::Resettable for UR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
