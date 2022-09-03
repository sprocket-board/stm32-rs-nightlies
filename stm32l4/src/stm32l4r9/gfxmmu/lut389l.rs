#[doc = "Register `LUT389L` reader"]
pub struct R(crate::R<LUT389L_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT389L_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT389L_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT389L_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT389L` writer"]
pub struct W(crate::W<LUT389L_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT389L_SPEC>;
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
impl From<crate::W<LUT389L_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT389L_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUT389L_SPEC, bool, O>;
#[doc = "Field `FVB` reader - First Valid Block"]
pub type FVB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FVB` writer - First Valid Block"]
pub type FVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT389L_SPEC, u8, u8, 8, O>;
#[doc = "Field `LVB` reader - Last Valid Block"]
pub type LVB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LVB` writer - Last Valid Block"]
pub type LVB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT389L_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    pub fn fvb(&self) -> FVB_R {
        FVB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    pub fn lvb(&self) -> LVB_R {
        LVB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:15 - First Valid Block"]
    #[inline(always)]
    pub fn fvb(&mut self) -> FVB_W<8> {
        FVB_W::new(self)
    }
    #[doc = "Bits 16:23 - Last Valid Block"]
    #[inline(always)]
    pub fn lvb(&mut self) -> LVB_W<16> {
        LVB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU LUT entry 389 low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut389l](index.html) module"]
pub struct LUT389L_SPEC;
impl crate::RegisterSpec for LUT389L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut389l::R](R) reader structure"]
impl crate::Readable for LUT389L_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut389l::W](W) writer structure"]
impl crate::Writable for LUT389L_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT389L to value 0"]
impl crate::Resettable for LUT389L_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
