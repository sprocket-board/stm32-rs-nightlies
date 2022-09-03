#[doc = "Register `PCSEL` reader"]
pub struct R(crate::R<PCSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSEL` writer"]
pub struct W(crate::W<PCSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSEL_SPEC>;
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
impl From<crate::W<PCSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PCSEL` reader - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_R = crate::FieldReader<u32, PCSEL_A>;
#[doc = "Channel x (VINP\\[i\\]) pre selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum PCSEL_A {
    #[doc = "0: Input channel x is not pre-selected"]
    NotPreselected = 0,
    #[doc = "1: Pre-select input channel x"]
    Preselected = 1,
}
impl From<PCSEL_A> for u32 {
    #[inline(always)]
    fn from(variant: PCSEL_A) -> Self {
        variant as _
    }
}
impl PCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCSEL_A> {
        match self.bits {
            0 => Some(PCSEL_A::NotPreselected),
            1 => Some(PCSEL_A::Preselected),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NotPreselected`"]
    #[inline(always)]
    pub fn is_not_preselected(&self) -> bool {
        *self == PCSEL_A::NotPreselected
    }
    #[doc = "Checks if the value of the field is `Preselected`"]
    #[inline(always)]
    pub fn is_preselected(&self) -> bool {
        *self == PCSEL_A::Preselected
    }
}
#[doc = "Field `PCSEL` writer - Channel x (VINP\\[i\\]) pre selection"]
pub type PCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCSEL_SPEC, u32, PCSEL_A, 20, O>;
impl<'a, const O: u8> PCSEL_W<'a, O> {
    #[doc = "Input channel x is not pre-selected"]
    #[inline(always)]
    pub fn not_preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::NotPreselected)
    }
    #[doc = "Pre-select input channel x"]
    #[inline(always)]
    pub fn preselected(self) -> &'a mut W {
        self.variant(PCSEL_A::Preselected)
    }
}
impl R {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&self) -> PCSEL_R {
        PCSEL_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - Channel x (VINP\\[i\\]) pre selection"]
    #[inline(always)]
    pub fn pcsel(&mut self) -> PCSEL_W<0> {
        PCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC pre channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel](index.html) module"]
pub struct PCSEL_SPEC;
impl crate::RegisterSpec for PCSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcsel::R](R) reader structure"]
impl crate::Readable for PCSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsel::W](W) writer structure"]
impl crate::Writable for PCSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSEL to value 0"]
impl crate::Resettable for PCSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}