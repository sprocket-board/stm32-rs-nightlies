#[doc = "Register `I2C_IPIDR` reader"]
pub struct R(crate::R<I2C_IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_IPIDR_SPEC>) -> Self {
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
#[doc = "I2C identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_ipidr](index.html) module"]
pub struct I2C_IPIDR_SPEC;
impl crate::RegisterSpec for I2C_IPIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_ipidr::R](R) reader structure"]
impl crate::Readable for I2C_IPIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_IPIDR to value 0x0013_0012"]
impl crate::Resettable for I2C_IPIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0013_0012
    }
}
