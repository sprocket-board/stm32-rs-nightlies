#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - EXTI12 configuration bits"]
pub type EXTI12_R = crate::FieldReader<u8, EXTI12_A>;
#[doc = "EXTI12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: Select PA12 as the source input for the EXTI12 external interrupt"]
    Pa12 = 0,
    #[doc = "1: Select PB12 as the source input for the EXTI12 external interrupt"]
    Pb12 = 1,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
impl EXTI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::Pa12),
            1 => Some(EXTI12_A::Pb12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_A::Pa12
    }
    #[doc = "Checks if the value of the field is `Pb12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12_A::Pb12
    }
}
#[doc = "Field `EXTI12` writer - EXTI12 configuration bits"]
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI12_A, 3, O>;
impl<'a, const O: u8> EXTI12_W<'a, O> {
    #[doc = "Select PA12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pa12)
    }
    #[doc = "Select PB12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::Pb12)
    }
}
#[doc = "Field `EXTI13` reader - EXTI13 configuration bits"]
pub type EXTI13_R = crate::FieldReader<u8, EXTI13_A>;
#[doc = "EXTI13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: Select PA13 as the source input for the EXTI13 external interrupt"]
    Pa13 = 0,
    #[doc = "1: Select PB13 as the source input for the EXTI13 external interrupt"]
    Pb13 = 1,
    #[doc = "2: Select PC13 as the source input for the EXTI13 external interrupt"]
    Pc13 = 2,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
impl EXTI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::Pa13),
            1 => Some(EXTI13_A::Pb13),
            2 => Some(EXTI13_A::Pc13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_A::Pa13
    }
    #[doc = "Checks if the value of the field is `Pb13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13_A::Pb13
    }
    #[doc = "Checks if the value of the field is `Pc13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_A::Pc13
    }
}
#[doc = "Field `EXTI13` writer - EXTI13 configuration bits"]
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI13_A, 3, O>;
impl<'a, const O: u8> EXTI13_W<'a, O> {
    #[doc = "Select PA13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pa13)
    }
    #[doc = "Select PB13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pb13)
    }
    #[doc = "Select PC13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::Pc13)
    }
}
#[doc = "Field `EXTI14` reader - EXTI14 configuration bits"]
pub type EXTI14_R = crate::FieldReader<u8, EXTI14_A>;
#[doc = "EXTI14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: Select PA14 as the source input for the EXTI14 external interrupt"]
    Pa14 = 0,
    #[doc = "1: Select PB14 as the source input for the EXTI14 external interrupt"]
    Pb14 = 1,
    #[doc = "2: Select PC14 as the source input for the EXTI14 external interrupt"]
    Pc14 = 2,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
impl EXTI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::Pa14),
            1 => Some(EXTI14_A::Pb14),
            2 => Some(EXTI14_A::Pc14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_A::Pa14
    }
    #[doc = "Checks if the value of the field is `Pb14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_A::Pb14
    }
    #[doc = "Checks if the value of the field is `Pc14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_A::Pc14
    }
}
#[doc = "Field `EXTI14` writer - EXTI14 configuration bits"]
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI14_A, 3, O>;
impl<'a, const O: u8> EXTI14_W<'a, O> {
    #[doc = "Select PA14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pa14)
    }
    #[doc = "Select PB14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pb14)
    }
    #[doc = "Select PC14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::Pc14)
    }
}
#[doc = "Field `EXTI15` reader - EXTI15 configuration bits"]
pub type EXTI15_R = crate::FieldReader<u8, EXTI15_A>;
#[doc = "EXTI15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: Select PA15 as the source input for the EXTI15 external interrupt"]
    Pa15 = 0,
    #[doc = "1: Select PB15 as the source input for the EXTI15 external interrupt"]
    Pb15 = 1,
    #[doc = "2: Select PC15 as the source input for the EXTI15 external interrupt"]
    Pc15 = 2,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
impl EXTI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::Pa15),
            1 => Some(EXTI15_A::Pb15),
            2 => Some(EXTI15_A::Pc15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Pa15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_A::Pa15
    }
    #[doc = "Checks if the value of the field is `Pb15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_A::Pb15
    }
    #[doc = "Checks if the value of the field is `Pc15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_A::Pc15
    }
}
#[doc = "Field `EXTI15` writer - EXTI15 configuration bits"]
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EXTICR4_SPEC, u8, EXTI15_A, 3, O>;
impl<'a, const O: u8> EXTI15_W<'a, O> {
    #[doc = "Select PA15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pa15)
    }
    #[doc = "Select PB15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pb15)
    }
    #[doc = "Select PC15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::Pc15)
    }
}
impl R {
    #[doc = "Bits 0:2 - EXTI12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - EXTI13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - EXTI14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - EXTI15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - EXTI12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    #[doc = "Bits 4:6 - EXTI13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<4> {
        EXTI13_W::new(self)
    }
    #[doc = "Bits 8:10 - EXTI14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<8> {
        EXTI14_W::new(self)
    }
    #[doc = "Bits 12:14 - EXTI15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<12> {
        EXTI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
