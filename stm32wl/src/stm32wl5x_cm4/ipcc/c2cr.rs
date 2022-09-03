#[doc = "Register `C2CR` reader"]
pub struct R(crate::R<C2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR` writer"]
pub struct W(crate::W<C2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR_SPEC>;
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
impl From<crate::W<C2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RXOIE_R = crate::BitReader<RXOIE_A>;
#[doc = "RXOIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXOIE_A {
    #[doc = "0: Processor RX occupied interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    Enabled = 1,
}
impl From<RXOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXOIE_A {
        match self.bits {
            false => RXOIE_A::Disabled,
            true => RXOIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXOIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXOIE_A::Enabled
    }
}
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RXOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, RXOIE_A, O>;
impl<'a, const O: u8> RXOIE_W<'a, O> {
    #[doc = "Processor RX occupied interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXOIE_A::Disabled)
    }
    #[doc = "Enable an unmasked processor receive channel occupied to generate an RX occupied interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXOIE_A::Enabled)
    }
}
#[doc = "Field `TXFIE` reader - TXFIE"]
pub type TXFIE_R = crate::BitReader<TXFIE_A>;
#[doc = "TXFIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXFIE_A {
    #[doc = "0: Processor TX free interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    Enabled = 1,
}
impl From<TXFIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXFIE_A {
        match self.bits {
            false => TXFIE_A::Disabled,
            true => TXFIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXFIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXFIE_A::Enabled
    }
}
#[doc = "Field `TXFIE` writer - TXFIE"]
pub type TXFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR_SPEC, TXFIE_A, O>;
impl<'a, const O: u8> TXFIE_W<'a, O> {
    #[doc = "Processor TX free interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXFIE_A::Disabled)
    }
    #[doc = "Enable an unmasked processor transmit channel free to generate a TX free interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXFIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<0> {
        RXOIE_W::new(self)
    }
    #[doc = "Bit 16 - TXFIE"]
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W<16> {
        TXFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IPCC Processor 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](index.html) module"]
pub struct C2CR_SPEC;
impl crate::RegisterSpec for C2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr::R](R) reader structure"]
impl crate::Readable for C2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr::W](W) writer structure"]
impl crate::Writable for C2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2CR to value 0"]
impl crate::Resettable for C2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
