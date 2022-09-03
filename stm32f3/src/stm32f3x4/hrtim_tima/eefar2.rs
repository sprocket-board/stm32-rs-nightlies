#[doc = "Register `EEFAR2` reader"]
pub struct R(crate::R<EEFAR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEFAR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEFAR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEFAR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEFAR2` writer"]
pub struct W(crate::W<EEFAR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEFAR2_SPEC>;
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
impl From<crate::W<EEFAR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEFAR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub type EE6LTCH_R = crate::BitReader<EE6LTCH_A>;
#[doc = "External Event 6 latch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EE6LTCH_A {
    #[doc = "0: Event is ignored if it happens during a blank, or passed through during a window"]
    Disabled = 0,
    #[doc = "1: Event is latched and delayed till the end of the blanking or windowing period"]
    Enabled = 1,
}
impl From<EE6LTCH_A> for bool {
    #[inline(always)]
    fn from(variant: EE6LTCH_A) -> Self {
        variant as u8 != 0
    }
}
impl EE6LTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6LTCH_A {
        match self.bits {
            false => EE6LTCH_A::Disabled,
            true => EE6LTCH_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6LTCH_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE6LTCH_A::Enabled
    }
}
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub type EE6LTCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEFAR2_SPEC, EE6LTCH_A, O>;
impl<'a, const O: u8> EE6LTCH_W<'a, O> {
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6LTCH_A::Disabled)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EE6LTCH_A::Enabled)
    }
}
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub type EE6FLTR_R = crate::FieldReader<u8, EE6FLTR_A>;
#[doc = "External Event 6 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EE6FLTR_A {
    #[doc = "0: No filtering"]
    Disabled = 0,
    #[doc = "1: Blanking from counter reset/roll-over to Compare 1"]
    BlankResetToCompare1 = 1,
    #[doc = "2: Blanking from counter reset/roll-over to Compare 2"]
    BlankResetToCompare2 = 2,
    #[doc = "3: Blanking from counter reset/roll-over to Compare 3"]
    BlankResetToCompare3 = 3,
    #[doc = "4: Blanking from counter reset/roll-over to Compare 4"]
    BlankResetToCompare4 = 4,
    #[doc = "5: Blanking from another timing unit: TIMFLTR1 source"]
    BlankTimfltr1 = 5,
    #[doc = "6: Blanking from another timing unit: TIMFLTR2 source"]
    BlankTimfltr2 = 6,
    #[doc = "7: Blanking from another timing unit: TIMFLTR3 source"]
    BlankTimfltr3 = 7,
    #[doc = "8: Blanking from another timing unit: TIMFLTR4 source"]
    BlankTimfltr4 = 8,
    #[doc = "9: Blanking from another timing unit: TIMFLTR5 source"]
    BlankTimfltr5 = 9,
    #[doc = "10: Blanking from another timing unit: TIMFLTR6 source"]
    BlankTimfltr6 = 10,
    #[doc = "11: Blanking from another timing unit: TIMFLTR7 source"]
    BlankTimfltr7 = 11,
    #[doc = "12: Blanking from another timing unit: TIMFLTR8 source"]
    BlankTimfltr8 = 12,
    #[doc = "13: Windowing from counter reset/roll-over to compare 2"]
    WindowResetToCompare2 = 13,
    #[doc = "14: Windowing from counter reset/roll-over to compare 3"]
    WindowResetToCompare3 = 14,
    #[doc = "15: Windowing from another timing unit: TIMWIN source"]
    WindowTimwin = 15,
}
impl From<EE6FLTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EE6FLTR_A) -> Self {
        variant as _
    }
}
impl EE6FLTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EE6FLTR_A {
        match self.bits {
            0 => EE6FLTR_A::Disabled,
            1 => EE6FLTR_A::BlankResetToCompare1,
            2 => EE6FLTR_A::BlankResetToCompare2,
            3 => EE6FLTR_A::BlankResetToCompare3,
            4 => EE6FLTR_A::BlankResetToCompare4,
            5 => EE6FLTR_A::BlankTimfltr1,
            6 => EE6FLTR_A::BlankTimfltr2,
            7 => EE6FLTR_A::BlankTimfltr3,
            8 => EE6FLTR_A::BlankTimfltr4,
            9 => EE6FLTR_A::BlankTimfltr5,
            10 => EE6FLTR_A::BlankTimfltr6,
            11 => EE6FLTR_A::BlankTimfltr7,
            12 => EE6FLTR_A::BlankTimfltr8,
            13 => EE6FLTR_A::WindowResetToCompare2,
            14 => EE6FLTR_A::WindowResetToCompare3,
            15 => EE6FLTR_A::WindowTimwin,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6FLTR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `BlankResetToCompare1`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare1
    }
    #[doc = "Checks if the value of the field is `BlankResetToCompare2`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare2
    }
    #[doc = "Checks if the value of the field is `BlankResetToCompare3`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare3
    }
    #[doc = "Checks if the value of the field is `BlankResetToCompare4`"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE6FLTR_A::BlankResetToCompare4
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr1`"]
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr1
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr2`"]
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr2
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr3`"]
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr3
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr4`"]
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr4
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr5`"]
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr5
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr6`"]
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr6
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr7`"]
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr7
    }
    #[doc = "Checks if the value of the field is `BlankTimfltr8`"]
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE6FLTR_A::BlankTimfltr8
    }
    #[doc = "Checks if the value of the field is `WindowResetToCompare2`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR_A::WindowResetToCompare2
    }
    #[doc = "Checks if the value of the field is `WindowResetToCompare3`"]
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR_A::WindowResetToCompare3
    }
    #[doc = "Checks if the value of the field is `WindowTimwin`"]
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE6FLTR_A::WindowTimwin
    }
}
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub type EE6FLTR_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, EEFAR2_SPEC, u8, EE6FLTR_A, 4, O>;
impl<'a, const O: u8> EE6FLTR_W<'a, O> {
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EE6FLTR_A::Disabled)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankResetToCompare4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut W {
        self.variant(EE6FLTR_A::BlankTimfltr8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowResetToCompare2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowResetToCompare3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut W {
        self.variant(EE6FLTR_A::WindowTimwin)
    }
}
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub use EE6FLTR_R as EE7FLTR_R;
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub use EE6FLTR_R as EE8FLTR_R;
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub use EE6FLTR_R as EE9FLTR_R;
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub use EE6FLTR_R as EE10FLTR_R;
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub use EE6FLTR_W as EE7FLTR_W;
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub use EE6FLTR_W as EE8FLTR_W;
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub use EE6FLTR_W as EE9FLTR_W;
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub use EE6FLTR_W as EE10FLTR_W;
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub use EE6LTCH_R as EE7LTCH_R;
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub use EE6LTCH_R as EE8LTCH_R;
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub use EE6LTCH_R as EE9LTCH_R;
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub use EE6LTCH_R as EE10LTCH_R;
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub use EE6LTCH_W as EE7LTCH_W;
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub use EE6LTCH_W as EE8LTCH_W;
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub use EE6LTCH_W as EE9LTCH_W;
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub use EE6LTCH_W as EE10LTCH_W;
impl R {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<0> {
        EE6LTCH_W::new(self)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<1> {
        EE6FLTR_W::new(self)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<6> {
        EE7LTCH_W::new(self)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<7> {
        EE7FLTR_W::new(self)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<12> {
        EE8LTCH_W::new(self)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<13> {
        EE8FLTR_W::new(self)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<18> {
        EE9LTCH_W::new(self)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<19> {
        EE9FLTR_W::new(self)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<24> {
        EE10LTCH_W::new(self)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<25> {
        EE10FLTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefar2](index.html) module"]
pub struct EEFAR2_SPEC;
impl crate::RegisterSpec for EEFAR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eefar2::R](R) reader structure"]
impl crate::Readable for EEFAR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eefar2::W](W) writer structure"]
impl crate::Writable for EEFAR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EEFAR2 to value 0"]
impl crate::Resettable for EEFAR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
