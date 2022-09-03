#[doc = "Register `PSR` reader"]
pub struct R(crate::R<PSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSR` writer"]
pub struct W(crate::W<PSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSR_SPEC>;
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
impl From<crate::W<PSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEC` reader - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ACT` reader - Activity Monitors the moduleâ\u{80}\u{99}s CAN communication state."]
pub type ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EP` reader - Error passive"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EW` reader - Warning Sstatus"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `BO` reader - Bus_Off status"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `DLEC` reader - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DLEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLEC` writer - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
pub type DLEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RESI_R = crate::BitReader<bool>;
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RBRS_R = crate::BitReader<bool>;
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
pub type RBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `REDL` reader - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type REDL_R = crate::BitReader<bool>;
#[doc = "Field `REDL` writer - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
pub type REDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PXE` reader - Protocol exception event"]
pub type PXE_R = crate::BitReader<bool>;
#[doc = "Field `PXE` writer - Protocol exception event"]
pub type PXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `TDCV` reader - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
pub type TDCV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity Monitors the moduleâ\u{80}\u{99}s CAN communication state."]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Sstatus"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter delay compensation value Position of the secondary sample point, defined by the sum of the measured delay from FDCAN_TX to FDCAN_RX and TDCR.TDCO. The SSP position is, in the data phase, the number of minimum time quanta (mtq) between the start of the transmitted bit and the secondary sample point. Valid values are 0 to 127 mtq."]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last error code The LEC indicates the type of the last error to occur on the CAN bus. This field is cleared to 0 when a message has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<0> {
        LEC_W::new(self)
    }
    #[doc = "Bits 8:10 - Data last error code Type of last error that occurred in the data phase of a FDCAN format frame with its BRS flag set. Coding is the same as for LEC. This field is cleared to 0 when a FDCAN format frame with its BRS flag set has been transferred (reception or transmission) without error. Access type is RS: set on read."]
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W<8> {
        DLEC_W::new(self)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W<11> {
        RESI_W::new(self)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN message This bit is set together with REDL, independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W<12> {
        RBRS_W::new(self)
    }
    #[doc = "Bit 13 - Received FDCAN message This bit is set independent of acceptance filtering. Access type is RX: reset on read."]
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W<13> {
        REDL_W::new(self)
    }
    #[doc = "Bit 14 - Protocol exception event"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W<14> {
        PXE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN protocol status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
pub struct PSR_SPEC;
impl crate::RegisterSpec for PSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [psr::R](R) reader structure"]
impl crate::Readable for PSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [psr::W](W) writer structure"]
impl crate::Writable for PSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSR to value 0x0707"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0707
    }
}
