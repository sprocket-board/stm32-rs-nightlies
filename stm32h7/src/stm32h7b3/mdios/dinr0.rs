#[doc = "Register `DINR0` reader"]
pub struct R(crate::R<DINR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DINR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DINR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DINR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIN0` reader - Input data received from MDIO Master during write frames"]
pub type DIN0_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Input data received from MDIO Master during write frames"]
    #[inline(always)]
    pub fn din0(&self) -> DIN0_R {
        DIN0_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MDIOS input data register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dinr0](index.html) module"]
pub struct DINR0_SPEC;
impl crate::RegisterSpec for DINR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dinr0::R](R) reader structure"]
impl crate::Readable for DINR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DINR0 to value 0"]
impl crate::Resettable for DINR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
