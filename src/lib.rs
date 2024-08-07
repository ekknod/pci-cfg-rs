pub mod config {
pub struct Pci
{
    cfg: [u8; 0x1000],
}

impl std::fmt::Debug for Pci {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pci")
         .field("vendor_id", &self.vendor_id())
         .field("device_id", &self.device_id())
         .field("subsystem_vendor_id", &self.subsystem_vendor_id())
         .field("subsystem_device_id", &self.subsystem_device_id())
         .field("command", &self.command())
         .field("status", &self.status())
         .field("header_type", &self.header())
         .field("interrupt_line", &self.interrupt_line())
         .field("interrupt_pin", &self.interrupt_pin())
         .field("capabilities_ptr", &self.capabilities_ptr())
         .field("bus_number", &self.bus_number())
         .field("secondary_bus", &self.secondary_bus())
         .field("subordinate_bus", &self.subordinate_bus())
         .field("class_code", &self.class_code())
         .finish()
    }
}

impl Pci {
    pub const MAX_CAPABILITIES: u8 = 0x16;
    pub const MAX_EXTENDED_CAPABILITIES: u8 = 0x2F;

    fn read_field<T: Clone>(buffer: &[u8], offset: isize) -> T
    {
        let ptr = unsafe { buffer.as_ptr().offset(offset) as *const T };
        unsafe { (*ptr).clone() }
    }

    pub fn read<T: Clone>(&self, offset: isize) -> T
    {
        return Pci::read_field(&self.cfg, offset)
    }

    pub fn new(cfg: &[u8]) -> Self
    {
        let mut cfg_buffer: [u8; 0x1000] = [0; 0x1000];
        let size = std::mem::size_of_val(cfg);
        cfg_buffer[..size].copy_from_slice(&cfg[..size]);
        Self{
            cfg : cfg_buffer
        }
    }

    pub fn vendor_id(&self) -> u16 { self.read::<u16>(0x00) }
    pub fn device_id(&self) -> u16 { self.read::<u16>(0x02) }

    pub fn subsystem_vendor_id(&self) -> u16 { self.read::<u16>(0x2C) }
    pub fn subsystem_device_id(&self) -> u16 { self.read::<u16>(0x2E) }

    pub fn command(&self) -> fld::Command { fld::Command(self.read::<u16>(0x04)) }
    pub fn status(&self)  -> fld::Status  { fld::Status(self.read::<u16>(0x06)) }
    pub fn header(&self)  -> fld::HeaderType { fld::HeaderType(self.read::<u8>(0x0E)) }

    pub fn bus_number(&self) -> u8 {
        if self.header().header_type() == 0 {
            return 0;
        }
        return self.read::<u8>(0x18);
    }

    pub fn secondary_bus(&self) -> u8 {
        if self.header().header_type() == 0 {
            return 0;
        }
        return self.read::<u8>(0x19);
    }

    pub fn subordinate_bus(&self) -> u8 {
        if self.header().header_type() == 0 {
            return 0;
        }
        return self.read::<u8>(0x1A);
    }

    pub fn revision_id(&self) -> u8 { return self.read::<u8>(0x08); }

    pub fn class_code(&self) -> u32 {
        let a0 = self.read::<u8>(0x09 + 0) as u32;
        let a1 = self.read::<u8>(0x09 + 1) as u32;
        let a2 = self.read::<u8>(0x09 + 2) as u32;
        return (a2 << 16) | (a1 << 8) | a0
    }

    pub fn interrupt_line(&self) -> u8 { return self.read::<u8>(0x3C); }
    pub fn interrupt_pin(&self) -> u8 { return self.read::<u8>(0x3D); }
    pub fn capabilities_ptr(&self) -> u8 { return self.read::<u8>(0x34); }

    pub fn get_capability_by_id(&self, id: u8) -> u8
    {
        let mut off = self.capabilities_ptr();
        if off == 0 {
            return 0;
        }
        loop {
            let cap = fld::CapHdr(self.read::<u16>(off as isize));

            if cap.0 == 0 {
                break;
            }

            if cap.cap_id() == id
            {
                return off;
            }

            let next = cap.cap_next_ptr();
            if next == 0
            {
                break;
            }
            off = next as u8;
        }
        return 0;
    }

