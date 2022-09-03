#[doc = "Register `C2CR3` reader"]
pub struct R(crate::R<C2CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2CR3` writer"]
pub struct W(crate::W<C2CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2CR3_SPEC>;
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
impl From<crate::W<C2CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWUP1` reader - Enable Wakeup pin WKUP1 for CPU2"]
pub type EWUP1_R = crate::BitReader<EWUP1_A>;
#[doc = "Enable Wakeup pin WKUP1 for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP1_A {
    #[doc = "0: WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP1_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP1_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP1_A {
        match self.bits {
            false => EWUP1_A::Disabled,
            true => EWUP1_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP1_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP1_A::Enabled
    }
}
#[doc = "Field `EWUP1` writer - Enable Wakeup pin WKUP1 for CPU2"]
pub type EWUP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP1_A, O>;
impl<'a, const O: u8> EWUP1_W<'a, O> {
    #[doc = "WKUP pin 1 is used for general purpose I/Os. An event on the WKUP pin 1 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Disabled)
    }
    #[doc = "WKUP pin 1 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 1 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP1_A::Enabled)
    }
}
#[doc = "Field `EWUP2` reader - Enable Wakeup pin WKUP2 for CPU2"]
pub type EWUP2_R = crate::BitReader<EWUP2_A>;
#[doc = "Enable Wakeup pin WKUP2 for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP2_A {
    #[doc = "0: WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP2_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP2_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP2_A {
        match self.bits {
            false => EWUP2_A::Disabled,
            true => EWUP2_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP2_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP2_A::Enabled
    }
}
#[doc = "Field `EWUP2` writer - Enable Wakeup pin WKUP2 for CPU2"]
pub type EWUP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP2_A, O>;
impl<'a, const O: u8> EWUP2_W<'a, O> {
    #[doc = "WKUP pin 2 is used for general purpose I/Os. An event on the WKUP pin 2 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Disabled)
    }
    #[doc = "WKUP pin 2 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 2 wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP2_A::Enabled)
    }
}
#[doc = "Field `EWUP3` reader - Enable Wakeup pin WKUP3 for CPU2"]
pub type EWUP3_R = crate::BitReader<EWUP3_A>;
#[doc = "Enable Wakeup pin WKUP3 for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWUP3_A {
    #[doc = "0: WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    Disabled = 0,
    #[doc = "1: WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    Enabled = 1,
}
impl From<EWUP3_A> for bool {
    #[inline(always)]
    fn from(variant: EWUP3_A) -> Self {
        variant as u8 != 0
    }
}
impl EWUP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWUP3_A {
        match self.bits {
            false => EWUP3_A::Disabled,
            true => EWUP3_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWUP3_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWUP3_A::Enabled
    }
}
#[doc = "Field `EWUP3` writer - Enable Wakeup pin WKUP3 for CPU2"]
pub type EWUP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWUP3_A, O>;
impl<'a, const O: u8> EWUP3_W<'a, O> {
    #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Disabled)
    }
    #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWUP3_A::Enabled)
    }
}
#[doc = "Field `EWPVD` reader - Enable wakeup PVD for CPU2"]
pub type EWPVD_R = crate::BitReader<EWPVD_A>;
#[doc = "Enable wakeup PVD for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWPVD_A {
    #[doc = "0: PVD not enabled by the sub-GHz radio active state"]
    Disabled = 0,
    #[doc = "1: PVD enabled while the sub-GHz radio is active"]
    Enabled = 1,
}
impl From<EWPVD_A> for bool {
    #[inline(always)]
    fn from(variant: EWPVD_A) -> Self {
        variant as u8 != 0
    }
}
impl EWPVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWPVD_A {
        match self.bits {
            false => EWPVD_A::Disabled,
            true => EWPVD_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWPVD_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWPVD_A::Enabled
    }
}
#[doc = "Field `EWPVD` writer - Enable wakeup PVD for CPU2"]
pub type EWPVD_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWPVD_A, O>;
impl<'a, const O: u8> EWPVD_W<'a, O> {
    #[doc = "PVD not enabled by the sub-GHz radio active state"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWPVD_A::Disabled)
    }
    #[doc = "PVD enabled while the sub-GHz radio is active"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWPVD_A::Enabled)
    }
}
#[doc = "Field `APC` reader - Apply pull-up and pull-down configuration for CPU2"]
pub type APC_R = crate::BitReader<APC_A>;
#[doc = "Apply pull-up and pull-down configuration for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum APC_A {
    #[doc = "0: I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied"]
    Disabled = 0,
    #[doc = "1: PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os"]
    Enabled = 1,
}
impl From<APC_A> for bool {
    #[inline(always)]
    fn from(variant: APC_A) -> Self {
        variant as u8 != 0
    }
}
impl APC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> APC_A {
        match self.bits {
            false => APC_A::Disabled,
            true => APC_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == APC_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == APC_A::Enabled
    }
}
#[doc = "Field `APC` writer - Apply pull-up and pull-down configuration for CPU2"]
pub type APC_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, APC_A, O>;
impl<'a, const O: u8> APC_W<'a, O> {
    #[doc = "I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(APC_A::Disabled)
    }
    #[doc = "PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(APC_A::Enabled)
    }
}
#[doc = "Field `EWRFBUSY` reader - EWRFBUSY"]
pub type EWRFBUSY_R = crate::BitReader<EWRFBUSY_A>;
#[doc = "EWRFBUSY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFBUSY_A {
    #[doc = "0: Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs"]
    Disabled = 0,
    #[doc = "1: Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4"]
    Enabled = 1,
}
impl From<EWRFBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFBUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRFBUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRFBUSY_A {
        match self.bits {
            false => EWRFBUSY_A::Disabled,
            true => EWRFBUSY_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFBUSY_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFBUSY_A::Enabled
    }
}
#[doc = "Field `EWRFBUSY` writer - EWRFBUSY"]
pub type EWRFBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWRFBUSY_A, O>;
impl<'a, const O: u8> EWRFBUSY_W<'a, O> {
    #[doc = "Radio Busy is disabled and does not trigger a wakeup from Standby event to CPU2 when a rising or a falling edge occurs"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::Disabled)
    }
    #[doc = "Radio Busy is enabled and triggers a wakeup from Standby event to CPU2 when a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFBUSY_A::Enabled)
    }
}
#[doc = "Field `EWRFIRQ` reader - akeup for CPU2"]
pub type EWRFIRQ_R = crate::BitReader<EWRFIRQ_A>;
#[doc = "akeup for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EWRFIRQ_A {
    #[doc = "0: Radio IRQ\\[2:0\\]
