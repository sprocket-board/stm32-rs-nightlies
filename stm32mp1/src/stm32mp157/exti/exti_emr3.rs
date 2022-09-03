#[doc = "Register `EXTI_EMR3` reader"]
pub struct R(crate::R<EXTI_EMR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTI_EMR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTI_EMR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTI_EMR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTI_EMR3` writer"]
pub struct W(crate::W<EXTI_EMR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTI_EMR3_SPEC>;
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
impl From<crate::W<EXTI_EMR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTI_EMR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM66` reader - EM66"]
pub type EM66_R = crate::BitReader<bool>;
#[doc = "Field `EM66` writer - EM66"]
pub type EM66_W<'a, const O: u8> = crate::BitWriter<'a, u32, EXTI_EMR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    pub fn em66(&self) -> EM66_R {
        EM66_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - EM66"]
    #[inline(always)]
    pub fn em66(&mut self) -> EM66_W<2> {
        EM66_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exti_emr3](index.html) module"]
pub struct EXTI_EMR3_SPEC;
impl crate::RegisterSpec for EXTI_EMR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exti_emr3::R](R) reader structure"]
impl crate::Readable for EXTI_EMR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exti_emr3::W](W) writer structure"]
impl crate::Writable for EXTI_EMR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTI_EMR3 to value 0"]
impl crate::Resettable for EXTI_EMR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
