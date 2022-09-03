#[doc = "Register `LUT452H` reader"]
pub struct R(crate::R<LUT452H_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUT452H_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUT452H_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUT452H_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUT452H` writer"]
pub struct W(crate::W<LUT452H_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUT452H_SPEC>;
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
impl From<crate::W<LUT452H_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUT452H_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LO` reader - Line offset"]
pub type LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LO` writer - Line offset"]
pub type LO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LUT452H_SPEC, u32, u32, 18, O>;
impl R {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(((self.bits >> 4) & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&mut self) -> LO_W<4> {
        LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU LUT entry 452 high\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lut452h](index.html) module"]
pub struct LUT452H_SPEC;
impl crate::RegisterSpec for LUT452H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lut452h::R](R) reader structure"]
impl crate::Readable for LUT452H_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lut452h::W](W) writer structure"]
impl crate::Writable for LUT452H_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LUT452H to value 0"]
impl crate::Resettable for LUT452H_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
