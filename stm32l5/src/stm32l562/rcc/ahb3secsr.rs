#[doc = "Register `AHB3SECSR` reader"]
pub struct R(crate::R<AHB3SECSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB3SECSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB3SECSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB3SECSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FSMCSECF` reader - FSMCSECF"]
pub type FSMCSECF_R = crate::BitReader<bool>;
#[doc = "Field `OSPI1SECF` reader - OSPI1SECF"]
pub type OSPI1SECF_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - FSMCSECF"]
    #[inline(always)]
    pub fn fsmcsecf(&self) -> FSMCSECF_R {
        FSMCSECF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - OSPI1SECF"]
    #[inline(always)]
    pub fn ospi1secf(&self) -> OSPI1SECF_R {
        OSPI1SECF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "RCC AHB3 security status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb3secsr](index.html) module"]
pub struct AHB3SECSR_SPEC;
impl crate::RegisterSpec for AHB3SECSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb3secsr::R](R) reader structure"]
impl crate::Readable for AHB3SECSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHB3SECSR to value 0"]
impl crate::Resettable for AHB3SECSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
