#[doc = "Register `OUTBR` reader"]
pub struct R(crate::R<OUTBR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUTBR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUTBR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUTBR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUTBR` writer"]
pub struct W(crate::W<OUTBR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUTBR_SPEC>;
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
impl From<crate::W<OUTBR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUTBR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL1` reader - Output 1 polarity"]
pub type POL1_R = crate::BitReader<bool>;
#[doc = "Field `POL1` writer - Output 1 polarity"]
pub type POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `IDLEM1` reader - Output 1 Idle mode"]
pub type IDLEM1_R = crate::BitReader<bool>;
#[doc = "Field `IDLEM1` writer - Output 1 Idle mode"]
pub type IDLEM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `IDLES1` reader - Output 1 Idle State"]
pub type IDLES1_R = crate::BitReader<bool>;
#[doc = "Field `IDLES1` writer - Output 1 Idle State"]
pub type IDLES1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `FAULT1` reader - Output 1 Fault state"]
pub type FAULT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULT1` writer - Output 1 Fault state"]
pub type FAULT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTBR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHP1` reader - Output 1 Chopper enable"]
pub type CHP1_R = crate::BitReader<bool>;
#[doc = "Field `CHP1` writer - Output 1 Chopper enable"]
pub type CHP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `DIDL1` reader - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_R = crate::BitReader<bool>;
#[doc = "Field `DIDL1` writer - Output 1 Deadtime upon burst mode Idle entry"]
pub type DIDL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `DTEN` reader - Deadtime enable"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - Deadtime enable"]
pub type DTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `DLYPRTEN` reader - Delayed Protection Enable"]
pub type DLYPRTEN_R = crate::BitReader<bool>;
#[doc = "Field `DLYPRTEN` writer - Delayed Protection Enable"]
pub type DLYPRTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `DLYPRT` reader - Delayed Protection"]
pub type DLYPRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLYPRT` writer - Delayed Protection"]
pub type DLYPRT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTBR_SPEC, u8, u8, 3, O>;
#[doc = "Field `BIAR` reader - Balanced Idle Automatic Resume"]
pub type BIAR_R = crate::BitReader<bool>;
#[doc = "Field `BIAR` writer - Balanced Idle Automatic Resume"]
pub type BIAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `POL2` reader - Output 2 polarity"]
pub type POL2_R = crate::BitReader<bool>;
#[doc = "Field `POL2` writer - Output 2 polarity"]
pub type POL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `IDLEM2` reader - Output 2 Idle mode"]
pub type IDLEM2_R = crate::BitReader<bool>;
#[doc = "Field `IDLEM2` writer - Output 2 Idle mode"]
pub type IDLEM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `IDLES2` reader - Output 2 Idle State"]
pub type IDLES2_R = crate::BitReader<bool>;
#[doc = "Field `IDLES2` writer - Output 2 Idle State"]
pub type IDLES2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `FAULT2` reader - Output 2 Fault state"]
pub type FAULT2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FAULT2` writer - Output 2 Fault state"]
pub type FAULT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OUTBR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CHP2` reader - Output 2 Chopper enable"]
pub type CHP2_R = crate::BitReader<bool>;
#[doc = "Field `CHP2` writer - Output 2 Chopper enable"]
pub type CHP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
#[doc = "Field `DIDL2` reader - Output 2 Deadtime upon burst mode Idle entry"]
pub type DIDL2_R = crate::BitReader<bool>;
#[doc = "Field `DIDL2` writer - Output 2 Deadtime upon burst mode Idle entry"]
pub type DIDL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, OUTBR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&self) -> IDLEM1_R {
        IDLEM1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&self) -> IDLES1_R {
        IDLES1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&self) -> FAULT1_R {
        FAULT1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&self) -> CHP1_R {
        CHP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&self) -> DIDL1_R {
        DIDL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&self) -> DLYPRTEN_R {
        DLYPRTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    pub fn biar(&self) -> BIAR_R {
        BIAR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&self) -> IDLEM2_R {
        IDLEM2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&self) -> IDLES2_R {
        IDLES2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&self) -> FAULT2_R {
        FAULT2_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&self) -> CHP2_R {
        CHP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&self) -> DIDL2_R {
        DIDL2_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Output 1 polarity"]
    #[inline(always)]
    pub fn pol1(&mut self) -> POL1_W<1> {
        POL1_W::new(self)
    }
    #[doc = "Bit 2 - Output 1 Idle mode"]
    #[inline(always)]
    pub fn idlem1(&mut self) -> IDLEM1_W<2> {
        IDLEM1_W::new(self)
    }
    #[doc = "Bit 3 - Output 1 Idle State"]
    #[inline(always)]
    pub fn idles1(&mut self) -> IDLES1_W<3> {
        IDLES1_W::new(self)
    }
    #[doc = "Bits 4:5 - Output 1 Fault state"]
    #[inline(always)]
    pub fn fault1(&mut self) -> FAULT1_W<4> {
        FAULT1_W::new(self)
    }
    #[doc = "Bit 6 - Output 1 Chopper enable"]
    #[inline(always)]
    pub fn chp1(&mut self) -> CHP1_W<6> {
        CHP1_W::new(self)
    }
    #[doc = "Bit 7 - Output 1 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl1(&mut self) -> DIDL1_W<7> {
        DIDL1_W::new(self)
    }
    #[doc = "Bit 8 - Deadtime enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W<8> {
        DTEN_W::new(self)
    }
    #[doc = "Bit 9 - Delayed Protection Enable"]
    #[inline(always)]
    pub fn dlyprten(&mut self) -> DLYPRTEN_W<9> {
        DLYPRTEN_W::new(self)
    }
    #[doc = "Bits 10:12 - Delayed Protection"]
    #[inline(always)]
    pub fn dlyprt(&mut self) -> DLYPRT_W<10> {
        DLYPRT_W::new(self)
    }
    #[doc = "Bit 14 - Balanced Idle Automatic Resume"]
    #[inline(always)]
    pub fn biar(&mut self) -> BIAR_W<14> {
        BIAR_W::new(self)
    }
    #[doc = "Bit 17 - Output 2 polarity"]
    #[inline(always)]
    pub fn pol2(&mut self) -> POL2_W<17> {
        POL2_W::new(self)
    }
    #[doc = "Bit 18 - Output 2 Idle mode"]
    #[inline(always)]
    pub fn idlem2(&mut self) -> IDLEM2_W<18> {
        IDLEM2_W::new(self)
    }
    #[doc = "Bit 19 - Output 2 Idle State"]
    #[inline(always)]
    pub fn idles2(&mut self) -> IDLES2_W<19> {
        IDLES2_W::new(self)
    }
    #[doc = "Bits 20:21 - Output 2 Fault state"]
    #[inline(always)]
    pub fn fault2(&mut self) -> FAULT2_W<20> {
        FAULT2_W::new(self)
    }
    #[doc = "Bit 22 - Output 2 Chopper enable"]
    #[inline(always)]
    pub fn chp2(&mut self) -> CHP2_W<22> {
        CHP2_W::new(self)
    }
    #[doc = "Bit 23 - Output 2 Deadtime upon burst mode Idle entry"]
    #[inline(always)]
    pub fn didl2(&mut self) -> DIDL2_W<23> {
        DIDL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outbr](index.html) module"]
pub struct OUTBR_SPEC;
impl crate::RegisterSpec for OUTBR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [outbr::R](R) reader structure"]
impl crate::Readable for OUTBR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [outbr::W](W) writer structure"]
impl crate::Writable for OUTBR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUTBR to value 0"]
impl crate::Resettable for OUTBR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
