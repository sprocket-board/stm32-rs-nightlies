#[doc = "Register `TAFCR` reader"]
pub struct R(crate::R<TAFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFCR` writer"]
pub struct W(crate::W<TAFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFCR_SPEC>;
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
impl From<crate::W<TAFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub type TAMP1E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub type TAMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input"]
pub type TAMP1TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TAMPIE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TAMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub type TAMP2E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub type TAMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP2_TRG` reader - Active level for RTC_TAMP2 input"]
pub type TAMP2_TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP2_TRG` writer - Active level for RTC_TAMP2 input"]
pub type TAMP2_TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub type TAMP3E_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub type TAMP3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_R = crate::BitReader<bool>;
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input"]
pub type TAMP3TRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event"]
pub type TAMPTS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event"]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency"]
pub type TAMPFREQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency"]
pub type TAMPFREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count"]
pub type TAMPFLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count"]
pub type TAMPFLT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMP_PRCH` reader - RTC_TAMPx precharge duration"]
pub type TAMP_PRCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TAMP_PRCH` writer - RTC_TAMPx precharge duration"]
pub type TAMP_PRCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `TAMP_PUDIS` reader - RTC_TAMPx pull-up disable"]
pub type TAMP_PUDIS_R = crate::BitReader<bool>;
#[doc = "Field `TAMP_PUDIS` writer - RTC_TAMPx pull-up disable"]
pub type TAMP_PUDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC13VALUE` reader - RTC_ALARM output type/PC13 value"]
pub type PC13VALUE_R = crate::BitReader<bool>;
#[doc = "Field `PC13VALUE` writer - RTC_ALARM output type/PC13 value"]
pub type PC13VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC13MODE` reader - PC13 mode"]
pub type PC13MODE_R = crate::BitReader<bool>;
#[doc = "Field `PC13MODE` writer - PC13 mode"]
pub type PC13MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC14VALUE` reader - PC14 value"]
pub type PC14VALUE_R = crate::BitReader<bool>;
#[doc = "Field `PC14VALUE` writer - PC14 value"]
pub type PC14VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC14MODE` reader - PC14 mode"]
pub type PC14MODE_R = crate::BitReader<bool>;
#[doc = "Field `PC14MODE` writer - PC14 mode"]
pub type PC14MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC15VALUE` reader - PC15 value"]
pub type PC15VALUE_R = crate::BitReader<bool>;
#[doc = "Field `PC15VALUE` writer - PC15 value"]
pub type PC15VALUE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
#[doc = "Field `PC15MODE` reader - PC15 mode"]
pub type PC15MODE_R = crate::BitReader<bool>;
#[doc = "Field `PC15MODE` writer - PC15 mode"]
pub type PC15MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAFCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2_trg(&self) -> TAMP2_TRG_R {
        TAMP2_TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tamp_prch(&self) -> TAMP_PRCH_R {
        TAMP_PRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamp_pudis(&self) -> TAMP_PUDIS_R {
        TAMP_PUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13value(&self) -> PC13VALUE_R {
        PC13VALUE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&self) -> PC13MODE_R {
        PC13MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&self) -> PC14VALUE_R {
        PC14VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mode(&self) -> PC14MODE_R {
        PC14MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&self) -> PC15VALUE_R {
        PC15VALUE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&self) -> PC15MODE_R {
        PC15MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<0> {
        TAMP1E_W::new(self)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<1> {
        TAMP1TRG_W::new(self)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<2> {
        TAMPIE_W::new(self)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<3> {
        TAMP2E_W::new(self)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input"]
    #[inline(always)]
    pub fn tamp2_trg(&mut self) -> TAMP2_TRG_W<4> {
        TAMP2_TRG_W::new(self)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<5> {
        TAMP3E_W::new(self)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<6> {
        TAMP3TRG_W::new(self)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event"]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<7> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency"]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<8> {
        TAMPFREQ_W::new(self)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count"]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<11> {
        TAMPFLT_W::new(self)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration"]
    #[inline(always)]
    pub fn tamp_prch(&mut self) -> TAMP_PRCH_W<13> {
        TAMP_PRCH_W::new(self)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable"]
    #[inline(always)]
    pub fn tamp_pudis(&mut self) -> TAMP_PUDIS_W<15> {
        TAMP_PUDIS_W::new(self)
    }
    #[doc = "Bit 18 - RTC_ALARM output type/PC13 value"]
    #[inline(always)]
    pub fn pc13value(&mut self) -> PC13VALUE_W<18> {
        PC13VALUE_W::new(self)
    }
    #[doc = "Bit 19 - PC13 mode"]
    #[inline(always)]
    pub fn pc13mode(&mut self) -> PC13MODE_W<19> {
        PC13MODE_W::new(self)
    }
    #[doc = "Bit 20 - PC14 value"]
    #[inline(always)]
    pub fn pc14value(&mut self) -> PC14VALUE_W<20> {
        PC14VALUE_W::new(self)
    }
    #[doc = "Bit 21 - PC14 mode"]
    #[inline(always)]
    pub fn pc14mode(&mut self) -> PC14MODE_W<21> {
        PC14MODE_W::new(self)
    }
    #[doc = "Bit 22 - PC15 value"]
    #[inline(always)]
    pub fn pc15value(&mut self) -> PC15VALUE_W<22> {
        PC15VALUE_W::new(self)
    }
    #[doc = "Bit 23 - PC15 mode"]
    #[inline(always)]
    pub fn pc15mode(&mut self) -> PC15MODE_W<23> {
        PC15MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "tamper and alternate function configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafcr](index.html) module"]
pub struct TAFCR_SPEC;
impl crate::RegisterSpec for TAFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafcr::R](R) reader structure"]
impl crate::Readable for TAFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafcr::W](W) writer structure"]
impl crate::Writable for TAFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAFCR to value 0"]
impl crate::Resettable for TAFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
