#[doc = "Register `I2C_PECR` reader"]
pub struct R(crate::R<I2C_PECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C_PECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C_PECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C_PECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PEC` reader - PEC"]
pub type PEC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - PEC"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Access: No wait states\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c_pecr](index.html) module"]
pub struct I2C_PECR_SPEC;
impl crate::RegisterSpec for I2C_PECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c_pecr::R](R) reader structure"]
impl crate::Readable for I2C_PECR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets I2C_PECR to value 0"]
impl crate::Resettable for I2C_PECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}