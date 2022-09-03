#[doc = "Register `MACHT0R` reader"]
pub struct R(crate::R<MACHT0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACHT0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACHT0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACHT0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACHT0R` writer"]
pub struct W(crate::W<MACHT0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACHT0R_SPEC>;
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
impl From<crate::W<MACHT0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACHT0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HT31T0` reader - MAC Hash Table First 32 Bits"]
pub type HT31T0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HT31T0` writer - MAC Hash Table First 32 Bits"]
pub type HT31T0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MACHT0R_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - MAC Hash Table First 32 Bits"]
    #[inline(always)]
    pub fn ht31t0(&self) -> HT31T0_R {
        HT31T0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Hash Table First 32 Bits"]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> HT31T0_W<0> {
        HT31T0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hash Table 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macht0r](index.html) module"]
pub struct MACHT0R_SPEC;
impl crate::RegisterSpec for MACHT0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macht0r::R](R) reader structure"]
impl crate::Readable for MACHT0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macht0r::W](W) writer structure"]
impl crate::Writable for MACHT0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACHT0R to value 0"]
impl crate::Resettable for MACHT0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
