#[doc = "Register `CLRFR` writer"]
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COVRUDRW_AW {
    #[doc = "1: Clears the OVRUDR flag"]
    Clear = 1,
}
impl From<COVRUDRW_AW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, COVRUDRW_AW, O>;
impl<'a, const O: u8> COVRUDR_W<'a, O> {
    #[doc = "Clears the OVRUDR flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDRW_AW::Clear)
    }
}
#[doc = "Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMUTEDETW_AW {
    #[doc = "1: Clears the MUTEDET flag"]
    Clear = 1,
}
impl From<CMUTEDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMUTEDET` writer - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CMUTEDETW_AW, O>;
impl<'a, const O: u8> CMUTEDET_W<'a, O> {
    #[doc = "Clears the MUTEDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDETW_AW::Clear)
    }
}
#[doc = "Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CWCKCFGW_AW {
    #[doc = "1: Clears the WCKCFG flag"]
    Clear = 1,
}
impl From<CWCKCFGW_AW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CWCKCFGW_AW, O>;
impl<'a, const O: u8> CWCKCFG_W<'a, O> {
    #[doc = "Clears the WCKCFG flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFGW_AW::Clear)
    }
}
#[doc = "Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCNRDYW_AW {
    #[doc = "1: Clears the CNRDY flag"]
    Clear = 1,
}
impl From<CCNRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCNRDY` writer - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CCNRDYW_AW, O>;
impl<'a, const O: u8> CCNRDY_W<'a, O> {
    #[doc = "Clears the CNRDY flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDYW_AW::Clear)
    }
}
#[doc = "Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAFSDETW_AW {
    #[doc = "1: Clears the AFSDET flag"]
    Clear = 1,
}
impl From<CAFSDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CAFSDETW_AW, O>;
impl<'a, const O: u8> CAFSDET_W<'a, O> {
    #[doc = "Clears the AFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDETW_AW::Clear)
    }
}
#[doc = "Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLFSDETW_AW {
    #[doc = "1: Clears the LFSDET flag"]
    Clear = 1,
}
impl From<CLFSDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CLFSDETW_AW, O>;
impl<'a, const O: u8> CLFSDET_W<'a, O> {
    #[doc = "Clears the LFSDET flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDETW_AW::Clear)
    }
}
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    #[doc = "Bit 1 - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\]
= 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    #[doc = "Bit 4 - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear flag register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clrfr](index.html) module"]
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [clrfr::W](W) writer structure"]
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLRFR to value 0"]
impl crate::Resettable for CLRFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
