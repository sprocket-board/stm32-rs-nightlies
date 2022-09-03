#[doc = "Register `RPR1` reader"]
pub struct R(crate::R<RPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RPR1` writer"]
pub struct W(crate::W<RPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RPR1_SPEC>;
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
impl From<crate::W<RPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RPIF0` reader - configurable event inputs x rising edge Pending bit."]
pub type RPIF0_R = crate::BitReader<RPIF0R_A>;
#[doc = "configurable event inputs x rising edge Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF0R_A {
    #[doc = "0: No trigger request occurred"]
    NotPending = 0,
    #[doc = "1: Selected trigger request occurred"]
    Pending = 1,
}
impl From<RPIF0R_A> for bool {
    #[inline(always)]
    fn from(variant: RPIF0R_A) -> Self {
        variant as u8 != 0
    }
}
impl RPIF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RPIF0R_A {
        match self.bits {
            false => RPIF0R_A::NotPending,
            true => RPIF0R_A::Pending,
        }
    }
    #[doc = "Checks if the value of the field is `NotPending`"]
    #[inline(always)]
    pub fn is_not_pending(&self) -> bool {
        *self == RPIF0R_A::NotPending
    }
    #[doc = "Checks if the value of the field is `Pending`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == RPIF0R_A::Pending
    }
}
#[doc = "configurable event inputs x rising edge Pending bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RPIF0W_AW {
    #[doc = "1: Clears pending bit"]
    Clear = 1,
}
impl From<RPIF0W_AW> for bool {
    #[inline(always)]
    fn from(variant: RPIF0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPIF0` writer - configurable event inputs x rising edge Pending bit."]
pub type RPIF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RPR1_SPEC, RPIF0W_AW, O>;
impl<'a, const O: u8> RPIF0_W<'a, O> {
    #[doc = "Clears pending bit"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(RPIF0W_AW::Clear)
    }
}
#[doc = "Field `RPIF1` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF1_R;
#[doc = "Field `RPIF2` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF2_R;
#[doc = "Field `RPIF3` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF3_R;
#[doc = "Field `RPIF4` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF4_R;
#[doc = "Field `RPIF5` reader - configurable event inputs x rising edge Pending bit"]
pub use RPIF0_R as RPIF5_R;
#[doc = "Field `RPIF6` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF6_R;
#[doc = "Field `RPIF7` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF7_R;
#[doc = "Field `RPIF8` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF8_R;
#[doc = "Field `RPIF9` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF9_R;
#[doc = "Field `RPIF10` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF10_R;
#[doc = "Field `RPIF11` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF11_R;
#[doc = "Field `RPIF12` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF12_R;
#[doc = "Field `RPIF13` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF13_R;
#[doc = "Field `RPIF14` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF14_R;
#[doc = "Field `RPIF15` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF15_R;
#[doc = "Field `RPIF16` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF16_R;
#[doc = "Field `RPIF17` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF17_R;
#[doc = "Field `RPIF18` reader - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_R as RPIF18_R;
#[doc = "Field `RPIF1` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF1_W;
#[doc = "Field `RPIF2` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF2_W;
#[doc = "Field `RPIF3` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF3_W;
#[doc = "Field `RPIF4` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF4_W;
#[doc = "Field `RPIF5` writer - configurable event inputs x rising edge Pending bit"]
pub use RPIF0_W as RPIF5_W;
#[doc = "Field `RPIF6` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF6_W;
#[doc = "Field `RPIF7` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF7_W;
#[doc = "Field `RPIF8` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF8_W;
#[doc = "Field `RPIF9` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF9_W;
#[doc = "Field `RPIF10` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF10_W;
#[doc = "Field `RPIF11` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF11_W;
#[doc = "Field `RPIF12` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF12_W;
#[doc = "Field `RPIF13` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF13_W;
#[doc = "Field `RPIF14` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF14_W;
#[doc = "Field `RPIF15` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF15_W;
#[doc = "Field `RPIF16` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF16_W;
#[doc = "Field `RPIF17` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF17_W;
#[doc = "Field `RPIF18` writer - configurable event inputs x rising edge Pending bit."]
pub use RPIF0_W as RPIF18_W;
impl R {
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif0(&self) -> RPIF0_R {
        RPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif1(&self) -> RPIF1_R {
        RPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif2(&self) -> RPIF2_R {
        RPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif3(&self) -> RPIF3_R {
        RPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif4(&self) -> RPIF4_R {
        RPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&self) -> RPIF5_R {
        RPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif6(&self) -> RPIF6_R {
        RPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif7(&self) -> RPIF7_R {
        RPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif8(&self) -> RPIF8_R {
        RPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif9(&self) -> RPIF9_R {
        RPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif10(&self) -> RPIF10_R {
        RPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif11(&self) -> RPIF11_R {
        RPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif12(&self) -> RPIF12_R {
        RPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif13(&self) -> RPIF13_R {
        RPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif14(&self) -> RPIF14_R {
        RPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif15(&self) -> RPIF15_R {
        RPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif16(&self) -> RPIF16_R {
        RPIF16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif17(&self) -> RPIF17_R {
        RPIF17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif18(&self) -> RPIF18_R {
        RPIF18_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif0(&mut self) -> RPIF0_W<0> {
        RPIF0_W::new(self)
    }
    #[doc = "Bit 1 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif1(&mut self) -> RPIF1_W<1> {
        RPIF1_W::new(self)
    }
    #[doc = "Bit 2 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif2(&mut self) -> RPIF2_W<2> {
        RPIF2_W::new(self)
    }
    #[doc = "Bit 3 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif3(&mut self) -> RPIF3_W<3> {
        RPIF3_W::new(self)
    }
    #[doc = "Bit 4 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif4(&mut self) -> RPIF4_W<4> {
        RPIF4_W::new(self)
    }
    #[doc = "Bit 5 - configurable event inputs x rising edge Pending bit"]
    #[inline(always)]
    pub fn rpif5(&mut self) -> RPIF5_W<5> {
        RPIF5_W::new(self)
    }
    #[doc = "Bit 6 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif6(&mut self) -> RPIF6_W<6> {
        RPIF6_W::new(self)
    }
    #[doc = "Bit 7 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif7(&mut self) -> RPIF7_W<7> {
        RPIF7_W::new(self)
    }
    #[doc = "Bit 8 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif8(&mut self) -> RPIF8_W<8> {
        RPIF8_W::new(self)
    }
    #[doc = "Bit 9 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif9(&mut self) -> RPIF9_W<9> {
        RPIF9_W::new(self)
    }
    #[doc = "Bit 10 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif10(&mut self) -> RPIF10_W<10> {
        RPIF10_W::new(self)
    }
    #[doc = "Bit 11 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif11(&mut self) -> RPIF11_W<11> {
        RPIF11_W::new(self)
    }
    #[doc = "Bit 12 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif12(&mut self) -> RPIF12_W<12> {
        RPIF12_W::new(self)
    }
    #[doc = "Bit 13 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif13(&mut self) -> RPIF13_W<13> {
        RPIF13_W::new(self)
    }
    #[doc = "Bit 14 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif14(&mut self) -> RPIF14_W<14> {
        RPIF14_W::new(self)
    }
    #[doc = "Bit 15 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif15(&mut self) -> RPIF15_W<15> {
        RPIF15_W::new(self)
    }
    #[doc = "Bit 16 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif16(&mut self) -> RPIF16_W<16> {
        RPIF16_W::new(self)
    }
    #[doc = "Bit 17 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif17(&mut self) -> RPIF17_W<17> {
        RPIF17_W::new(self)
    }
    #[doc = "Bit 18 - configurable event inputs x rising edge Pending bit."]
    #[inline(always)]
    pub fn rpif18(&mut self) -> RPIF18_W<18> {
        RPIF18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI rising edge pending register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rpr1](index.html) module"]
pub struct RPR1_SPEC;
impl crate::RegisterSpec for RPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rpr1::R](R) reader structure"]
impl crate::Readable for RPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rpr1::W](W) writer structure"]
impl crate::Writable for RPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RPR1 to value 0"]
impl crate::Resettable for RPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
