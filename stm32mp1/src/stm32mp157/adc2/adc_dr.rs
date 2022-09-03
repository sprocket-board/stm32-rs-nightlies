#[doc = "Register `ADC_DR` reader"]
pub struct R(crate::R<ADC_DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_DR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RDATA` reader - RDATA"]
pub type RDATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - RDATA"]
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new(self.bits)
    }
}
#[doc = "ADC regular Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_dr](index.html) module"]
pub struct ADC_DR_SPEC;
impl crate::RegisterSpec for ADC_DR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_dr::R](R) reader structure"]
impl crate::Readable for ADC_DR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_DR to value 0"]
impl crate::Resettable for ADC_DR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
