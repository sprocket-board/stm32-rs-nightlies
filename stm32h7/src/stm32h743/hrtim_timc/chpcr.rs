#[doc = "Register `CHPCR` reader"]
pub struct R(crate::R<CHPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHPCR` writer"]
pub struct W(crate::W<CHPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPCR_SPEC>;
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
impl From<crate::W<CHPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPFRQ` reader - Timerx carrier frequency value"]
pub type CHPFRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHPFRQ` writer - Timerx carrier frequency value"]
pub type CHPFRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHPCR_SPEC, u8, u8, 4, O>;
#[doc = "Field `CHPDTY` reader - Timerx chopper duty cycle value"]
pub type CHPDTY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHPDTY` writer - Timerx chopper duty cycle value"]
pub type CHPDTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHPCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type STRTPW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type STRTPW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHPCR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> CHPFRQ_R {
        CHPFRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&self) -> CHPDTY_R {
        CHPDTY_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> STRTPW_R {
        STRTPW_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&mut self) -> CHPFRQ_W<0> {
        CHPFRQ_W::new(self)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&mut self) -> CHPDTY_W<4> {
        CHPDTY_W::new(self)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&mut self) -> STRTPW_W<7> {
        STRTPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpcr](index.html) module"]
pub struct CHPCR_SPEC;
impl crate::RegisterSpec for CHPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chpcr::R](R) reader structure"]
impl crate::Readable for CHPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chpcr::W](W) writer structure"]
impl crate::Writable for CHPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPCR to value 0"]
impl crate::Resettable for CHPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
