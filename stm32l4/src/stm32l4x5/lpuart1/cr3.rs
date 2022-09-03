#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<EIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EIE_A {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    Enabled = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::Disabled,
            true => EIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE_A::Enabled
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, EIE_A, O>;
impl<'a, const O: u8> EIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::Disabled)
    }
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::Enabled)
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<HDSEL_A>;
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HDSEL_A {
    #[doc = "0: Half duplex mode is not selected"]
    NotSelected = 0,
    #[doc = "1: Half duplex mode is selected"]
    Selected = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::NotSelected,
            true => HDSEL_A::Selected,
        }
    }
    #[doc = "Checks if the value of the field is `NotSelected`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL_A::NotSelected
    }
    #[doc = "Checks if the value of the field is `Selected`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL_A::Selected
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, HDSEL_A, O>;
impl<'a, const O: u8> HDSEL_W<'a, O> {
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDSEL_A::NotSelected)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDSEL_A::Selected)
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<DMAR_A>;
#[doc = "DMA enable receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAR_A {
    #[doc = "0: DMA mode is disabled for reception"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    Enabled = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::Disabled,
            true => DMAR_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR_A::Enabled
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAR_A, O>;
impl<'a, const O: u8> DMAR_W<'a, O> {
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::Disabled)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::Enabled)
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<DMAT_A>;
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMAT_A {
    #[doc = "0: DMA mode is disabled for transmission"]
    Disabled = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    Enabled = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::Disabled,
            true => DMAT_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT_A::Enabled
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DMAT_A, O>;
impl<'a, const O: u8> DMAT_W<'a, O> {
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::Disabled)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::Enabled)
    }
}
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<RTSE_A>;
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSE_A {
    #[doc = "0: RTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer"]
    Enabled = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::Disabled,
            true => RTSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE_A::Enabled
    }
}
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, RTSE_A, O>;
impl<'a, const O: u8> RTSE_W<'a, O> {
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSE_A::Disabled)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSE_A::Enabled)
    }
}
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<CTSE_A>;
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSE_A {
    #[doc = "0: CTS hardware flow control disabled"]
    Disabled = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    Enabled = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::Disabled,
            true => CTSE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE_A::Enabled
    }
}
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, CTSE_A, O>;
impl<'a, const O: u8> CTSE_W<'a, O> {
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSE_A::Disabled)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSE_A::Enabled)
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSIE_A {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF=1 in the ISR register"]
    Enabled = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::Disabled,
            true => CTSIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE_A::Enabled
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, CTSIE_A, O>;
impl<'a, const O: u8> CTSIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::Disabled)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::Enabled)
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OVRDIS_R = crate::BitReader<OVRDIS_A>;
#[doc = "Overrun Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OVRDIS_A {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    Enabled = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    Disabled = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::Enabled,
            true => OVRDIS_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS_A::Enabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS_A::Disabled
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, OVRDIS_A, O>;
impl<'a, const O: u8> OVRDIS_W<'a, O> {
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::Enabled)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::Disabled)
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DDRE_R = crate::BitReader<DDRE_A>;
#[doc = "DMA Disable on Reception Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DDRE_A {
    #[doc = "0: DMA is not disabled in case of reception error"]
    NotDisabled = 0,
    #[doc = "1: DMA is disabled following a reception error"]
    Disabled = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl DDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NotDisabled,
            true => DDRE_A::Disabled,
        }
    }
    #[doc = "Checks if the value of the field is `NotDisabled`"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE_A::NotDisabled
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE_A::Disabled
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DDRE_A, O>;
impl<'a, const O: u8> DDRE_W<'a, O> {
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NotDisabled)
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::Disabled)
    }
}
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader<DEM_A>;
#[doc = "Driver enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEM_A {
    #[doc = "0: DE function is disabled"]
    Disabled = 0,
    #[doc = "1: The DE signal is output on the RTS pin"]
    Enabled = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
impl DEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::Disabled,
            true => DEM_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM_A::Enabled
    }
}
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DEM_A, O>;
impl<'a, const O: u8> DEM_W<'a, O> {
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::Disabled)
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::Enabled)
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DEP_R = crate::BitReader<DEP_A>;
#[doc = "Driver enable polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEP_A {
    #[doc = "0: DE signal is active high"]
    High = 0,
    #[doc = "1: DE signal is active low"]
    Low = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::High,
            true => DEP_A::Low,
        }
    }
    #[doc = "Checks if the value of the field is `High`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP_A::High
    }
    #[doc = "Checks if the value of the field is `Low`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP_A::Low
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, DEP_A, O>;
impl<'a, const O: u8> DEP_W<'a, O> {
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::High)
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::Low)
    }
}
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_R = crate::FieldReader<u8, WUS_A>;
#[doc = "Wakeup from Stop mode interrupt flag selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WUS_A {
    #[doc = "0: WUF active on address match"]
    Address = 0,
    #[doc = "2: WuF active on Start bit detection"]
    Start = 2,
    #[doc = "3: WUF active on RXNE"]
    Rxne = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
impl WUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::Address),
            2 => Some(WUS_A::Start),
            3 => Some(WUS_A::Rxne),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `Address`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS_A::Address
    }
    #[doc = "Checks if the value of the field is `Start`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS_A::Start
    }
    #[doc = "Checks if the value of the field is `Rxne`"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS_A::Rxne
    }
}
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, WUS_A, 2, O>;
impl<'a, const O: u8> WUS_W<'a, O> {
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUS_A::Address)
    }
    #[doc = "WuF active on Start bit detection"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUS_A::Start)
    }
    #[doc = "WUF active on RXNE"]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUS_A::Rxne)
    }
}
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_R = crate::BitReader<WUFIE_A>;
#[doc = "Wakeup from Stop mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WUFIE_A {
    #[doc = "0: Interrupt is inhibited"]
    Disabled = 0,
    #[doc = "1: An USART interrupt is generated whenever WUF=1 in the ISR register"]
    Enabled = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::Disabled,
            true => WUFIE_A::Enabled,
        }
    }
    #[doc = "Checks if the value of the field is `Disabled`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE_A::Disabled
    }
    #[doc = "Checks if the value of the field is `Enabled`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE_A::Enabled
    }
}
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, WUFIE_A, O>;
impl<'a, const O: u8> WUFIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIE_A::Disabled)
    }
    #[doc = "An USART interrupt is generated whenever WUF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIE_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}