#[doc = "Register `EECR1` reader"]
pub struct R(crate::R<EECR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EECR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EECR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EECR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EECR1` writer"]
pub struct W(crate::W<EECR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EECR1_SPEC>;
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
impl From<crate::W<EECR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EECR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE1SRC` reader - External Event 1 Source"]
pub type EE1SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE1SRC` writer - External Event 1 Source"]
pub type EE1SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE1POL` reader - External Event 1 Polarity"]
pub type EE1POL_R = crate::BitReader<bool>;
#[doc = "Field `EE1POL` writer - External Event 1 Polarity"]
pub type EE1POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE1SNS` reader - External Event 1 Sensitivity"]
pub type EE1SNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE1SNS` writer - External Event 1 Sensitivity"]
pub type EE1SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE1FAST` reader - External Event 1 Fast mode"]
pub type EE1FAST_R = crate::BitReader<bool>;
#[doc = "Field `EE1FAST` writer - External Event 1 Fast mode"]
pub type EE1FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE2SRC` reader - External Event 2 Source"]
pub type EE2SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE2SRC` writer - External Event 2 Source"]
pub type EE2SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE2POL` reader - External Event 2 Polarity"]
pub type EE2POL_R = crate::BitReader<bool>;
#[doc = "Field `EE2POL` writer - External Event 2 Polarity"]
pub type EE2POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE2SNS` reader - External Event 2 Sensitivity"]
pub type EE2SNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE2SNS` writer - External Event 2 Sensitivity"]
pub type EE2SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE2FAST` reader - External Event 2 Fast mode"]
pub type EE2FAST_R = crate::BitReader<bool>;
#[doc = "Field `EE2FAST` writer - External Event 2 Fast mode"]
pub type EE2FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE3SRC` reader - External Event 3 Source"]
pub type EE3SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE3SRC` writer - External Event 3 Source"]
pub type EE3SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE3POL` reader - External Event 3 Polarity"]
pub type EE3POL_R = crate::BitReader<bool>;
#[doc = "Field `EE3POL` writer - External Event 3 Polarity"]
pub type EE3POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE3SNS` reader - External Event 3 Sensitivity"]
pub type EE3SNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE3SNS` writer - External Event 3 Sensitivity"]
pub type EE3SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE3FAST` reader - External Event 3 Fast mode"]
pub type EE3FAST_R = crate::BitReader<bool>;
#[doc = "Field `EE3FAST` writer - External Event 3 Fast mode"]
pub type EE3FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE4SRC` reader - External Event 4 Source"]
pub type EE4SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE4SRC` writer - External Event 4 Source"]
pub type EE4SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE4POL` reader - External Event 4 Polarity"]
pub type EE4POL_R = crate::BitReader<bool>;
#[doc = "Field `EE4POL` writer - External Event 4 Polarity"]
pub type EE4POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE4SNS` reader - External Event 4 Sensitivity"]
pub type EE4SNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE4SNS` writer - External Event 4 Sensitivity"]
pub type EE4SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE4FAST` reader - External Event 4 Fast mode"]
pub type EE4FAST_R = crate::BitReader<bool>;
#[doc = "Field `EE4FAST` writer - External Event 4 Fast mode"]
pub type EE4FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE5SRC` reader - External Event 5 Source"]
pub type EE5SRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE5SRC` writer - External Event 5 Source"]
pub type EE5SRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE5POL` reader - External Event 5 Polarity"]
pub type EE5POL_R = crate::BitReader<bool>;
#[doc = "Field `EE5POL` writer - External Event 5 Polarity"]
pub type EE5POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
#[doc = "Field `EE5SNS` reader - External Event 5 Sensitivity"]
pub type EE5SNS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EE5SNS` writer - External Event 5 Sensitivity"]
pub type EE5SNS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EECR1_SPEC, u8, u8, 2, O>;
#[doc = "Field `EE5FAST` reader - External Event 5 Fast mode"]
pub type EE5FAST_R = crate::BitReader<bool>;
#[doc = "Field `EE5FAST` writer - External Event 5 Fast mode"]
pub type EE5FAST_W<'a, const O: u8> = crate::BitWriter<'a, u32, EECR1_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&self) -> EE1SRC_R {
        EE1SRC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&self) -> EE1POL_R {
        EE1POL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&self) -> EE1SNS_R {
        EE1SNS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&self) -> EE1FAST_R {
        EE1FAST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&self) -> EE2SRC_R {
        EE2SRC_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&self) -> EE2POL_R {
        EE2POL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&self) -> EE2SNS_R {
        EE2SNS_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&self) -> EE2FAST_R {
        EE2FAST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&self) -> EE3SRC_R {
        EE3SRC_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&self) -> EE3POL_R {
        EE3POL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&self) -> EE3SNS_R {
        EE3SNS_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&self) -> EE3FAST_R {
        EE3FAST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&self) -> EE4SRC_R {
        EE4SRC_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&self) -> EE4POL_R {
        EE4POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&self) -> EE4SNS_R {
        EE4SNS_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&self) -> EE4FAST_R {
        EE4FAST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&self) -> EE5SRC_R {
        EE5SRC_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&self) -> EE5POL_R {
        EE5POL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&self) -> EE5SNS_R {
        EE5SNS_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&self) -> EE5FAST_R {
        EE5FAST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Event 1 Source"]
    #[inline(always)]
    pub fn ee1src(&mut self) -> EE1SRC_W<0> {
        EE1SRC_W::new(self)
    }
    #[doc = "Bit 2 - External Event 1 Polarity"]
    #[inline(always)]
    pub fn ee1pol(&mut self) -> EE1POL_W<2> {
        EE1POL_W::new(self)
    }
    #[doc = "Bits 3:4 - External Event 1 Sensitivity"]
    #[inline(always)]
    pub fn ee1sns(&mut self) -> EE1SNS_W<3> {
        EE1SNS_W::new(self)
    }
    #[doc = "Bit 5 - External Event 1 Fast mode"]
    #[inline(always)]
    pub fn ee1fast(&mut self) -> EE1FAST_W<5> {
        EE1FAST_W::new(self)
    }
    #[doc = "Bits 6:7 - External Event 2 Source"]
    #[inline(always)]
    pub fn ee2src(&mut self) -> EE2SRC_W<6> {
        EE2SRC_W::new(self)
    }
    #[doc = "Bit 8 - External Event 2 Polarity"]
    #[inline(always)]
    pub fn ee2pol(&mut self) -> EE2POL_W<8> {
        EE2POL_W::new(self)
    }
    #[doc = "Bits 9:10 - External Event 2 Sensitivity"]
    #[inline(always)]
    pub fn ee2sns(&mut self) -> EE2SNS_W<9> {
        EE2SNS_W::new(self)
    }
    #[doc = "Bit 11 - External Event 2 Fast mode"]
    #[inline(always)]
    pub fn ee2fast(&mut self) -> EE2FAST_W<11> {
        EE2FAST_W::new(self)
    }
    #[doc = "Bits 12:13 - External Event 3 Source"]
    #[inline(always)]
    pub fn ee3src(&mut self) -> EE3SRC_W<12> {
        EE3SRC_W::new(self)
    }
    #[doc = "Bit 14 - External Event 3 Polarity"]
    #[inline(always)]
    pub fn ee3pol(&mut self) -> EE3POL_W<14> {
        EE3POL_W::new(self)
    }
    #[doc = "Bits 15:16 - External Event 3 Sensitivity"]
    #[inline(always)]
    pub fn ee3sns(&mut self) -> EE3SNS_W<15> {
        EE3SNS_W::new(self)
    }
    #[doc = "Bit 17 - External Event 3 Fast mode"]
    #[inline(always)]
    pub fn ee3fast(&mut self) -> EE3FAST_W<17> {
        EE3FAST_W::new(self)
    }
    #[doc = "Bits 18:19 - External Event 4 Source"]
    #[inline(always)]
    pub fn ee4src(&mut self) -> EE4SRC_W<18> {
        EE4SRC_W::new(self)
    }
    #[doc = "Bit 20 - External Event 4 Polarity"]
    #[inline(always)]
    pub fn ee4pol(&mut self) -> EE4POL_W<20> {
        EE4POL_W::new(self)
    }
    #[doc = "Bits 21:22 - External Event 4 Sensitivity"]
    #[inline(always)]
    pub fn ee4sns(&mut self) -> EE4SNS_W<21> {
        EE4SNS_W::new(self)
    }
    #[doc = "Bit 23 - External Event 4 Fast mode"]
    #[inline(always)]
    pub fn ee4fast(&mut self) -> EE4FAST_W<23> {
        EE4FAST_W::new(self)
    }
    #[doc = "Bits 24:25 - External Event 5 Source"]
    #[inline(always)]
    pub fn ee5src(&mut self) -> EE5SRC_W<24> {
        EE5SRC_W::new(self)
    }
    #[doc = "Bit 26 - External Event 5 Polarity"]
    #[inline(always)]
    pub fn ee5pol(&mut self) -> EE5POL_W<26> {
        EE5POL_W::new(self)
    }
    #[doc = "Bits 27:28 - External Event 5 Sensitivity"]
    #[inline(always)]
    pub fn ee5sns(&mut self) -> EE5SNS_W<27> {
        EE5SNS_W::new(self)
    }
    #[doc = "Bit 29 - External Event 5 Fast mode"]
    #[inline(always)]
    pub fn ee5fast(&mut self) -> EE5FAST_W<29> {
        EE5FAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer External Event Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr1](index.html) module"]
pub struct EECR1_SPEC;
impl crate::RegisterSpec for EECR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eecr1::R](R) reader structure"]
impl crate::Readable for EECR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eecr1::W](W) writer structure"]
impl crate::Writable for EECR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EECR1 to value 0"]
impl crate::Resettable for EECR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
