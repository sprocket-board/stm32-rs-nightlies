#[doc = "Register `C1MISR` reader"]
pub struct R(crate::R<C1MISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1MISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1MISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1MISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MISF0` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub type MISF0_R = crate::BitReader<MISF0_A>;
#[doc = "masked interrupt(N) semaphore n status bit after enable (mask)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MISF0_A {
    #[doc = "0: No interrupt pending after masking"]
    NotPending = 0,
    #[doc = "1: Interrupt pending after masking"]
    Pending = 1,
}
impl From<MISF0_A> for bool {
    #[inline(always)]
    fn from(variant: MISF0_A) -> Self {
        variant as u8 != 0
    }
}
impl MISF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MISF0_A {
        match self.bits {
            false => MISF0_A::NotPending,
            true => MISF0_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == MISF0_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == MISF0_A::Pending
    }
}
#[doc = "Field `MISF1` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF1_R;
#[doc = "Field `MISF2` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF2_R;
#[doc = "Field `MISF3` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF3_R;
#[doc = "Field `MISF4` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF4_R;
#[doc = "Field `MISF5` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF5_R;
#[doc = "Field `MISF6` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF6_R;
#[doc = "Field `MISF7` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF7_R;
#[doc = "Field `MISF8` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF8_R;
#[doc = "Field `MISF9` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF9_R;
#[doc = "Field `MISF10` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF10_R;
#[doc = "Field `MISF11` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF11_R;
#[doc = "Field `MISF12` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF12_R;
#[doc = "Field `MISF13` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF13_R;
#[doc = "Field `MISF14` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF14_R;
#[doc = "Field `MISF15` reader - masked interrupt(N) semaphore n status bit after enable (mask)"]
pub use MISF0_R as MISF15_R;
impl R {
    #[doc = "Bit 0 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf0(&self) -> MISF0_R {
        MISF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf1(&self) -> MISF1_R {
        MISF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf2(&self) -> MISF2_R {
        MISF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf3(&self) -> MISF3_R {
        MISF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf4(&self) -> MISF4_R {
        MISF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf5(&self) -> MISF5_R {
        MISF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf6(&self) -> MISF6_R {
        MISF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf7(&self) -> MISF7_R {
        MISF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf8(&self) -> MISF8_R {
        MISF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf9(&self) -> MISF9_R {
        MISF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf10(&self) -> MISF10_R {
        MISF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf11(&self) -> MISF11_R {
        MISF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf12(&self) -> MISF12_R {
        MISF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf13(&self) -> MISF13_R {
        MISF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf14(&self) -> MISF14_R {
        MISF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - masked interrupt(N) semaphore n status bit after enable (mask)"]
    #[inline(always)]
    pub fn misf15(&self) -> MISF15_R {
        MISF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "HSEM Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1misr](index.html) module"]
pub struct C1MISR_SPEC;
impl crate::RegisterSpec for C1MISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1misr::R](R) reader structure"]
impl crate::Readable for C1MISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C1MISR to value 0"]
impl crate::Resettable for C1MISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
