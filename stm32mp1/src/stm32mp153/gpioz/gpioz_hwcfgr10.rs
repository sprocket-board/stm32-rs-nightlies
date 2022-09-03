#[doc = "Register `GPIOZ_HWCFGR10` reader"]
pub struct R(crate::R<GPIOZ_HWCFGR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIOZ_HWCFGR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIOZ_HWCFGR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIOZ_HWCFGR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AHB_IOP` reader - AHB_IOP"]
pub type AHB_IOP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AF_SIZE` reader - AF_SIZE"]
pub type AF_SIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED_CFG` reader - SPEED_CFG"]
pub type SPEED_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LOCK_CFG` reader - LOCK_CFG"]
pub type LOCK_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SEC_CFG` reader - SEC_CFG"]
pub type SEC_CFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OR_CFG` reader - OR_CFG"]
pub type OR_CFG_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - AHB_IOP"]
    #[inline(always)]
    pub fn ahb_iop(&self) -> AHB_IOP_R {
        AHB_IOP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AF_SIZE"]
    #[inline(always)]
    pub fn af_size(&self) -> AF_SIZE_R {
        AF_SIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SPEED_CFG"]
    #[inline(always)]
    pub fn speed_cfg(&self) -> SPEED_CFG_R {
        SPEED_CFG_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - LOCK_CFG"]
    #[inline(always)]
    pub fn lock_cfg(&self) -> LOCK_CFG_R {
        LOCK_CFG_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SEC_CFG"]
    #[inline(always)]
    pub fn sec_cfg(&self) -> SEC_CFG_R {
        SEC_CFG_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - OR_CFG"]
    #[inline(always)]
    pub fn or_cfg(&self) -> OR_CFG_R {
        OR_CFG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[doc = "For GPIOA, B, C, D, E, F, G, H, I, J and GPIOK: For GPIOZ:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpioz_hwcfgr10](index.html) module"]
pub struct GPIOZ_HWCFGR10_SPEC;
impl crate::RegisterSpec for GPIOZ_HWCFGR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpioz_hwcfgr10::R](R) reader structure"]
impl crate::Readable for GPIOZ_HWCFGR10_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GPIOZ_HWCFGR10 to value 0x0001_1240"]
impl crate::Resettable for GPIOZ_HWCFGR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_1240
    }
}