    pub fn get_ext_capability_by_id(&self, id: u8) -> u16
    {
        let mut off = 0x100 as u16;
        loop {
            let cap = fld::CapExtHdr(self.read::<u32>(off as isize));

            if cap.0 == 0 {
                break;
            }

            if cap.cap_id() == id
            {
                return off;
            }

            let next = cap.cap_next_ptr();
            if next == 0
            {
                break;
            }
            off = next as u16;
        }
        return 0;
    }

    pub fn get_pm(&self) -> cap::PM {
        let cap = self.get_capability_by_id(0x01);
        if cap != 0 {
            let val = self.read::<u64>(cap as isize);
            return cap::PM{
                cap_on: val != 0,
                base_ptr: cap,
                hdr: fld::CapHdr((val & 0xFFFF) as u16),
                cap: fld::PmCap(((val >> 16) & 0xFFFF) as u16),
                csr: fld::PmCsr(((val >> 32) & 0xFFFF) as u16),
            };
        }
        return cap::PM::new();
    }

    pub fn get_msi(&self) -> cap::MSI {
        let cap = self.get_capability_by_id(0x05);
        if cap != 0 {
            let val = self.read::<u32>(cap as isize);
            return cap::MSI{
                cap_on: val != 0,
                base_ptr: cap,
                hdr: fld::CapHdr((val & 0xFFFF) as u16),
                cap: fld::MsiCap(((val >> 16) & 0xFFFF) as u16)
            };
        }
        return cap::MSI::new();
    }

    pub fn get_msix(&self) -> cap::MSIX {
        let cap = self.get_capability_by_id(0x11);
        if cap != 0 {
            let val = self.read::<u32>(cap as isize);
            return cap::MSIX{
                cap_on: val != 0,
                base_ptr: cap,
                hdr: fld::CapHdr((val & 0xFFFF) as u16),
                cap: fld::MsixCap(((val >> 16) & 0xFFFF) as u16)
            };
        }
        return cap::MSIX::new();
    }

    pub fn get_pci(&self) -> cap::PCIE {
        let cap = self.get_capability_by_id(0x10);
        if cap != 0 {
            let pci = self.read::<u32>(cap as isize);
            let dev = self.read::<u64>(cap as isize + 0x04);
            let dev2 = self.read::<u64>(cap as isize + 0x04 + 0x20);
            let link = self.read::<u64>(cap as isize + 0x0C);
            let link2 = self.read::<u64>(cap as isize + 0x0C + 0x20);
            return cap::PCIE{
                cap_on: pci != 0,
                base_ptr: cap,
                hdr: fld::CapHdr((pci & 0xFFFF) as u16),
                cap: fld::PciCap(((pci >> 16) & 0xFFFF) as u16),
                dev: cap::DEV{
                    cap: fld::DevCap((dev & 0xFFFFFFFF) as u32),
                    control: fld::DevControl(((dev >> 32) & 0xFFFF) as u16),
                    status: fld::DevStatus(((dev >> 48) & 0xFFFF) as u16),
                },
                dev2: cap::DEV2{
                    cap: fld::DevCap2((dev2 & 0xFFFFFFFF) as u32),
                    control: fld::DevControl2(((dev2 >> 32) & 0xFFFF) as u16),
                    status: fld::DevStatus2(((dev2 >> 48) & 0xFFFF) as u16),
                },
                link: cap::LINK{
                    cap: fld::LinkCap((link & 0xFFFFFFFF) as u32),
                    control: fld::LinkControl(((link >> 32) & 0xFFFF) as u16),
                    status: fld::LinkStatus(((link >> 48) & 0xFFFF) as u16),
                },
                link2: cap::LINK2{
                    cap: fld::LinkCap2((link2 & 0xFFFFFFFF) as u32),
                    control: fld::LinkControl2(((link2 >> 32) & 0xFFFF) as u16),
                    status: fld::LinkStatus2(((link2 >> 48) & 0xFFFF) as u16),
                }
            };
        }
        return cap::PCIE::new();
    }

    pub fn get_dsn(&self) -> cap::DSN {
        let cap = self.get_ext_capability_by_id(0x03);
        if cap != 0 {
            let hdr = self.read::<u32>(cap as isize);
            return cap::DSN {
                cap_on: hdr != 0,
                base_ptr: cap,
                hdr: fld::CapExtHdr(hdr),
                serial: self.read::<u64>(cap as isize + 4)
            };
        }
        return cap::DSN::new();
    }

