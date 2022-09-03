#[doc = "Register `RXF0S` reader"]
pub struct R(crate::R<RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0FL` reader - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
pub type F0FL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0GI` reader - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
pub type F0GI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0PI` reader - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
pub type F0PI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F0F` reader - Rx FIFO 0 full"]
pub type F0F_R = crate::BitReader<bool>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\]
is reset, this bit is also reset."]
pub type RF0L_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Rx FIFO 0 fill level Number of elements stored in Rx FIFO 0, range 0 to 3."]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Rx FIFO 0 get index Rx FIFO 0 read index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Rx FIFO 0 put index Rx FIFO 0 write index pointer, range 0 to 2."]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 message lost This bit is a copy of interrupt flag IR\\[RF0L\\]. When IR\\[RF0L\\]
is reset, this bit is also reset."]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "FDCAN Rx FIFO 0 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](index.html) module"]
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0s::R](R) reader structure"]
impl crate::Readable for RXF0S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
