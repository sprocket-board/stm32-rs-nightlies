#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD` reader - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
pub type RELOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELOAD` writer - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u16, u16, 16, O>;
#[doc = "Field `FELIM` reader - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
pub type FELIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FELIM` writer - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
pub type FELIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYNCDIV` reader - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCDIV` writer - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
pub type SYNCDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SYNCSRC` reader - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
pub type SYNCSRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCSRC` writer - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
pub type SYNCSRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFGR_SPEC, u8, u8, 2, O>;
#[doc = "Field `SYNCPOL` reader - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_R = crate::BitReader<bool>;
#[doc = "Field `SYNCPOL` writer - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
pub type SYNCPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn felim(&self) -> FELIM_R {
        FELIM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn syncdiv(&self) -> SYNCDIV_R {
        SYNCDIV_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
    #[inline(always)]
    pub fn syncsrc(&self) -> SYNCSRC_R {
        SYNCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn syncpol(&self) -> SYNCPOL_R {
        SYNCPOL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter reload value RELOAD is the value to be loaded in the frequency error counter with each SYNC event. Refer to Section7.3.3: Frequency error measurement for more details about counter behavior."]
    #[inline(always)]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Bits 16:23 - Frequency error limit FELIM contains the value to be used to evaluate the captured frequency error value latched in the FECAP\\[15:0\\]
bits of the CRS_ISR register. Refer to Section7.3.4: Frequency error evaluation and automatic trimming for more details about FECAP evaluation."]
    #[inline(always)]
    pub fn felim(&mut self) -> FELIM_W<16> {
        FELIM_W::new(self)
    }
    #[doc = "Bits 24:26 - SYNC divider These bits are set and cleared by software to control the division factor of the SYNC signal."]
    #[inline(always)]
    pub fn syncdiv(&mut self) -> SYNCDIV_W<24> {
        SYNCDIV_W::new(self)
    }
    #[doc = "Bits 28:29 - SYNC signal source selection These bits are set and cleared by software to select the SYNC signal source. Note: When using USB LPM (Link Power Management) and the device is in Sleep mode, the periodic USB SOF will not be generated by the host. No SYNC signal will therefore be provided to the CRS to calibrate the HSI48 on the run. To guarantee the required clock precision after waking up from Sleep mode, the LSE or reference clock on the GPIOs should be used as SYNC signal."]
    #[inline(always)]
    pub fn syncsrc(&mut self) -> SYNCSRC_W<28> {
        SYNCSRC_W::new(self)
    }
    #[doc = "Bit 31 - SYNC polarity selection This bit is set and cleared by software to select the input polarity for the SYNC signal source."]
    #[inline(always)]
    pub fn syncpol(&mut self) -> SYNCPOL_W<31> {
        SYNCPOL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be written only when the frequency error counter is disabled (CEN bit is cleared in CRS_CR). When the counter is enabled, this register is write-protected.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0x2022_bb7f"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_bb7f
    }
}
