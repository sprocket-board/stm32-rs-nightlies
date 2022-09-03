#[doc = "Register `PLLI2SCFGR` reader"]
pub struct R(crate::R<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLLI2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLLI2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLLI2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLLI2SCFGR` writer"]
pub struct W(crate::W<PLLI2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLLI2SCFGR_SPEC>;
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
impl From<crate::W<PLLI2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLLI2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLLI2SM` reader - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLI2SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLI2SM` writer - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
pub type PLLI2SM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u8, u8, 6, O>;
#[doc = "Field `PLLI2SN` reader - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PLLI2SN` writer - PLLI2S multiplication factor for VCO"]
pub type PLLI2SN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u16, u16, 9, O>;
#[doc = "Field `PLLI2SSRC` reader - PLLI2S entry clock source"]
pub type PLLI2SSRC_R = crate::BitReader<PLLI2SSRC_A>;
#[doc = "PLLI2S entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLI2SSRC_A {
    #[doc = "0: HSE or HSI depending on PLLSRC of PLLCFGR"]
    HseHsi = 0,
    #[doc = "1: External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    External = 1,
}
impl From<PLLI2SSRC_A> for bool {
    #[inline(always)]
    fn from(variant: PLLI2SSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLI2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLI2SSRC_A {
        match self.bits {
            false => PLLI2SSRC_A::HseHsi,
            true => PLLI2SSRC_A::External,
        }
    }
    #[doc = "Checks if the value of the field is `HseHsi`"]
    #[inline(always)]
    pub fn is_hse_hsi(&self) -> bool {
        *self == PLLI2SSRC_A::HseHsi
    }
    #[doc = "Checks if the value of the field is `External`"]
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == PLLI2SSRC_A::External
    }
}
#[doc = "Field `PLLI2SSRC` writer - PLLI2S entry clock source"]
pub type PLLI2SSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLLI2SCFGR_SPEC, PLLI2SSRC_A, O>;
impl<'a, const O: u8> PLLI2SSRC_W<'a, O> {
    #[doc = "HSE or HSI depending on PLLSRC of PLLCFGR"]
    #[inline(always)]
    pub fn hse_hsi(self) -> &'a mut W {
        self.variant(PLLI2SSRC_A::HseHsi)
    }
    #[doc = "External AFI clock (CK_PLLI2S_EXT) selected as PLL clock entry"]
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(PLLI2SSRC_A::External)
    }
}
#[doc = "Field `PLLI2SQ` reader - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
pub type PLLI2SQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLI2SQ` writer - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
pub type PLLI2SQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u8, u8, 4, O>;
#[doc = "Field `PLLI2SR` reader - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLI2SR` writer - PLLI2S division factor for I2S clocks"]
pub type PLLI2SR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PLLI2SCFGR_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn plli2sm(&self) -> PLLI2SM_R {
        PLLI2SM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&self) -> PLLI2SN_R {
        PLLI2SN_R::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    pub fn plli2ssrc(&self) -> PLLI2SSRC_R {
        PLLI2SSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    pub fn plli2sq(&self) -> PLLI2SQ_R {
        PLLI2SQ_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&self) -> PLLI2SR_R {
        PLLI2SR_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Division factor for the main PLL (PLL) and audio PLL (PLLI2S) input clock"]
    #[inline(always)]
    pub fn plli2sm(&mut self) -> PLLI2SM_W<0> {
        PLLI2SM_W::new(self)
    }
    #[doc = "Bits 6:14 - PLLI2S multiplication factor for VCO"]
    #[inline(always)]
    pub fn plli2sn(&mut self) -> PLLI2SN_W<6> {
        PLLI2SN_W::new(self)
    }
    #[doc = "Bit 22 - PLLI2S entry clock source"]
    #[inline(always)]
    pub fn plli2ssrc(&mut self) -> PLLI2SSRC_W<22> {
        PLLI2SSRC_W::new(self)
    }
    #[doc = "Bits 24:27 - PLLI2S division factor for USB OTG FS/SDIO/RNG clock"]
    #[inline(always)]
    pub fn plli2sq(&mut self) -> PLLI2SQ_W<24> {
        PLLI2SQ_W::new(self)
    }
    #[doc = "Bits 28:30 - PLLI2S division factor for I2S clocks"]
    #[inline(always)]
    pub fn plli2sr(&mut self) -> PLLI2SR_W<28> {
        PLLI2SR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLI2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plli2scfgr](index.html) module"]
pub struct PLLI2SCFGR_SPEC;
impl crate::RegisterSpec for PLLI2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [plli2scfgr::R](R) reader structure"]
impl crate::Readable for PLLI2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [plli2scfgr::W](W) writer structure"]
impl crate::Writable for PLLI2SCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PLLI2SCFGR to value 0x2000_3000"]
impl crate::Resettable for PLLI2SCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000_3000
    }
}