is disabled and does not trigger a wakeup from Standby event to CPU2."]
    Disabled = 0,
    #[doc = "1: Radio IRQ\\[2:0\\]
is enabled and triggers a wakeup from Standby event to CPU2."]
    Enabled = 1,
}
impl From<EWRFIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: EWRFIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl EWRFIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWRFIRQ_A {
        match self.bits {
            false => EWRFIRQ_A::Disabled,
            true => EWRFIRQ_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EWRFIRQ_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EWRFIRQ_A::Enabled
    }
}
#[doc = "Field `EWRFIRQ` writer - akeup for CPU2"]
pub type EWRFIRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EWRFIRQ_A, O>;
impl<'a, const O: u8> EWRFIRQ_W<'a, O> {
    #[doc = "Radio IRQ\\[2:0\\]
is disabled and does not trigger a wakeup from Standby event to CPU2."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::Disabled)
    }
    #[doc = "Radio IRQ\\[2:0\\]
is enabled and triggers a wakeup from Standby event to CPU2."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EWRFIRQ_A::Enabled)
    }
}
#[doc = "Field `EIWUL` reader - Enable internal wakeup line for CPU2"]
pub type EIWUL_R = crate::BitReader<EIWUL_A>;
#[doc = "Enable internal wakeup line for CPU2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIWUL_A {
    #[doc = "0: Internal wakeup line interrupt to CPU2 disabled"]
    Disabled = 0,
    #[doc = "1: Internal wakeup line interrupt to CPU2 enabled"]
    Enabled = 1,
}
impl From<EIWUL_A> for bool {
    #[inline(always)]
    fn from(variant: EIWUL_A) -> Self {
        variant as u8 != 0
    }
}
impl EIWUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIWUL_A {
        match self.bits {
            false => EIWUL_A::Disabled,
            true => EIWUL_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIWUL_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIWUL_A::Enabled
    }
}
#[doc = "Field `EIWUL` writer - Enable internal wakeup line for CPU2"]
pub type EIWUL_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2CR3_SPEC, EIWUL_A, O>;
impl<'a, const O: u8> EIWUL_W<'a, O> {
    #[doc = "Internal wakeup line interrupt to CPU2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Disabled)
    }
    #[doc = "Internal wakeup line interrupt to CPU2 enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIWUL_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU2"]
    #[inline(always)]
    pub fn ewup1(&self) -> EWUP1_R {
        EWUP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU2"]
    #[inline(always)]
    pub fn ewup2(&self) -> EWUP2_R {
        EWUP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU2"]
    #[inline(always)]
    pub fn ewup3(&self) -> EWUP3_R {
        EWUP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU2"]
    #[inline(always)]
    pub fn ewpvd(&self) -> EWPVD_R {
        EWPVD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration for CPU2"]
    #[inline(always)]
    pub fn apc(&self) -> APC_R {
        APC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - EWRFBUSY"]
    #[inline(always)]
    pub fn ewrfbusy(&self) -> EWRFBUSY_R {
        EWRFBUSY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - akeup for CPU2"]
    #[inline(always)]
    pub fn ewrfirq(&self) -> EWRFIRQ_R {
        EWRFIRQ_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable internal wakeup line for CPU2"]
    #[inline(always)]
    pub fn eiwul(&self) -> EIWUL_R {
        EIWUL_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Wakeup pin WKUP1 for CPU2"]
    #[inline(always)]
    pub fn ewup1(&mut self) -> EWUP1_W<0> {
        EWUP1_W::new(self)
    }
    #[doc = "Bit 1 - Enable Wakeup pin WKUP2 for CPU2"]
    #[inline(always)]
    pub fn ewup2(&mut self) -> EWUP2_W<1> {
        EWUP2_W::new(self)
    }
    #[doc = "Bit 2 - Enable Wakeup pin WKUP3 for CPU2"]
    #[inline(always)]
    pub fn ewup3(&mut self) -> EWUP3_W<2> {
        EWUP3_W::new(self)
    }
    #[doc = "Bit 8 - Enable wakeup PVD for CPU2"]
    #[inline(always)]
    pub fn ewpvd(&mut self) -> EWPVD_W<8> {
        EWPVD_W::new(self)
    }
    #[doc = "Bit 10 - Apply pull-up and pull-down configuration for CPU2"]
    #[inline(always)]
    pub fn apc(&mut self) -> APC_W<10> {
        APC_W::new(self)
    }
    #[doc = "Bit 11 - EWRFBUSY"]
    #[inline(always)]
    pub fn ewrfbusy(&mut self) -> EWRFBUSY_W<11> {
        EWRFBUSY_W::new(self)
    }
    #[doc = "Bit 13 - akeup for CPU2"]
    #[inline(always)]
    pub fn ewrfirq(&mut self) -> EWRFIRQ_W<13> {
        EWRFIRQ_W::new(self)
    }
    #[doc = "Bit 15 - Enable internal wakeup line for CPU2"]
    #[inline(always)]
    pub fn eiwul(&mut self) -> EIWUL_W<15> {
        EIWUL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power CPU2 control register 3 \\[dual core device only\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr3](index.html) module"]
pub struct C2CR3_SPEC;
impl crate::RegisterSpec for C2CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c2cr3::R](R) reader structure"]
impl crate::Readable for C2CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2cr3::W](W) writer structure"]
impl crate::Writable for C2CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C2CR3 to value 0x8000"]
impl crate::Resettable for C2CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
