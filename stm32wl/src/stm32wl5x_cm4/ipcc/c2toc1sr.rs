#[doc = "Register `C2TOC1SR` reader"]
pub struct R(crate::R<C2TOC1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2TOC1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2TOC1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2TOC1SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH1F` reader - CH1F"]
pub type CH1F_R = crate::BitReader<CH1F_A>;
#[doc = "CH1F\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1F_A {
    #[doc = "0: Channel free, data can be written by the sending processor. Generates a channel TX free interrupt to the current processor, when unmasked"]
    Free = 0,
    #[doc = "1: Channel occupied, data can be read by the receiving processor. Generates a channel RX occupied interrupt to the other processor, when unmasked"]
    Occupied = 1,
}
impl From<CH1F_A> for bool {
    #[inline(always)]
    fn from(variant: CH1F_A) -> Self {
        variant as u8 != 0
    }
}
impl CH1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1F_A {
        match self.bits {
            false => CH1F_A::Free,
            true => CH1F_A::Occupied,
        }
    }
    #[doc = "Checks if the value of the field is `Free`"]
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == CH1F_A::Free
    }
    #[doc = "Checks if the value of the field is `Occupied`"]
    #[inline(always)]
    pub fn is_occupied(&self) -> bool {
        *self == CH1F_A::Occupied
    }
}
#[doc = "Field `CH2F` reader - CH2F"]
pub use CH1F_R as CH2F_R;
#[doc = "Field `CH3F` reader - CH3F"]
pub use CH1F_R as CH3F_R;
#[doc = "Field `CH4F` reader - CH4F"]
pub use CH1F_R as CH4F_R;
#[doc = "Field `CH5F` reader - CH5F"]
pub use CH1F_R as CH5F_R;
#[doc = "Field `CH6F` reader - CH6F"]
pub type CH6F_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CH1F"]
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH2F"]
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH3F"]
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH4F"]
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH5F"]
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH6F"]
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "IPCC processor 2 to processor 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2toc1sr](index.html) module"]
pub struct C2TOC1SR_SPEC;
impl crate::RegisterSpec for C2TOC1SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2toc1sr::R](R) reader structure"]
impl crate::Readable for C2TOC1SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C2TOC1SR to value 0"]
impl crate::Resettable for C2TOC1SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
