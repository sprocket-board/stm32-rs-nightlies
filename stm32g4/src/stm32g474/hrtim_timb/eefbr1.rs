#[doc = "Register `EEFBR1` reader"]
pub struct R(crate::R<EEFBR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFBR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFBR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFBR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFBR1` writer"]
pub struct W(crate::W<EEFBR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFBR1_SPEC>;
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
impl From<crate::W<EEFBR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFBR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub type EE1LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub type EE1LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFBR1_SPEC, bool, O>;
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub type EE1FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub type EE1FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFBR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub type EE2LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub type EE2LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFBR1_SPEC, bool, O>;
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub type EE2FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub type EE2FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFBR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub type EE3LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub type EE3LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFBR1_SPEC, bool, O>;
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub type EE3FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub type EE3FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFBR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub type EE4LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub type EE4LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFBR1_SPEC, bool, O>;
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub type EE4FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub type EE4FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFBR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub type EE5LTCH_R = crate::BitReader<bool>;
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub type EE5LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFBR1_SPEC, bool, O>;
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub type EE5FLTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub type EE5FLTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EEFBR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<0> {
        EE1LTCH_W::new(self)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<1> {
        EE1FLTR_W::new(self)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<6> {
        EE2LTCH_W::new(self)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<7> {
        EE2FLTR_W::new(self)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<12> {
        EE3LTCH_W::new(self)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<13> {
        EE3FLTR_W::new(self)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<18> {
        EE4LTCH_W::new(self)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<19> {
        EE4FLTR_W::new(self)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<24> {
        EE5LTCH_W::new(self)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<25> {
        EE5FLTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefbr1](index.html) module"]
pub struct EEFBR1_SPEC;
impl crate::RegisterSpec for EEFBR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefbr1::R](R) reader structure"]
impl crate::Readable for EEFBR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefbr1::W](W) writer structure"]
impl crate::Writable for EEFBR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFBR1 to value 0"]
impl crate::Resettable for EEFBR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
