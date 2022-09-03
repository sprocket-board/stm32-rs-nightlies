#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUCKSEL` reader - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value"]
pub type WUCKSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WUCKSEL` writer - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value"]
pub type WUCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_R = crate::BitReader<bool>;
#[doc = "Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF."]
pub type REFCKON_R = crate::BitReader<bool>;
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF."]
pub type REFCKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_R = crate::BitReader<bool>;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BYPSHAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader<bool>;
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader<bool>;
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader<bool>;
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WUTE` reader - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WUTE_R = crate::BitReader<bool>;
#[doc = "Field `WUTE` writer - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
pub type WUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TSE_R = crate::BitReader<bool>;
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type ALRBIE_R = crate::BitReader<bool>;
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type ALRBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WUTIE_R = crate::BitReader<bool>;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TSIE` reader - Timestamp interrupt enable"]
pub type TSIE_R = crate::BitReader<bool>;
#[doc = "Field `TSIE` writer - Timestamp interrupt enable"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
pub type ADD1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
pub type SUB1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_R = crate::BitReader<bool>;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type COSEL_R = crate::BitReader<bool>;
#[doc = "Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
pub type COSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_R = crate::BitReader<bool>;
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
pub type OSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `COE` reader - Calibration output enable This bit enables the CALIB output"]
pub type COE_R = crate::BitReader<bool>;
#[doc = "Field `COE` writer - Calibration output enable This bit enables the CALIB output"]
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ITSE_R = crate::BitReader<bool>;
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ITSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TAMPTS_R = crate::BitReader<bool>;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
pub type TAMPTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPOE` reader - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPOE` writer - Tamper detection output enable on TAMPALRM"]
pub type TAMPOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_R = crate::BitReader<bool>;
#[doc = "Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable"]
pub type TAMPALRM_PU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `TAMPALRM_TYPE` reader - TAMPALRM output type"]
pub type TAMPALRM_TYPE_R = crate::BitReader<bool>;
#[doc = "Field `TAMPALRM_TYPE` writer - TAMPALRM output type"]
pub type TAMPALRM_TYPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OUT2EN` reader - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â\u{89} 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â\u{89} 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ\u{89} 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_R = crate::BitReader<bool>;
#[doc = "Field `OUT2EN` writer - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â\u{89} 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â\u{89} 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ\u{89} 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
pub type OUT2EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&self) -> TAMPOE_R {
        TAMPOE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â\u{89} 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â\u{89} 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ\u{89} 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - ck_wut wakeup clock selection 10x: ck_spre (usually 1Â Hz) clock is selected 11x: ck_spre (usually 1Â Hz) clock is selected and 216Â is added to the WUT counter value"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WUCKSEL_W<0> {
        WUCKSEL_W::new(self)
    }
    #[doc = "Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<3> {
        TSEDGE_W::new(self)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Â Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<4> {
        REFCKON_W::new(self)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<5> {
        BYPSHAD_W::new(self)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<6> {
        FMT_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<8> {
        ALRAE_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> ALRBE_W<9> {
        ALRBE_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer enable Note: When the wakeup timer is disabled, wait for WUTWF=1 before enabling it again."]
    #[inline(always)]
    pub fn wute(&mut self) -> WUTE_W<10> {
        WUTE_W::new(self)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<11> {
        TSE_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<12> {
        ALRAIE_W::new(self)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> ALRBIE_W<13> {
        ALRBIE_W::new(self)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WUTIE_W<14> {
        WUTIE_W::new(self)
    }
    #[doc = "Bit 15 - Timestamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<15> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<16> {
        ADD1H_W::new(self)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<17> {
        SUB1H_W::new(self)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<18> {
        BKP_W::new(self)
    }
    #[doc = "Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768Â kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to ."]
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<19> {
        COSEL_W::new(self)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output."]
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<20> {
        POL_W::new(self)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output."]
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<21> {
        OSEL_W::new(self)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the CALIB output"]
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<23> {
        COE_W::new(self)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&mut self) -> ITSE_W<24> {
        ITSE_W::new(self)
    }
    #[doc = "Bit 25 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set after the tamper flags, therefore if TAMPTS and TSIE are set, it is recommended to disable the tamper interrupts in order to avoid servicing 2 interrupts."]
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<25> {
        TAMPTS_W::new(self)
    }
    #[doc = "Bit 26 - Tamper detection output enable on TAMPALRM"]
    #[inline(always)]
    pub fn tampoe(&mut self) -> TAMPOE_W<26> {
        TAMPOE_W::new(self)
    }
    #[doc = "Bit 29 - TAMPALRM pull-up enable"]
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<29> {
        TAMPALRM_PU_W::new(self)
    }
    #[doc = "Bit 30 - TAMPALRM output type"]
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<30> {
        TAMPALRM_TYPE_W::new(self)
    }
    #[doc = "Bit 31 - RTC_OUT2 output enable Setting this bit allows to remap the RTC outputs on RTC_OUT2 as follows: OUT2EN = 0: RTC output 2 disable If OSEL â\u{89} 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1 OUT2EN = 1: RTC output 2 enable If (OSEL â\u{89} 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2 If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2 If (OSELâ\u{89} 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and TAMPALRM is output on RTC_OUT1."]
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W<31> {
        OUT2EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
