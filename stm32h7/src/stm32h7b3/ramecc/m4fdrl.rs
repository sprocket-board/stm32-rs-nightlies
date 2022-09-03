#[doc = "Register `M4FDRL` reader"]
pub struct R(crate::R<M4FDRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4FDRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4FDRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4FDRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M4FDRL` writer"]
pub struct W(crate::W<M4FDRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M4FDRL_SPEC>;
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
impl From<crate::W<M4FDRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M4FDRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FDATAH_R = crate::FieldReader<u32, u32>;
#[doc = "Field `FDATAH` writer - Failing data high (64-bit memory)"]
pub type FDATAH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, M4FDRL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FDATAH_R {
        FDATAH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&mut self) -> FDATAH_W<0> {
        FDATAH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4fdrl](index.html) module"]
pub struct M4FDRL_SPEC;
impl crate::RegisterSpec for M4FDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4fdrl::R](R) reader structure"]
impl crate::Readable for M4FDRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m4fdrl::W](W) writer structure"]
impl crate::Writable for M4FDRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M4FDRL to value 0"]
impl crate::Resettable for M4FDRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
