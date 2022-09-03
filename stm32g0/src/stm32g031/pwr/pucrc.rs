#[doc = "Register `PUCRC` reader"]
pub struct R(crate::R<PUCRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUCRC` writer"]
pub struct W(crate::W<PUCRC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRC_SPEC>;
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
impl From<crate::W<PUCRC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PU6` reader - Port C pull-up bit y (y=0..15)"]
pub type PU6_R = crate::BitReader<bool>;
#[doc = "Field `PU6` writer - Port C pull-up bit y (y=0..15)"]
pub type PU6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
#[doc = "Field `PU7` reader - Port C pull-up bit y (y=0..15)"]
pub type PU7_R = crate::BitReader<bool>;
#[doc = "Field `PU7` writer - Port C pull-up bit y (y=0..15)"]
pub type PU7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
#[doc = "Field `PU13` reader - Port C pull-up bit y (y=0..15)"]
pub type PU13_R = crate::BitReader<bool>;
#[doc = "Field `PU13` writer - Port C pull-up bit y (y=0..15)"]
pub type PU13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
#[doc = "Field `PU14` reader - Port C pull-up bit y (y=0..15)"]
pub type PU14_R = crate::BitReader<bool>;
#[doc = "Field `PU14` writer - Port C pull-up bit y (y=0..15)"]
pub type PU14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
#[doc = "Field `PU15` reader - Port C pull-up bit y (y=0..15)"]
pub type PU15_R = crate::BitReader<bool>;
#[doc = "Field `PU15` writer - Port C pull-up bit y (y=0..15)"]
pub type PU15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 6 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&self) -> PU14_R {
        PU14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&self) -> PU15_R {
        PU15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<6> {
        PU6_W::new(self)
    }
    #[doc = "Bit 7 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<7> {
        PU7_W::new(self)
    }
    #[doc = "Bit 13 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<13> {
        PU13_W::new(self)
    }
    #[doc = "Bit 14 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu14(&mut self) -> PU14_W<14> {
        PU14_W::new(self)
    }
    #[doc = "Bit 15 - Port C pull-up bit y (y=0..15)"]
    #[inline(always)]
    pub fn pu15(&mut self) -> PU15_W<15> {
        PU15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Port C pull-up control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pucrc](index.html) module"]
pub struct PUCRC_SPEC;
impl crate::RegisterSpec for PUCRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pucrc::R](R) reader structure"]
impl crate::Readable for PUCRC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pucrc::W](W) writer structure"]
impl crate::Writable for PUCRC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PUCRC to value 0"]
impl crate::Resettable for PUCRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
