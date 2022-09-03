#[doc = "Register `FDCAN_DBTP` reader"]
pub struct R(crate::R<FDCAN_DBTP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_DBTP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_DBTP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_DBTP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDCAN_DBTP` writer"]
pub struct W(crate::W<FDCAN_DBTP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDCAN_DBTP_SPEC>;
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
impl From<crate::W<FDCAN_DBTP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDCAN_DBTP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSJW` reader - DSJW"]
pub type DSJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSJW` writer - DSJW"]
pub type DSJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_DBTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTSEG2` reader - DTSEG2"]
pub type DTSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEG2` writer - DTSEG2"]
pub type DTSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_DBTP_SPEC, u8, u8, 4, O>;
#[doc = "Field `DTSEG1` reader - DTSEG1"]
pub type DTSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTSEG1` writer - DTSEG1"]
pub type DTSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_DBTP_SPEC, u8, u8, 5, O>;
#[doc = "Field `DBRP` reader - DBRP"]
pub type DBRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBRP` writer - DBRP"]
pub type DBRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FDCAN_DBTP_SPEC, u8, u8, 5, O>;
#[doc = "Field `TDC` reader - TDC"]
pub type TDC_R = crate::BitReader<bool>;
#[doc = "Field `TDC` writer - TDC"]
pub type TDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, FDCAN_DBTP_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&self) -> DSJW_R {
        DSJW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&self) -> DTSEG2_R {
        DTSEG2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    pub fn dtseg1(&self) -> DTSEG1_R {
        DTSEG1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&self) -> DBRP_R {
        DBRP_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    pub fn tdc(&self) -> TDC_R {
        TDC_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - DSJW"]
    #[inline(always)]
    pub fn dsjw(&mut self) -> DSJW_W<0> {
        DSJW_W::new(self)
    }
    #[doc = "Bits 4:7 - DTSEG2"]
    #[inline(always)]
    pub fn dtseg2(&mut self) -> DTSEG2_W<4> {
        DTSEG2_W::new(self)
    }
    #[doc = "Bits 8:12 - DTSEG1"]
    #[inline(always)]
    pub fn dtseg1(&mut self) -> DTSEG1_W<8> {
        DTSEG1_W::new(self)
    }
    #[doc = "Bits 16:20 - DBRP"]
    #[inline(always)]
    pub fn dbrp(&mut self) -> DBRP_W<16> {
        DBRP_W::new(self)
    }
    #[doc = "Bit 23 - TDC"]
    #[inline(always)]
    pub fn tdc(&mut self) -> TDC_W<23> {
        TDC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is dedicated to data bit timing phase and only writable if bits FDCAN_CCCR.CCE and FDCAN_CCCR.INIT are set. The CAN time quantum may be programmed in the range from 1 to 32 FDCAN clock periods. tq = (DBRP + 1) FDCAN clock periods. DTSEG1 is the sum of Prop_Seg and Phase_Seg1. DTSEG2 is Phase_Seg2. Therefore the length of the bit time is (DTSEG1 + DTSEG2 + 3) tq for programmed values, or (Sync_Seg+Prop_Seg+Phase_Seg1+Phase_Seg2) tq for functional values. The information processing time (IPT) is zero, meaning the data for the next bit is available at the first clock edge after the sample point.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_dbtp](index.html) module"]
pub struct FDCAN_DBTP_SPEC;
impl crate::RegisterSpec for FDCAN_DBTP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_dbtp::R](R) reader structure"]
impl crate::Readable for FDCAN_DBTP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdcan_dbtp::W](W) writer structure"]
impl crate::Writable for FDCAN_DBTP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0x0a33"]
impl crate::Resettable for FDCAN_DBTP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0a33
    }
}
