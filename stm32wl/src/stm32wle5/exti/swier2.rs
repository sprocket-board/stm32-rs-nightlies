#[doc = "Register `SWIER2` reader"]
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWIER2` writer"]
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWI34` reader - Software interrupt on event"]
pub type SWI34_R = crate::BitReader<SWI34W_A>;
#[doc = "Software interrupt on event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWI34W_A {
    #[doc = "1: Generates an interrupt request"]
    Pend = 1,
}
impl From<SWI34W_A> for bool {
    #[inline(always)]
    fn from(variant: SWI34W_A) -> Self {
        variant as u8 != 0
    }
}
impl SWI34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWI34W_A> {
        match self.bits {
            true => Some(SWI34W_A::Pend),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pend`"]
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SWI34W_A::Pend
    }
}
#[doc = "Field `SWI34` writer - Software interrupt on event"]
pub type SWI34_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, SWI34W_A, O>;
impl<'a, const O: u8> SWI34_W<'a, O> {
    #[doc = "Generates an interrupt request"]
    #[inline(always)]
    pub fn pend(self) -> &'a mut W {
        self.variant(SWI34W_A::Pend)
    }
}
#[doc = "Field `SWI45` reader - Software interrupt on event 45"]
pub use SWI34_R as SWI45_R;
#[doc = "Field `SWI45` writer - Software interrupt on event 45"]
pub use SWI34_W as SWI45_W;
impl R {
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi34(&self) -> SWI34_R {
        SWI34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    pub fn swi45(&self) -> SWI45_R {
        SWI45_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Software interrupt on event"]
    #[inline(always)]
    pub fn swi34(&mut self) -> SWI34_W<2> {
        SWI34_W::new(self)
    }
    #[doc = "Bit 13 - Software interrupt on event 45"]
    #[inline(always)]
    pub fn swi45(&mut self) -> SWI45_W<13> {
        SWI45_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "software interrupt event register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swier2](index.html) module"]
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swier2::R](R) reader structure"]
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swier2::W](W) writer structure"]
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for SWIER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
