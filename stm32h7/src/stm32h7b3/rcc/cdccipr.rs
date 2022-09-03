#[doc = "Register `CDCCIPR` reader"]
pub struct R(crate::R<CDCCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCCIPR` writer"]
pub struct W(crate::W<CDCCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCCIPR_SPEC>;
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
impl From<crate::W<CDCCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMCSEL` reader - FMC kernel clock source selection"]
pub type FMCSEL_R = crate::FieldReader<u8, FMCSEL_A>;
#[doc = "FMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FMCSEL_A {
    #[doc = "0: rcc_hclk3 selected as peripheral clock"]
    RccHclk3 = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    Pll1Q = 1,
    #[doc = "2: pll2_r selected as peripheral clock"]
    Pll2R = 2,
    #[doc = "3: PER selected as peripheral clock"]
    Per = 3,
}
impl From<FMCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMCSEL_A) -> Self {
        variant as _
    }
}
impl FMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCSEL_A {
        match self.bits {
            0 => FMCSEL_A::RccHclk3,
            1 => FMCSEL_A::Pll1Q,
            2 => FMCSEL_A::Pll2R,
            3 => FMCSEL_A::Per,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RccHclk3`"]
    #[inline(always)]
    pub fn is_rcc_hclk3(&self) -> bool {
        *self == FMCSEL_A::RccHclk3
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FMCSEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == FMCSEL_A::Pll2R
    }
    #[doc = "Checks if the value of the field is `Per`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == FMCSEL_A::Per
    }
}
#[doc = "Field `FMCSEL` writer - FMC kernel clock source selection"]
pub type FMCSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDCCIPR_SPEC, u8, FMCSEL_A, 2, O>;
impl<'a, const O: u8> FMCSEL_W<'a, O> {
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut W {
        self.variant(FMCSEL_A::RccHclk3)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FMCSEL_A::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(FMCSEL_A::Pll2R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(FMCSEL_A::Per)
    }
}
#[doc = "Field `OCTOSPISEL` reader - OCTOSPI kernel clock source selection"]
pub use FMCSEL_R as OCTOSPISEL_R;
#[doc = "Field `OCTOSPISEL` writer - OCTOSPI kernel clock source selection"]
pub use FMCSEL_W as OCTOSPISEL_W;
#[doc = "Field `SDMMCSEL` reader - SDMMC kernel clock source selection"]
pub type SDMMCSEL_R = crate::BitReader<SDMMCSEL_A>;
#[doc = "SDMMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMCSEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    Pll1Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    Pll2R = 1,
}
impl From<SDMMCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SDMMCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMCSEL_A {
        match self.bits {
            false => SDMMCSEL_A::Pll1Q,
            true => SDMMCSEL_A::Pll2R,
        }
    }
    #[doc = "Checks if the value of the field is `Pll1Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SDMMCSEL_A::Pll1Q
    }
    #[doc = "Checks if the value of the field is `Pll2R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SDMMCSEL_A::Pll2R
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC kernel clock source selection"]
pub type SDMMCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CDCCIPR_SPEC, SDMMCSEL_A, O>;
impl<'a, const O: u8> SDMMCSEL_W<'a, O> {
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::Pll1Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::Pll2R)
    }
}
#[doc = "Field `CKPERSEL` reader - per_ck clock source selection"]
pub type CKPERSEL_R = crate::FieldReader<u8, CKPERSEL_A>;
#[doc = "per_ck clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKPERSEL_A {
    #[doc = "0: HSI selected as peripheral clock"]
    Hsi = 0,
    #[doc = "1: CSI selected as peripheral clock"]
    Csi = 1,
    #[doc = "2: HSE selected as peripheral clock"]
    Hse = 2,
}
impl From<CKPERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSEL_A) -> Self {
        variant as _
    }
}
impl CKPERSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKPERSEL_A> {
        match self.bits {
            0 => Some(CKPERSEL_A::Hsi),
            1 => Some(CKPERSEL_A::Csi),
            2 => Some(CKPERSEL_A::Hse),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Hsi`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == CKPERSEL_A::Hsi
    }
    #[doc = "Checks if the value of the field is `Csi`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CKPERSEL_A::Csi
    }
    #[doc = "Checks if the value of the field is `Hse`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == CKPERSEL_A::Hse
    }
}
#[doc = "Field `CKPERSEL` writer - per_ck clock source selection"]
pub type CKPERSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCCIPR_SPEC, u8, CKPERSEL_A, 2, O>;
impl<'a, const O: u8> CKPERSEL_W<'a, O> {
    #[doc = "HSI selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(CKPERSEL_A::Hsi)
    }
    #[doc = "CSI selected as peripheral clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(CKPERSEL_A::Csi)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(CKPERSEL_A::Hse)
    }
}
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - OCTOSPI kernel clock source selection"]
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&mut self) -> FMCSEL_W<0> {
        FMCSEL_W::new(self)
    }
    #[doc = "Bits 4:5 - OCTOSPI kernel clock source selection"]
    #[inline(always)]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W<4> {
        OCTOSPISEL_W::new(self)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W<16> {
        SDMMCSEL_W::new(self)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&mut self) -> CKPERSEL_W<28> {
        CKPERSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC CPU domain kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdccipr](index.html) module"]
pub struct CDCCIPR_SPEC;
impl crate::RegisterSpec for CDCCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdccipr::R](R) reader structure"]
impl crate::Readable for CDCCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdccipr::W](W) writer structure"]
impl crate::Writable for CDCCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDCCIPR to value 0"]
impl crate::Resettable for CDCCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
