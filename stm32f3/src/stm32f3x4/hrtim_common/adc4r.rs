#[doc = "Register `ADC4R` reader"]
pub struct R(crate::R<ADC4R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC4R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC4R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC4R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC4R` writer"]
pub struct W(crate::W<ADC4R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC4R_SPEC>;
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
impl From<crate::W<ADC4R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC4R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD2MC1` reader - AD2MC1"]
pub type AD2MC1_R = crate::BitReader<AD2MC1_A>;
#[doc = "AD2MC1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2MC1_A {
    #[doc = "0: No generation of ADC trigger on master compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on master compare event"]
    Enabled = 1,
}
impl From<AD2MC1_A> for bool {
    #[inline(always)]
    fn from(variant: AD2MC1_A) -> Self {
        variant as u8 != 0
    }
}
impl AD2MC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2MC1_A {
        match self.bits {
            false => AD2MC1_A::Disabled,
            true => AD2MC1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2MC1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2MC1_A::Enabled
    }
}
#[doc = "Field `AD2MC1` writer - AD2MC1"]
pub type AD2MC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, AD2MC1_A, O>;
impl<'a, const O: u8> AD2MC1_W<'a, O> {
    #[doc = "No generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MC1_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on master compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MC1_A::Enabled)
    }
}
#[doc = "Field `AD2MC2` reader - AD2MC2"]
pub use AD2MC1_R as AD2MC2_R;
#[doc = "Field `AD2MC3` reader - AD2MC3"]
pub use AD2MC1_R as AD2MC3_R;
#[doc = "Field `AD2MC4` reader - AD2MC4"]
pub use AD2MC1_R as AD2MC4_R;
#[doc = "Field `AD2MC2` writer - AD2MC2"]
pub use AD2MC1_W as AD2MC2_W;
#[doc = "Field `AD2MC3` writer - AD2MC3"]
pub use AD2MC1_W as AD2MC3_W;
#[doc = "Field `AD2MC4` writer - AD2MC4"]
pub use AD2MC1_W as AD2MC4_W;
#[doc = "Field `AD2MPER` reader - AD2MPER"]
pub type AD2MPER_R = crate::BitReader<AD2MPER_A>;
#[doc = "AD2MPER\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2MPER_A {
    #[doc = "0: No generation of ADC trigger on timer period event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer period event"]
    Enabled = 1,
}
impl From<AD2MPER_A> for bool {
    #[inline(always)]
    fn from(variant: AD2MPER_A) -> Self {
        variant as u8 != 0
    }
}
impl AD2MPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2MPER_A {
        match self.bits {
            false => AD2MPER_A::Disabled,
            true => AD2MPER_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2MPER_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2MPER_A::Enabled
    }
}
#[doc = "Field `AD2MPER` writer - AD2MPER"]
pub type AD2MPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, AD2MPER_A, O>;
impl<'a, const O: u8> AD2MPER_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2MPER_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer period event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2MPER_A::Enabled)
    }
}
#[doc = "Field `AD2EEV6` reader - AD2EEV6"]
pub type AD2EEV6_R = crate::BitReader<AD2EEV6_A>;
#[doc = "AD2EEV6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2EEV6_A {
    #[doc = "0: No generation of ADC trigger on external event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on external event"]
    Enabled = 1,
}
impl From<AD2EEV6_A> for bool {
    #[inline(always)]
    fn from(variant: AD2EEV6_A) -> Self {
        variant as u8 != 0
    }
}
impl AD2EEV6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2EEV6_A {
        match self.bits {
            false => AD2EEV6_A::Disabled,
            true => AD2EEV6_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2EEV6_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2EEV6_A::Enabled
    }
}
#[doc = "Field `AD2EEV6` writer - AD2EEV6"]
pub type AD2EEV6_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, AD2EEV6_A, O>;
impl<'a, const O: u8> AD2EEV6_W<'a, O> {
    #[doc = "No generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2EEV6_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on external event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2EEV6_A::Enabled)
    }
}
#[doc = "Field `AD2EEV7` reader - AD2EEV7"]
pub use AD2EEV6_R as AD2EEV7_R;
#[doc = "Field `AD2EEV8` reader - AD2EEV8"]
pub use AD2EEV6_R as AD2EEV8_R;
#[doc = "Field `AD2EEV9` reader - AD2EEV9"]
pub use AD2EEV6_R as AD2EEV9_R;
#[doc = "Field `AD2EEV10` reader - AD2EEV10"]
pub use AD2EEV6_R as AD2EEV10_R;
#[doc = "Field `AD2EEV7` writer - AD2EEV7"]
pub use AD2EEV6_W as AD2EEV7_W;
#[doc = "Field `AD2EEV8` writer - AD2EEV8"]
pub use AD2EEV6_W as AD2EEV8_W;
#[doc = "Field `AD2EEV9` writer - AD2EEV9"]
pub use AD2EEV6_W as AD2EEV9_W;
#[doc = "Field `AD2EEV10` writer - AD2EEV10"]
pub use AD2EEV6_W as AD2EEV10_W;
#[doc = "Field `AD2TAC2` reader - AD2TAC2"]
pub type AD2TAC2_R = crate::BitReader<AD2TAC2_A>;
#[doc = "AD2TAC2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2TAC2_A {
    #[doc = "0: No generation of ADC trigger on timer compare event"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer compare event"]
    Enabled = 1,
}
impl From<AD2TAC2_A> for bool {
    #[inline(always)]
    fn from(variant: AD2TAC2_A) -> Self {
        variant as u8 != 0
    }
}
impl AD2TAC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2TAC2_A {
        match self.bits {
            false => AD2TAC2_A::Disabled,
            true => AD2TAC2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TAC2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TAC2_A::Enabled
    }
}
#[doc = "Field `AD2TAC2` writer - AD2TAC2"]
pub type AD2TAC2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, AD2TAC2_A, O>;
impl<'a, const O: u8> AD2TAC2_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TAC2_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer compare event"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TAC2_A::Enabled)
    }
}
#[doc = "Field `AD2TAPER` reader - AD2TAPER"]
pub use AD2MPER_R as AD2TAPER_R;
#[doc = "Field `AD2TBPER` reader - AD2TBPER"]
pub use AD2MPER_R as AD2TBPER_R;
#[doc = "Field `AD2TCPER` reader - AD2TCPER"]
pub use AD2MPER_R as AD2TCPER_R;
#[doc = "Field `AD2TAPER` writer - AD2TAPER"]
pub use AD2MPER_W as AD2TAPER_W;
#[doc = "Field `AD2TBPER` writer - AD2TBPER"]
pub use AD2MPER_W as AD2TBPER_W;
#[doc = "Field `AD2TCPER` writer - AD2TCPER"]
pub use AD2MPER_W as AD2TCPER_W;
#[doc = "Field `AD2TAC3` reader - AD2TAC3"]
pub use AD2TAC2_R as AD2TAC3_R;
#[doc = "Field `AD2TAC4` reader - AD2TAC4"]
pub use AD2TAC2_R as AD2TAC4_R;
#[doc = "Field `AD2TBC2` reader - AD2TBC2"]
pub use AD2TAC2_R as AD2TBC2_R;
#[doc = "Field `AD2TBC3` reader - AD2TBC3"]
pub use AD2TAC2_R as AD2TBC3_R;
#[doc = "Field `AD2TBC4` reader - AD2TBC4"]
pub use AD2TAC2_R as AD2TBC4_R;
#[doc = "Field `AD2TCC2` reader - AD2TCC2"]
pub use AD2TAC2_R as AD2TCC2_R;
#[doc = "Field `AD2TCC3` reader - AD2TCC3"]
pub use AD2TAC2_R as AD2TCC3_R;
#[doc = "Field `AD2TCC4` reader - AD2TCC4"]
pub use AD2TAC2_R as AD2TCC4_R;
#[doc = "Field `AD2TAC3` writer - AD2TAC3"]
pub use AD2TAC2_W as AD2TAC3_W;
#[doc = "Field `AD2TAC4` writer - AD2TAC4"]
pub use AD2TAC2_W as AD2TAC4_W;
#[doc = "Field `AD2TBC2` writer - AD2TBC2"]
pub use AD2TAC2_W as AD2TBC2_W;
#[doc = "Field `AD2TBC3` writer - AD2TBC3"]
pub use AD2TAC2_W as AD2TBC3_W;
#[doc = "Field `AD2TBC4` writer - AD2TBC4"]
pub use AD2TAC2_W as AD2TBC4_W;
#[doc = "Field `AD2TCC2` writer - AD2TCC2"]
pub use AD2TAC2_W as AD2TCC2_W;
#[doc = "Field `AD2TCC3` writer - AD2TCC3"]
pub use AD2TAC2_W as AD2TCC3_W;
#[doc = "Field `AD2TCC4` writer - AD2TCC4"]
pub use AD2TAC2_W as AD2TCC4_W;
#[doc = "Field `AD2TCRST` reader - AD2TCRST"]
pub type AD2TCRST_R = crate::BitReader<AD2TCRST_A>;
#[doc = "AD2TCRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AD2TCRST_A {
    #[doc = "0: No generation of ADC trigger on timer reset and roll-over"]
    Disabled = 0,
    #[doc = "1: Generation of ADC trigger on timer reset and roll-over"]
    Enabled = 1,
}
impl From<AD2TCRST_A> for bool {
    #[inline(always)]
    fn from(variant: AD2TCRST_A) -> Self {
        variant as u8 != 0
    }
}
impl AD2TCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AD2TCRST_A {
        match self.bits {
            false => AD2TCRST_A::Disabled,
            true => AD2TCRST_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AD2TCRST_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AD2TCRST_A::Enabled
    }
}
#[doc = "Field `AD2TCRST` writer - AD2TCRST"]
pub type AD2TCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADC4R_SPEC, AD2TCRST_A, O>;
impl<'a, const O: u8> AD2TCRST_W<'a, O> {
    #[doc = "No generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AD2TCRST_A::Disabled)
    }
    #[doc = "Generation of ADC trigger on timer reset and roll-over"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AD2TCRST_A::Enabled)
    }
}
#[doc = "Field `AD2TDPER` reader - AD2TDPER"]
pub use AD2MPER_R as AD2TDPER_R;
#[doc = "Field `AD2TDPER` writer - AD2TDPER"]
pub use AD2MPER_W as AD2TDPER_W;
#[doc = "Field `AD2TDC2` reader - AD2TDC2"]
pub use AD2TAC2_R as AD2TDC2_R;
#[doc = "Field `AD2TDC3` reader - AD2TDC3"]
pub use AD2TAC2_R as AD2TDC3_R;
#[doc = "Field `AD2TDC4` reader - AD2TDC4"]
pub use AD2TAC2_R as AD2TDC4_R;
#[doc = "Field `AD2TEC2` reader - AD2TEC2"]
pub use AD2TAC2_R as AD2TEC2_R;
#[doc = "Field `AD2TEC3` reader - AD2TEC3"]
pub use AD2TAC2_R as AD2TEC3_R;
#[doc = "Field `AD2TEC4` reader - AD2TEC4"]
pub use AD2TAC2_R as AD2TEC4_R;
#[doc = "Field `AD2TDC2` writer - AD2TDC2"]
pub use AD2TAC2_W as AD2TDC2_W;
#[doc = "Field `AD2TDC3` writer - AD2TDC3"]
pub use AD2TAC2_W as AD2TDC3_W;
#[doc = "Field `AD2TDC4` writer - AD2TDC4"]
pub use AD2TAC2_W as AD2TDC4_W;
#[doc = "Field `AD2TEC2` writer - AD2TEC2"]
pub use AD2TAC2_W as AD2TEC2_W;
#[doc = "Field `AD2TEC3` writer - AD2TEC3"]
pub use AD2TAC2_W as AD2TEC3_W;
#[doc = "Field `AD2TEC4` writer - AD2TEC4"]
pub use AD2TAC2_W as AD2TEC4_W;
#[doc = "Field `AD2TDRST` reader - AD2TDRST"]
pub use AD2TCRST_R as AD2TDRST_R;
#[doc = "Field `AD2TERST` reader - AD2TERST"]
pub use AD2TCRST_R as AD2TERST_R;
#[doc = "Field `AD2TDRST` writer - AD2TDRST"]
pub use AD2TCRST_W as AD2TDRST_W;
#[doc = "Field `AD2TERST` writer - AD2TERST"]
pub use AD2TCRST_W as AD2TERST_W;
impl R {
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    pub fn ad2mc1(&self) -> AD2MC1_R {
        AD2MC1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    pub fn ad2mc2(&self) -> AD2MC2_R {
        AD2MC2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    pub fn ad2mc3(&self) -> AD2MC3_R {
        AD2MC3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    pub fn ad2mc4(&self) -> AD2MC4_R {
        AD2MC4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    pub fn ad2mper(&self) -> AD2MPER_R {
        AD2MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    pub fn ad2eev6(&self) -> AD2EEV6_R {
        AD2EEV6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    pub fn ad2eev7(&self) -> AD2EEV7_R {
        AD2EEV7_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    pub fn ad2eev8(&self) -> AD2EEV8_R {
        AD2EEV8_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    pub fn ad2eev9(&self) -> AD2EEV9_R {
        AD2EEV9_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    pub fn ad2eev10(&self) -> AD2EEV10_R {
        AD2EEV10_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    pub fn ad2tac2(&self) -> AD2TAC2_R {
        AD2TAC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AD2TAC3"]
    #[inline(always)]
    pub fn ad2tac3(&self) -> AD2TAC3_R {
        AD2TAC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    pub fn ad2tac4(&self) -> AD2TAC4_R {
        AD2TAC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    pub fn ad2taper(&self) -> AD2TAPER_R {
        AD2TAPER_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    pub fn ad2tbc2(&self) -> AD2TBC2_R {
        AD2TBC2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AD2TBC3"]
    #[inline(always)]
    pub fn ad2tbc3(&self) -> AD2TBC3_R {
        AD2TBC3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    pub fn ad2tbc4(&self) -> AD2TBC4_R {
        AD2TBC4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    pub fn ad2tbper(&self) -> AD2TBPER_R {
        AD2TBPER_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    pub fn ad2tcc2(&self) -> AD2TCC2_R {
        AD2TCC2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - AD2TCC3"]
    #[inline(always)]
    pub fn ad2tcc3(&self) -> AD2TCC3_R {
        AD2TCC3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    pub fn ad2tcc4(&self) -> AD2TCC4_R {
        AD2TCC4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    pub fn ad2tcper(&self) -> AD2TCPER_R {
        AD2TCPER_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    pub fn ad2tcrst(&self) -> AD2TCRST_R {
        AD2TCRST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    pub fn ad2tdc2(&self) -> AD2TDC2_R {
        AD2TDC2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - AD2TDC3"]
    #[inline(always)]
    pub fn ad2tdc3(&self) -> AD2TDC3_R {
        AD2TDC3_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    pub fn ad2tdc4(&self) -> AD2TDC4_R {
        AD2TDC4_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    pub fn ad2tdper(&self) -> AD2TDPER_R {
        AD2TDPER_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    pub fn ad2tdrst(&self) -> AD2TDRST_R {
        AD2TDRST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    pub fn ad2tec2(&self) -> AD2TEC2_R {
        AD2TEC2_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    pub fn ad2tec3(&self) -> AD2TEC3_R {
        AD2TEC3_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    pub fn ad2tec4(&self) -> AD2TEC4_R {
        AD2TEC4_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    pub fn ad2terst(&self) -> AD2TERST_R {
        AD2TERST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AD2MC1"]
    #[inline(always)]
    pub fn ad2mc1(&mut self) -> AD2MC1_W<0> {
        AD2MC1_W::new(self)
    }
    #[doc = "Bit 1 - AD2MC2"]
    #[inline(always)]
    pub fn ad2mc2(&mut self) -> AD2MC2_W<1> {
        AD2MC2_W::new(self)
    }
    #[doc = "Bit 2 - AD2MC3"]
    #[inline(always)]
    pub fn ad2mc3(&mut self) -> AD2MC3_W<2> {
        AD2MC3_W::new(self)
    }
    #[doc = "Bit 3 - AD2MC4"]
    #[inline(always)]
    pub fn ad2mc4(&mut self) -> AD2MC4_W<3> {
        AD2MC4_W::new(self)
    }
    #[doc = "Bit 4 - AD2MPER"]
    #[inline(always)]
    pub fn ad2mper(&mut self) -> AD2MPER_W<4> {
        AD2MPER_W::new(self)
    }
    #[doc = "Bit 5 - AD2EEV6"]
    #[inline(always)]
    pub fn ad2eev6(&mut self) -> AD2EEV6_W<5> {
        AD2EEV6_W::new(self)
    }
    #[doc = "Bit 6 - AD2EEV7"]
    #[inline(always)]
    pub fn ad2eev7(&mut self) -> AD2EEV7_W<6> {
        AD2EEV7_W::new(self)
    }
    #[doc = "Bit 7 - AD2EEV8"]
    #[inline(always)]
    pub fn ad2eev8(&mut self) -> AD2EEV8_W<7> {
        AD2EEV8_W::new(self)
    }
    #[doc = "Bit 8 - AD2EEV9"]
    #[inline(always)]
    pub fn ad2eev9(&mut self) -> AD2EEV9_W<8> {
        AD2EEV9_W::new(self)
    }
    #[doc = "Bit 9 - AD2EEV10"]
    #[inline(always)]
    pub fn ad2eev10(&mut self) -> AD2EEV10_W<9> {
        AD2EEV10_W::new(self)
    }
    #[doc = "Bit 10 - AD2TAC2"]
    #[inline(always)]
    pub fn ad2tac2(&mut self) -> AD2TAC2_W<10> {
        AD2TAC2_W::new(self)
    }
    #[doc = "Bit 11 - AD2TAC3"]
    #[inline(always)]
    pub fn ad2tac3(&mut self) -> AD2TAC3_W<11> {
        AD2TAC3_W::new(self)
    }
    #[doc = "Bit 12 - AD2TAC4"]
    #[inline(always)]
    pub fn ad2tac4(&mut self) -> AD2TAC4_W<12> {
        AD2TAC4_W::new(self)
    }
    #[doc = "Bit 13 - AD2TAPER"]
    #[inline(always)]
    pub fn ad2taper(&mut self) -> AD2TAPER_W<13> {
        AD2TAPER_W::new(self)
    }
    #[doc = "Bit 14 - AD2TBC2"]
    #[inline(always)]
    pub fn ad2tbc2(&mut self) -> AD2TBC2_W<14> {
        AD2TBC2_W::new(self)
    }
    #[doc = "Bit 15 - AD2TBC3"]
    #[inline(always)]
    pub fn ad2tbc3(&mut self) -> AD2TBC3_W<15> {
        AD2TBC3_W::new(self)
    }
    #[doc = "Bit 16 - AD2TBC4"]
    #[inline(always)]
    pub fn ad2tbc4(&mut self) -> AD2TBC4_W<16> {
        AD2TBC4_W::new(self)
    }
    #[doc = "Bit 17 - AD2TBPER"]
    #[inline(always)]
    pub fn ad2tbper(&mut self) -> AD2TBPER_W<17> {
        AD2TBPER_W::new(self)
    }
    #[doc = "Bit 18 - AD2TCC2"]
    #[inline(always)]
    pub fn ad2tcc2(&mut self) -> AD2TCC2_W<18> {
        AD2TCC2_W::new(self)
    }
    #[doc = "Bit 19 - AD2TCC3"]
    #[inline(always)]
    pub fn ad2tcc3(&mut self) -> AD2TCC3_W<19> {
        AD2TCC3_W::new(self)
    }
    #[doc = "Bit 20 - AD2TCC4"]
    #[inline(always)]
    pub fn ad2tcc4(&mut self) -> AD2TCC4_W<20> {
        AD2TCC4_W::new(self)
    }
    #[doc = "Bit 21 - AD2TCPER"]
    #[inline(always)]
    pub fn ad2tcper(&mut self) -> AD2TCPER_W<21> {
        AD2TCPER_W::new(self)
    }
    #[doc = "Bit 22 - AD2TCRST"]
    #[inline(always)]
    pub fn ad2tcrst(&mut self) -> AD2TCRST_W<22> {
        AD2TCRST_W::new(self)
    }
    #[doc = "Bit 23 - AD2TDC2"]
    #[inline(always)]
    pub fn ad2tdc2(&mut self) -> AD2TDC2_W<23> {
        AD2TDC2_W::new(self)
    }
    #[doc = "Bit 24 - AD2TDC3"]
    #[inline(always)]
    pub fn ad2tdc3(&mut self) -> AD2TDC3_W<24> {
        AD2TDC3_W::new(self)
    }
    #[doc = "Bit 25 - AD2TDC4"]
    #[inline(always)]
    pub fn ad2tdc4(&mut self) -> AD2TDC4_W<25> {
        AD2TDC4_W::new(self)
    }
    #[doc = "Bit 26 - AD2TDPER"]
    #[inline(always)]
    pub fn ad2tdper(&mut self) -> AD2TDPER_W<26> {
        AD2TDPER_W::new(self)
    }
    #[doc = "Bit 27 - AD2TDRST"]
    #[inline(always)]
    pub fn ad2tdrst(&mut self) -> AD2TDRST_W<27> {
        AD2TDRST_W::new(self)
    }
    #[doc = "Bit 28 - AD2TEC2"]
    #[inline(always)]
    pub fn ad2tec2(&mut self) -> AD2TEC2_W<28> {
        AD2TEC2_W::new(self)
    }
    #[doc = "Bit 29 - AD2TEC3"]
    #[inline(always)]
    pub fn ad2tec3(&mut self) -> AD2TEC3_W<29> {
        AD2TEC3_W::new(self)
    }
    #[doc = "Bit 30 - AD2TEC4"]
    #[inline(always)]
    pub fn ad2tec4(&mut self) -> AD2TEC4_W<30> {
        AD2TEC4_W::new(self)
    }
    #[doc = "Bit 31 - AD2TERST"]
    #[inline(always)]
    pub fn ad2terst(&mut self) -> AD2TERST_W<31> {
        AD2TERST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Trigger 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc4r](index.html) module"]
pub struct ADC4R_SPEC;
impl crate::RegisterSpec for ADC4R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc4r::R](R) reader structure"]
impl crate::Readable for ADC4R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc4r::W](W) writer structure"]
impl crate::Writable for ADC4R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC4R to value 0"]
impl crate::Resettable for ADC4R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
