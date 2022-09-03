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
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ACT` reader - Activity"]
pub type ACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACT` writer - Activity"]
pub type ACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 2, O>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EP_R = crate::BitReader<bool>;
#[doc = "Field `EP` writer - Error Passive"]
pub type EP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EW_R = crate::BitReader<bool>;
#[doc = "Field `EW` writer - Warning Status"]
pub type EW_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BO_R = crate::BitReader<bool>;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `DLEC` reader - Data Last Error Code"]
pub type DLEC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLEC` writer - Data Last Error Code"]
pub type DLEC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 3, O>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN Message"]
pub type RESI_R = crate::BitReader<bool>;
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN Message"]
pub type RESI_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN Message"]
pub type RBRS_R = crate::BitReader<bool>;
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN Message"]
pub type RBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `REDL` reader - Received FDCAN Message"]
pub type REDL_R = crate::BitReader<bool>;
#[doc = "Field `REDL` writer - Received FDCAN Message"]
pub type REDL_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PXE_R = crate::BitReader<bool>;
#[doc = "Field `PXE` writer - Protocol Exception Event"]
pub type PXE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSR_SPEC, bool, O>;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TDCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDCV` writer - Transmitter Delay Compensation Value"]
pub type TDCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PSR_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LEC_W<0> {
        LEC_W::new(self)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&mut self) -> ACT_W<3> {
        ACT_W::new(self)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<5> {
        EP_W::new(self)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<6> {
        EW_W::new(self)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<7> {
        BO_W::new(self)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&mut self) -> DLEC_W<8> {
        DLEC_W::new(self)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W<11> {
        RESI_W::new(self)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W<12> {
        RBRS_W::new(self)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W<13> {
        REDL_W::new(self)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W<14> {
        PXE_W::new(self)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&mut self) -> TDCV_W<16> {
        TDCV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Protocol Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psr](index.html) module"]
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
#[doc = "`reset()` method sets PSR to value 0"]
impl crate::Resettable for PSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