    pub fn get_empty_extended_cap(&self, id: u8) -> cap::EmptyExtPcieCap {
        let cap = self.get_ext_capability_by_id(id);
        if cap != 0 {
            let val = self.read::<u32>(cap as isize);
            return cap::EmptyExtPcieCap{
                cap_on: val != 0,
                base_ptr: cap,
                hdr: fld::CapExtHdr(val)
            };
        }
        return cap::EmptyExtPcieCap::new();
    }
}

pub mod fld {

    bitfield::bitfield!{
        pub struct CapHdr(u16);
        impl Debug;
        u8;
        pub cap_id, _: 7, 0;
        pub cap_next_ptr, _: 15, 8;
    }

    bitfield::bitfield!{
        pub struct CapExtHdr(u32);
        impl Debug;
        u8;
        pub cap_id, _: 7, 0;
        u16;
        pub cap_next_ptr, _: 31, 20;
    }

    bitfield::bitfield!{
        pub struct PmCap(u16);
        impl Debug;
        u8;
        pub pm_cap_version, _: 2, 0;
        pub pm_cap_pme_clock, _: 3;
        pub pm_cap_dsi, _: 5;
        pub pm_cap_auxcurrent, _: 8, 6;
        pub pm_cap_d1support, _: 9;
        pub pm_cap_d2support, _: 10;
        pub pm_cap_pmesupport, _: 15, 11;
    }

    bitfield::bitfield!{
        pub struct PmCsr(u16);
        impl Debug;
        u8;
        pub pm_csr_power_state, _: 1,0;
        pub pm_csr_nosoftrst, _: 3;
        pub pm_csr_dynamic_data, _: 4;
        pub pm_csr_pme_enabled, _: 8;
        pub pm_csr_data_select, _: 12,9;
        pub pm_csr_data_scale, _: 14,13;
        pub pm_csr_pme_status, _: 15;
    }

    bitfield::bitfield!{
        pub struct MsiCap(u16);
        impl Debug;
        u8;
        pub msi_cap_enabled, _:  0;
        pub msi_cap_multimsgcap, _:  3, 1;
        pub msi_cap_multimsg_extension, _:  6, 4;
        pub msi_cap_64_bit_addr_capable, _:  7;
        pub msi_cap_per_vector_masking_capable, _:  8;
    }

    bitfield::bitfield!{
        pub struct MsixCap(u16);
        impl Debug;
        u8;
        pub msix_cap_enabled, _:  15;
    }

    bitfield::bitfield!{
        pub struct PciCap(u16);
        impl Debug;
        u8;
        pub pcie_cap_capability_version, _: 3, 0;
        pub pcie_cap_device_port_type, _: 7, 4;
        pub pcie_cap_slot_implemented, _: 8;
        pub pcie_cap_interrupt_message_number, _: 13, 9;
    }

    bitfield::bitfield!{
        pub struct DevCap(u32);
        impl Debug;
        u8;
        pub dev_cap_max_payload_supported, _: 2, 0;
        pub dev_cap_phantom_functions_support, _: 4, 3;
        pub dev_cap_ext_tag_supported, _: 5;
        pub dev_cap_endpoint_l0s_latency, _: 8, 6;
        pub dev_cap_endpoint_l1_latency, _: 11, 9;
        pub dev_cap_role_based_error, _: 15;
        pub dev_cap_enable_slot_pwr_limit_value, _: 25, 18;
        pub dev_cap_enable_slot_pwr_limit_scale, _: 27, 26;
        pub dev_cap_function_level_reset_capable, _: 28;
    }

    bitfield::bitfield!{
        pub struct DevCap2(u32);
        impl Debug;
        u8;
        pub cpl_timeout_ranges_supported, _: 3, 0;
        pub cpl_timeout_disable_supported, _: 4;
    }

    bitfield::bitfield!{
        pub struct LinkCap(u32);
        impl Debug;
        u8;
        pub link_cap_max_link_speed, _:  3, 0;
        pub link_cap_max_link_width, _:  9, 4;
        pub link_cap_aspm_support, _: 11, 10;
        pub link_cap_l0s_exit_latency, _: 14, 12;
        pub link_cap_l1_exit_latency, _: 17, 15;
        pub link_cap_clock_power_management, _: 19, 18;
        pub link_cap_aspm_optionality, _: 22;
        pub link_cap_rsvd_23, _: 23, 19;
    }

