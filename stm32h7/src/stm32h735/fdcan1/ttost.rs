#[doc = "Register `TTOST` reader"]
pub struct R(crate::R<TTOST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TTOST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TTOST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TTOST` writer"]
pub struct W(crate::W<TTOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TTOST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TTOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TTOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EL` reader - Error Level"]
pub type EL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EL` writer - Error Level"]
pub type EL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOST_SPEC, u8, u8, 2, O>;
#[doc = "Field `MS` reader - Master State."]
pub type MS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MS` writer - Master State."]
pub type MS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOST_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYS` reader - Synchronization State"]
pub type SYS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYS` writer - Synchronization State"]
pub type SYS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOST_SPEC, u8, u8, 2, O>;
#[doc = "Field `QGTP` reader - Quality of Global Time Phase"]
pub type QGTP_R = crate::BitReader<bool>;
#[doc = "Field `QGTP` writer - Quality of Global Time Phase"]
pub type QGTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub type QCS_R = crate::BitReader<bool>;
#[doc = "Field `QCS` writer - Quality of Clock Speed"]
pub type QCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub type RTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTO` writer - Reference Trigger Offset"]
pub type RTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOST_SPEC, u8, u8, 8, O>;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub type WGTD_R = crate::BitReader<bool>;
#[doc = "Field `WGTD` writer - Wait for Global Time Discontinuity"]
pub type WGTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub type GFI_R = crate::BitReader<bool>;
#[doc = "Field `GFI` writer - Gap Finished Indicator."]
pub type GFI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `TMP` reader - Time Master Priority"]
pub type TMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMP` writer - Time Master Priority"]
pub type TMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TTOST_SPEC, u8, u8, 3, O>;
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub type GSI_R = crate::BitReader<bool>;
#[doc = "Field `GSI` writer - Gap Started Indicator."]
pub type GSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `WFE` reader - Wait for Event"]
pub type WFE_R = crate::BitReader<bool>;
#[doc = "Field `WFE` writer - Wait for Event"]
pub type WFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub type AWE_R = crate::BitReader<bool>;
#[doc = "Field `AWE` writer - Application Watchdog Event"]
pub type AWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub type WECS_R = crate::BitReader<bool>;
#[doc = "Field `WECS` writer - Wait for External Clock Synchronization"]
pub type WECS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub type SPL_R = crate::BitReader<bool>;
#[doc = "Field `SPL` writer - Schedule Phase Lock"]
pub type SPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TTOST_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> EL_R {
        EL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn qgtp(&self) -> QGTP_R {
        QGTP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QCS_R {
        QCS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RTO_R {
        RTO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WGTD_R {
        WGTD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GFI_R {
        GFI_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TMP_R {
        TMP_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GSI_R {
        GSI_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WFE_R {
        WFE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AWE_R {
        AWE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WECS_R {
        WECS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SPL_R {
        SPL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&mut self) -> EL_W<0> {
        EL_W::new(self)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&mut self) -> MS_W<2> {
        MS_W::new(self)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W<4> {
        SYS_W::new(self)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn qgtp(&mut self) -> QGTP_W<6> {
        QGTP_W::new(self)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&mut self) -> QCS_W<7> {
        QCS_W::new(self)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&mut self) -> RTO_W<8> {
        RTO_W::new(self)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&mut self) -> WGTD_W<22> {
        WGTD_W::new(self)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&mut self) -> GFI_W<23> {
        GFI_W::new(self)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&mut self) -> TMP_W<24> {
        TMP_W::new(self)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&mut self) -> GSI_W<27> {
        GSI_W::new(self)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WFE_W<28> {
        WFE_W::new(self)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&mut self) -> AWE_W<29> {
        AWE_W::new(self)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&mut self) -> WECS_W<30> {
        WECS_W::new(self)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&mut self) -> SPL_W<31> {
        SPL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ttost](index.html) module"]
pub struct TTOST_SPEC;
impl crate::RegisterSpec for TTOST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ttost::R](R) reader structure"]
impl crate::Readable for TTOST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ttost::W](W) writer structure"]
impl crate::Writable for TTOST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TTOST to value 0"]
impl crate::Resettable for TTOST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
