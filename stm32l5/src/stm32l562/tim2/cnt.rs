#[doc = "Register `CNT` reader"]
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNT` writer"]
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_L` reader - Least significant part of counter value"]
pub type CNT_L_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_L` writer - Least significant part of counter value"]
pub type CNT_L_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u16, u16, 16, O>;
#[doc = "Field `CNT_H` reader - Most significant part counter value (on TIM2 and TIM5)"]
pub type CNT_H_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CNT_H` writer - Most significant part counter value (on TIM2 and TIM5)"]
pub type CNT_H_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u16, u16, 15, O>;
#[doc = "Field `CNT_bit31` reader - Most significant bit of counter value (on TIM2 and TIM5)"]
pub type CNT_BIT31_R = crate::BitReader<bool>;
#[doc = "Field `CNT_bit31` writer - Most significant bit of counter value (on TIM2 and TIM5)"]
pub type CNT_BIT31_W<'a, const O: u8> = crate::BitWriter<'a, u32, CNT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&self) -> CNT_L_R {
        CNT_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&self) -> CNT_BIT31_R {
        CNT_BIT31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Least significant part of counter value"]
    #[inline(always)]
    pub fn cnt_l(&mut self) -> CNT_L_W<0> {
        CNT_L_W::new(self)
    }
    #[doc = "Bits 16:30 - Most significant part counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_h(&mut self) -> CNT_H_W<16> {
        CNT_H_W::new(self)
    }
    #[doc = "Bit 31 - Most significant bit of counter value (on TIM2 and TIM5)"]
    #[inline(always)]
    pub fn cnt_bit31(&mut self) -> CNT_BIT31_W<31> {
        CNT_BIT31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnt](index.html) module"]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnt::R](R) reader structure"]
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnt::W](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
