#[doc = "Register `CSR1` reader"]
pub struct R(crate::R<CSR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PVDO` reader - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
pub type PVDO_R = crate::BitReader<bool>;
#[doc = "Field `ACTVOSRDY` reader - Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
pub type ACTVOSRDY_R = crate::BitReader<bool>;
#[doc = "Field `ACTVOS` reader - VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
pub type ACTVOS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AVDO` reader - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
pub type AVDO_R = crate::BitReader<bool>;
#[doc = "Field `MMCVDO` reader - MMCVDO"]
pub type MMCVDO_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 4 - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn pvdo(&self) -> PVDO_R {
        PVDO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn actvosrdy(&self) -> ACTVOSRDY_R {
        ACTVOSRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
    #[inline(always)]
    pub fn actvos(&self) -> ACTVOS_R {
        ACTVOS_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn avdo(&self) -> AVDO_R {
        AVDO_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MMCVDO"]
    #[inline(always)]
    pub fn mmcvdo(&self) -> MMCVDO_R {
        MMCVDO_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "PWR control status register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr1](index.html) module"]
pub struct CSR1_SPEC;
impl crate::RegisterSpec for CSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr1::R](R) reader structure"]
impl crate::Readable for CSR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR1 to value 0x4000"]
impl crate::Resettable for CSR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
