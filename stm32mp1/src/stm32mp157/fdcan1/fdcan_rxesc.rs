#[doc = "Register `FDCAN_RXESC` reader"]
pub struct R(crate::R<FDCAN_RXESC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_RXESC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_RXESC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_RXESC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `F0DS` reader - F0DS"]
pub type F0DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `F1DS` reader - F1DS"]
pub type F1DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RBDS` reader - RBDS"]
pub type RBDS_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - F0DS"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - F1DS"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - RBDS"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
#[doc = "Configures the number of data bytes belonging to an Rx buffer / Rx FIFO element. Data field sizes higher than 8 bytes are intended for CAN FD operation only.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_rxesc](index.html) module"]
pub struct FDCAN_RXESC_SPEC;
impl crate::RegisterSpec for FDCAN_RXESC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_rxesc::R](R) reader structure"]
impl crate::Readable for FDCAN_RXESC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_RXESC to value 0"]
impl crate::Resettable for FDCAN_RXESC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
