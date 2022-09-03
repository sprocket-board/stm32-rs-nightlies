#[doc = "Register `ITLINE29` reader"]
pub struct R(crate::R<ITLINE29_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ITLINE29_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ITLINE29_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ITLINE29_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `USART3` reader - USART3"]
pub type USART3_R = crate::BitReader<bool>;
#[doc = "Field `USART4` reader - USART4"]
pub type USART4_R = crate::BitReader<bool>;
#[doc = "Field `USART5` reader - USART5"]
pub type USART5_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - USART3"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART4"]
    #[inline(always)]
    pub fn usart4(&self) -> USART4_R {
        USART4_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USART5"]
    #[inline(always)]
    pub fn usart5(&self) -> USART5_R {
        USART5_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "interrupt line 29 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itline29](index.html) module"]
pub struct ITLINE29_SPEC;
impl crate::RegisterSpec for ITLINE29_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [itline29::R](R) reader structure"]
impl crate::Readable for ITLINE29_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ITLINE29 to value 0"]
impl crate::Resettable for ITLINE29_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