    bitfield::bitfield!{
        pub struct LinkCap2(u32);
        impl Debug;
        u8;
        pub link_cap2_linkspeedssupported, _:  3, 1;
    }

    bitfield::bitfield!{
        pub struct Command(u16);
        impl Debug;
        pub memory_space_enable, _: 1;
        pub bus_master_enable, _: 2;
        pub special_cycle_enable, _: 3;
        pub memory_write, _: 4;
        pub vga_enable, _: 5;
        pub parity_err_enable, _: 6;
        pub serr_enable, _: 8;
        pub b2b_enable, _: 9;
        pub interrupt_disable, _: 10;
    }

    bitfield::bitfield!{
        pub struct Status(u16);
        impl Debug;
        pub parity_error, _: 15;
        pub signaled_error, _: 14;
        pub master_abort, _: 13;
        pub target_abort, _: 12;
        pub signaled_abort, _: 11;
        pub devsel_timing, _: 10, 9;
        pub master_parity_error, _: 8;
        pub fast_b2b_capable, _: 7;
        pub c66_capable, _: 5;
        pub capabilities_list, _: 4;
        pub interrupt_status, _: 3;
    }

    bitfield::bitfield!{
        pub struct HeaderType(u8);
        impl Debug;
        pub multifunc_device, _: 7;
        pub header_type, _: 6, 0;
    }

    bitfield::bitfield!{
        pub struct DevControl(u16);
        impl Debug;
        u8;
        dev_ctrl_corr_err_reporting, _: 0;
        dev_ctrl_non_fatal_reporting, _: 1;
        dev_ctrl_fatal_err_reporting, _: 2;
        dev_ctrl_ur_reporting, _: 3;
        dev_ctrl_relaxed_ordering, _: 4;
        dev_ctrl_max_payload_size, _: 7, 5;
        dev_ctrl_ext_tag_default, _: 8;
        dev_ctrl_phantom_func_enable, _: 9;
        dev_ctrl_aux_power_enable, _: 10;
        dev_ctrl_enable_no_snoop, _: 11;
        dev_ctrl_max_read_request_size, _: 14, 12;
        dev_ctrl_cfg_retry_status_enable, _: 15;
    }

    bitfield::bitfield!{
        pub struct DevStatus(u16);
        impl Debug;
        u8;
        correctable_error_detected, _: 0;
        non_fatal_error_detected, _: 1;
        fatal_error_detected, _: 2;
        unsupported_request_detected, _: 3;
        aux_power_detected, _: 4;
        transactions_pending, _: 5;
    }

    bitfield::bitfield!{
        pub struct DevControl2(u16);
        impl Debug;
        u8;
        obff_enable, _: 0;
        latency_tolerance_reporting, _: 1;
        completion_timeout_disable, _: 2;
        completion_timeout_value, _: 3;
    }

    bitfield::bitfield!{
        pub struct DevStatus2(u16);
        impl Debug;
        u8;
        correctable_error_detected, _: 0;
        non_fatal_error_detected, _: 1;
        fatal_error_detected, _: 2;
        unsupported_request_detected, _: 3;
        aux_power_detected, _: 4;
        transactions_pending, _: 5;
    }

    bitfield::bitfield!{
        pub struct LinkStatus(u16);
        impl Debug;
        u8;
        link_status_link_speed, _: 3, 0;
        link_status_link_width, _: 9, 4;
        link_status_slot_clock_config, _: 12;
    }

    bitfield::bitfield!{
        pub struct LinkControl(u16);
        impl Debug;
        u8;
        link_aspmc, _: 1;
        link_control_rcb, _: 3;
        link_common_control_configuration, _: 6;
        link_extended_synch, _: 7;
        link_enable_clock_power_management, _: 8;
        link_hardware_autonomous_width_disable, _: 9;
    }

    bitfield::bitfield!{
        pub struct LinkControl2(u16);
        impl Debug;
        u8;
        pub link_ctrl2_target_link_speed, _: 3, 0;
        pub link_ctrl2_entercompliance, _: 4;
        pub link_ctrl2_hw_autonomous_speed_disable, _: 5;
        pub link_ctrl2_deemphasis, _: 6;
        pub link_ctrl2_transmitmargin, _: 7;
        pub link_ctrl2_entermodifiedcompliance, _: 10;
        pub link_ctrl2_compliancesos, _: 11;
    }

