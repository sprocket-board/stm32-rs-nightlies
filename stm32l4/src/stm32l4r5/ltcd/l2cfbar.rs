#[doc = "Register `L2CFBAR` reader"]
pub struct R(crate::R<L2CFBAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2CFBAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2CFBAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2CFBAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L2CFBAR` writer"]
pub struct W(crate::W<L2CFBAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L2CFBAR_SPEC>;
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
impl From<crate::W<L2CFBAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L2CFBAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBADD` reader - Color Frame Buffer Start Address"]
pub type CFBADD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFBADD` writer - Color Frame Buffer Start Address"]
pub type CFBADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, L2CFBAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&self) -> CFBADD_R {
        CFBADD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Color Frame Buffer Start Address"]
    #[inline(always)]
    pub fn cfbadd(&mut self) -> CFBADD_W<0> {
        CFBADD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Color Frame Buffer Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2cfbar](index.html) module"]
pub struct L2CFBAR_SPEC;
impl crate::RegisterSpec for L2CFBAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2cfbar::R](R) reader structure"]
impl crate::Readable for L2CFBAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l2cfbar::W](W) writer structure"]
impl crate::Writable for L2CFBAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L2CFBAR to value 0"]
impl crate::Resettable for L2CFBAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
