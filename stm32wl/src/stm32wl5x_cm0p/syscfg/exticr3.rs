#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - EXTI 8 configuration bits"]
pub type EXTI8_R = crate::FieldReader<u8, EXTI8_A>;
#[doc = "EXTI 8 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: Select PA8 as the source input for the EXTI8 external interrupt"]
    Pa8 = 0,
    #[doc = "1: Select PB8 as the source input for the EXTI8 external interrupt"]
    Pb8 = 1,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
impl EXTI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::Pa8),
            1 => Some(EXTI8_A::Pb8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8_A::Pa8
    }
    #[doc = "Checks if the value of the field is `Pb8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8_A::Pb8
    }
}
#[doc = "Field `EXTI8` writer - EXTI 8 configuration bits"]
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI8_A, 3, O>;
impl<'a, const O: u8> EXTI8_W<'a, O> {
    #[doc = "Select PA8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pa8)
    }
    #[doc = "Select PB8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::Pb8)
    }
}
#[doc = "Field `EXTI9` reader - EXTI 9 configuration bits"]
pub type EXTI9_R = crate::FieldReader<u8, EXTI9_A>;
#[doc = "EXTI 9 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: Select PA9 as the source input for the EXTI9 external interrupt"]
    Pa9 = 0,
    #[doc = "1: Select PB9 as the source input for the EXTI9 external interrupt"]
    Pb9 = 1,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
impl EXTI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::Pa9),
            1 => Some(EXTI9_A::Pb9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9_A::Pa9
    }
    #[doc = "Checks if the value of the field is `Pb9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9_A::Pb9
    }
}
#[doc = "Field `EXTI9` writer - EXTI 9 configuration bits"]
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI9_A, 3, O>;
impl<'a, const O: u8> EXTI9_W<'a, O> {
    #[doc = "Select PA9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pa9)
    }
    #[doc = "Select PB9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::Pb9)
    }
}
#[doc = "Field `EXTI10` reader - EXTI 10 configuration bits"]
pub type EXTI10_R = crate::FieldReader<u8, EXTI10_A>;
#[doc = "EXTI 10 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: Select PA10 as the source input for the EXTI10 external interrupt"]
    Pa10 = 0,
    #[doc = "1: Select PB10 as the source input for the EXTI10 external interrupt"]
    Pb10 = 1,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
impl EXTI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::Pa10),
            1 => Some(EXTI10_A::Pb10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10_A::Pa10
    }
    #[doc = "Checks if the value of the field is `Pb10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10_A::Pb10
    }
}
#[doc = "Field `EXTI10` writer - EXTI 10 configuration bits"]
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI10_A, 3, O>;
impl<'a, const O: u8> EXTI10_W<'a, O> {
    #[doc = "Select PA10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pa10)
    }
    #[doc = "Select PB10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::Pb10)
    }
}
#[doc = "Field `EXTI11` reader - EXTI 11 configuration bits"]
pub type EXTI11_R = crate::FieldReader<u8, EXTI11_A>;
#[doc = "EXTI 11 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: Select PA11 as the source input for the EXTI11 external interrupt"]
    Pa11 = 0,
    #[doc = "1: Select PB11 as the source input for the EXTI11 external interrupt"]
    Pb11 = 1,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
impl EXTI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::Pa11),
            1 => Some(EXTI11_A::Pb11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11_A::Pa11
    }
    #[doc = "Checks if the value of the field is `Pb11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11_A::Pb11
    }
}
#[doc = "Field `EXTI11` writer - EXTI 11 configuration bits"]
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR3_SPEC, u8, EXTI11_A, 3, O>;
impl<'a, const O: u8> EXTI11_W<'a, O> {
    #[doc = "Select PA11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pa11)
    }
    #[doc = "Select PB11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::Pb11)
    }
}
impl R {
    #[doc = "Bits 0:2 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    #[doc = "Bits 4:6 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<4> {
        EXTI9_W::new(self)
    }
    #[doc = "Bits 8:10 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<8> {
        EXTI10_W::new(self)
    }
    #[doc = "Bits 12:14 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<12> {
        EXTI11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