    bitfield::bitfield!{
        pub struct LinkStatus2(u16);
        impl Debug;
        u8;
        pub link_status2_deemphasislvl, _: 0;
    }

}

mod cap
{
    use crate::config::fld;

    #[derive(Debug)]
    pub struct PM {
        pub cap_on: bool,
        pub base_ptr: u8,
        pub hdr : fld::CapHdr,
        pub cap : fld::PmCap,
        pub csr : fld::PmCsr
    }
    impl PM { pub fn new() -> Self { Self {cap_on: false, base_ptr: 0, hdr: fld::CapHdr(0), cap: fld::PmCap(0), csr: fld::PmCsr(0)} } }

    #[derive(Debug)]
    pub struct MSI {
        pub cap_on: bool,
        pub base_ptr: u8,
        pub hdr : fld::CapHdr,
        pub cap : fld::MsiCap
    }
    impl MSI { pub fn new() -> Self { Self{ cap_on: false, base_ptr: 0, hdr: fld::CapHdr(0), cap: fld::MsiCap(0) } } }

    #[derive(Debug)]
    pub struct MSIX {
        pub cap_on: bool,
        pub base_ptr: u8,
        pub hdr : fld::CapHdr,
        pub cap : fld::MsixCap
    }
    impl MSIX { pub fn new() -> Self { Self{ cap_on: false, base_ptr: 0, hdr: fld::CapHdr(0), cap: fld::MsixCap(0) } } }

    #[derive(Debug)]
    pub struct DEV {
        pub cap : fld::DevCap,
        pub control : fld::DevControl,
        pub status : fld::DevStatus,
    }
    impl DEV { pub fn new() -> Self { Self { cap: fld::DevCap(0), control: fld::DevControl(0), status: fld::DevStatus(0) } } }

    #[derive(Debug)]
    pub struct DEV2 {
        pub cap : fld::DevCap2,
        pub control : fld::DevControl2,
        pub status : fld::DevStatus2,
    }
    impl DEV2 { pub fn new() -> Self { Self { cap: fld::DevCap2(0), control: fld::DevControl2(0), status: fld::DevStatus2(0) } } }

    #[derive(Debug)]
    pub struct LINK {
        pub cap : fld::LinkCap,
        pub control : fld::LinkControl,
        pub status : fld::LinkStatus,
    }
    impl LINK { pub fn new() -> Self { Self { cap: fld::LinkCap(0), control: fld::LinkControl(0), status: fld::LinkStatus(0) } } }

    #[derive(Debug)]
    pub struct LINK2 {
        pub cap : fld::LinkCap2,
        pub control : fld::LinkControl2,
        pub status : fld::LinkStatus2,
    }
    impl LINK2 { pub fn new() -> Self { Self { cap: fld::LinkCap2(0), control: fld::LinkControl2(0), status: fld::LinkStatus2(0) } } }

    #[derive(Debug)]
    pub struct PCIE {
        pub cap_on: bool,
        pub base_ptr: u8,
        pub hdr : fld::CapHdr,
        pub cap : fld::PciCap,
        pub dev : DEV,
        pub dev2 : DEV2,
        pub link : LINK,
        pub link2 : LINK2,
    }
    impl PCIE { pub fn new() -> Self { Self { cap_on: false, base_ptr: 0, hdr: fld::CapHdr(0), cap: fld::PciCap(0),
        dev: DEV::new(), dev2: DEV2::new(), link: LINK::new(), link2: LINK2::new() } } }

    #[derive(Debug)]
    pub struct DSN {
        pub cap_on : bool,
        pub base_ptr : u16,
        pub hdr : fld::CapExtHdr,
        pub serial : u64
    }
    impl DSN {pub fn new() -> Self { Self { cap_on: false, base_ptr: 0, hdr: fld::CapExtHdr(0), serial: 0 } }}

    #[derive(Debug)]
    pub struct EmptyExtPcieCap {
        pub cap_on : bool,
        pub base_ptr : u16,
        pub hdr : fld::CapExtHdr,
    }
    impl EmptyExtPcieCap {pub fn new() -> Self { Self {cap_on:false, base_ptr:0, hdr: fld::CapExtHdr(0)}}}
}

}

