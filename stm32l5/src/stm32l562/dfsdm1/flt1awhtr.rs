#[doc = "Register `FLT1AWHTR` reader"]
pub struct R(crate::R<FLT1AWHTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLT1AWHTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLT1AWHTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLT1AWHTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLT1AWHTR` writer"]
pub struct W(crate::W<FLT1AWHTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLT1AWHTR_SPEC>;
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
impl From<crate::W<FLT1AWHTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLT1AWHTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKAWH` reader - Break signal assignment to analog watchdog high threshold event"]
pub type BKAWH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BKAWH` writer - Break signal assignment to analog watchdog high threshold event"]
pub type BKAWH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1AWHTR_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWHT` reader - Analog watchdog high threshold"]
pub type AWHT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `AWHT` writer - Analog watchdog high threshold"]
pub type AWHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLT1AWHTR_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&self) -> BKAWH_R {
        BKAWH_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&self) -> AWHT_R {
        AWHT_R::new(((self.bits >> 8) & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event"]
    #[inline(always)]
    pub fn bkawh(&mut self) -> BKAWH_W<0> {
        BKAWH_W::new(self)
    }
    #[doc = "Bits 8:31 - Analog watchdog high threshold"]
    #[inline(always)]
    pub fn awht(&mut self) -> AWHT_W<8> {
        AWHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "analog watchdog high threshold register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flt1awhtr](index.html) module"]
pub struct FLT1AWHTR_SPEC;
impl crate::RegisterSpec for FLT1AWHTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flt1awhtr::R](R) reader structure"]
impl crate::Readable for FLT1AWHTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flt1awhtr::W](W) writer structure"]
impl crate::Writable for FLT1AWHTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLT1AWHTR to value 0"]
impl crate::Resettable for FLT1AWHTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
