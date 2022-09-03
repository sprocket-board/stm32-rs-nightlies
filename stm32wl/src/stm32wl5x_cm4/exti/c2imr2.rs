#[doc = "Register `C2IMR2` reader"]
pub struct R(crate::R<C2IMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2IMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2IMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2IMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2IMR2` writer"]
pub struct W(crate::W<C2IMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2IMR2_SPEC>;
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
impl From<crate::W<C2IMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2IMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IM34` reader - wakeup with interrupt mask on event input"]
pub type IM34_R = crate::BitReader<IM34_A>;
#[doc = "wakeup with interrupt mask on event input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IM34_A {
    #[doc = "0: Interrupt request line is masked"]
    Masked = 0,
    #[doc = "1: Interrupt request line is unmasked"]
    Unmasked = 1,
}
impl From<IM34_A> for bool {
    #[inline(always)]
    fn from(variant: IM34_A) -> Self {
        variant as u8 != 0
    }
}
impl IM34_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IM34_A {
        match self.bits {
            false => IM34_A::Masked,
            true => IM34_A::Unmasked,
        }
    }
    #[doc = "Checks if the value of the field is `Masked`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == IM34_A::Masked
    }
    #[doc = "Checks if the value of the field is `Unmasked`"]
    #[inline(always)]
    pub fn is_unmasked(&self) -> bool {
        *self == IM34_A::Unmasked
    }
}
#[doc = "Field `IM34` writer - wakeup with interrupt mask on event input"]
pub type IM34_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2IMR2_SPEC, IM34_A, O>;
impl<'a, const O: u8> IM34_W<'a, O> {
    #[doc = "Interrupt request line is masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(IM34_A::Masked)
    }
    #[doc = "Interrupt request line is unmasked"]
    #[inline(always)]
    pub fn unmasked(self) -> &'a mut W {
        self.variant(IM34_A::Unmasked)
    }
}
#[doc = "Field `IM36` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM36_R;
#[doc = "Field `IM37` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM37_R;
#[doc = "Field `IM38` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM38_R;
#[doc = "Field `IM39` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM39_R;
#[doc = "Field `IM40` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM40_R;
#[doc = "Field `IM41` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM41_R;
#[doc = "Field `IM42` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM42_R;
#[doc = "Field `IM43` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM43_R;
#[doc = "Field `IM44` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM44_R;
#[doc = "Field `IM45` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM45_R;
#[doc = "Field `IM46` reader - wakeup with interrupt mask on event input"]
pub use IM34_R as IM46_R;
#[doc = "Field `IM36` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM36_W;
#[doc = "Field `IM37` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM37_W;
#[doc = "Field `IM38` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM38_W;
#[doc = "Field `IM39` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM39_W;
#[doc = "Field `IM40` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM40_W;
#[doc = "Field `IM41` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM41_W;
#[doc = "Field `IM42` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM42_W;
#[doc = "Field `IM43` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM43_W;
#[doc = "Field `IM44` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM44_W;
#[doc = "Field `IM45` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM45_W;
#[doc = "Field `IM46` writer - wakeup with interrupt mask on event input"]
pub use IM34_W as IM46_W;
impl R {
    #[doc = "Bit 2 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im39(&self) -> IM39_R {
        IM39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im43(&self) -> IM43_R {
        IM43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im44(&self) -> IM44_R {
        IM44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im45(&self) -> IM45_R {
        IM45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im46(&self) -> IM46_R {
        IM46_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im34(&mut self) -> IM34_W<2> {
        IM34_W::new(self)
    }
    #[doc = "Bit 4 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im36(&mut self) -> IM36_W<4> {
        IM36_W::new(self)
    }
    #[doc = "Bit 5 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im37(&mut self) -> IM37_W<5> {
        IM37_W::new(self)
    }
    #[doc = "Bit 6 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im38(&mut self) -> IM38_W<6> {
        IM38_W::new(self)
    }
    #[doc = "Bit 7 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im39(&mut self) -> IM39_W<7> {
        IM39_W::new(self)
    }
    #[doc = "Bit 8 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im40(&mut self) -> IM40_W<8> {
        IM40_W::new(self)
    }
    #[doc = "Bit 9 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im41(&mut self) -> IM41_W<9> {
        IM41_W::new(self)
    }
    #[doc = "Bit 10 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im42(&mut self) -> IM42_W<10> {
        IM42_W::new(self)
    }
    #[doc = "Bit 11 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im43(&mut self) -> IM43_W<11> {
        IM43_W::new(self)
    }
    #[doc = "Bit 12 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im44(&mut self) -> IM44_W<12> {
        IM44_W::new(self)
    }
    #[doc = "Bit 13 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im45(&mut self) -> IM45_W<13> {
        IM45_W::new(self)
    }
    #[doc = "Bit 14 - wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im46(&mut self) -> IM46_W<14> {
        IM46_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "wakeup with interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2imr2](index.html) module"]
pub struct C2IMR2_SPEC;
impl crate::RegisterSpec for C2IMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2imr2::R](R) reader structure"]
impl crate::Readable for C2IMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2imr2::W](W) writer structure"]
impl crate::Writable for C2IMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2IMR2 to value 0"]
impl crate::Resettable for C2IMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
