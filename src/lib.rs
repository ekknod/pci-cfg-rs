pub mod config {

#[derive(Debug)]
pub struct Pci
{
    cfg: [u8; 0x1000],
    pub vendor_id: u16,
    pub device_id: u16,
    pub subsystem_vendor_id: u16,
    pub subsystem_device_id: u16,
    pub command: u16,
    pub status: u16,
    pub header_type: u8,
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub capabilities_ptr: u8,
    pub bus_number: u8,
    pub secondary_bus: u8,
    pub subordinate_bus: u8,
    pub class_code: u32,
}

pub fn get_bit(value: u32, bit_index: usize) -> bool {
    return (value >> bit_index) & 1 != 0;
}

pub fn get_bits(value: u32, end_bit: usize, start_bit: usize) -> u32 {
    let mask = ((1 << (end_bit - start_bit + 1)) - 1) << start_bit;
    return (value & mask) >> start_bit;
}

pub mod cap {
    use crate::config::get_bit;
    use crate::config::get_bits;

    #[derive(Debug)]
    pub struct PmCap {
        pub pm_cap_on:           u8,
        pub pm_cap_next_ptr:     u8,
        pub pm_base_ptr:         u8,
        pub pm_cap_id:           u8,
        pub pm_cap_version:      u8,
        pub pm_cap_pme_clock:    u8,
        pub pm_cap_rsvd_04:      u8,
        pub pm_cap_dsi:          u8,
        pub pm_cap_auxcurrent:   u8,
        pub pm_cap_d1support:    u8,
        pub pm_cap_d2support:    u8,
        pub pm_cap_pmesupport:   u8,
        pub pm_csr_nosoftrst:    u8,
        pub pm_csr_bpccen:       u8,
        pub pm_csr_b2b3s:        u8,
        pub pm_csr_power_state:  u8,
        pub pm_csr_dynamic_data: u8,
        pub pm_csr_reserved:     u8,
        pub pm_csr_pme_enabled:  u8,
        pub pm_csr_data_select:  u8,
        pub pm_csr_data_scale:   u8,
        pub pm_csr_pme_status:   u8,
    }

    impl PmCap
    {
        pub fn new(data: u64, pm_base_ptr : u8) -> Self {
            Self {
                //
                // pm
                //
                pm_cap_on: (data & 0xFFFFFFFF != 0) as u8,
                pm_cap_next_ptr: get_bits((data & 0xFFFFFFFF) as u32, 15, 8) as u8,
                pm_base_ptr,
                pm_cap_id: get_bits((data & 0xFFFFFFFF) as u32, 7, 0) as u8,
                pm_cap_version: get_bits((data & 0xFFFFFFFF) as u32, 18, 16) as u8,
                pm_cap_pme_clock: get_bit((data & 0xFFFFFFFF) as u32, 19) as u8,
                pm_cap_rsvd_04: get_bit((data & 0xFFFFFFFF) as u32, 20) as u8,
                pm_cap_dsi: get_bit((data & 0xFFFFFFFF) as u32, 21) as u8,
                pm_cap_auxcurrent: get_bits((data & 0xFFFFFFFF) as u32, 24, 22) as u8,
                pm_cap_d1support: get_bit((data & 0xFFFFFFFF) as u32, 25) as u8,
                pm_cap_d2support: get_bit((data & 0xFFFFFFFF) as u32, 26) as u8,
                pm_cap_pmesupport: get_bits((data & 0xFFFFFFFF) as u32, 31, 27) as u8,
                //
                // csr
                //
                pm_csr_nosoftrst: get_bits(((data >> 32) & 0xFFFFFFFF) as u32, 3, 2) as u8,
                pm_csr_bpccen: get_bit(((data >> 32) & 0xFFFFFFFF) as u32, 23) as u8,
                pm_csr_b2b3s: get_bit(((data >> 32) & 0xFFFFFFFF) as u32, 22) as u8,
                pm_csr_power_state: get_bits(((data >> 32) & 0xFFFFFFFF) as u32, 1, 0) as u8,
                pm_csr_dynamic_data: get_bit(((data >> 32) & 0xFFFFFFFF) as u32, 4) as u8,
                pm_csr_reserved: get_bits(((data >> 32) & 0xFFFFFFFF) as u32, 7, 5) as u8,
                pm_csr_pme_enabled: get_bit(((data >> 32) & 0xFFFFFFFF) as u32, 8) as u8,
                pm_csr_data_select: get_bits(((data >> 32) & 0xFFFFFFFF) as u32, 12, 9) as u8,
                pm_csr_data_scale: get_bits(((data >> 32) & 0xFFFFFFFF) as u32, 14, 13) as u8,
                pm_csr_pme_status: get_bit(((data >> 32) & 0xFFFFFFFF) as u32, 15) as u8,
            }
        }
    }
    #[derive(Debug)]
    pub struct MsiCap  {
        pub msi_cap_on : u8,
        pub msi_cap_nextptr : u8,
        pub msi_base_ptr : u8,
        pub msi_cap_multimsgcap : u8,
        pub msi_cap_multimsg_extension : u8,
        pub msi_cap_64_bit_addr_capable : u8,
        pub msi_cap_per_vector_masking_capable : u8,
    }
    impl MsiCap
    {
        pub fn new(data: u32, msi_base_ptr: u8) -> Self {
            Self {
                msi_cap_on: (data != 0) as u8,
                msi_cap_nextptr: get_bits(data, 15, 8) as u8,
                msi_base_ptr,
                msi_cap_multimsgcap: get_bits(data, 19, 17) as u8,
                msi_cap_multimsg_extension: get_bits(data, 22, 20) as u8,
                msi_cap_64_bit_addr_capable: get_bit(data, 23) as u8,
                msi_cap_per_vector_masking_capable: get_bit(data, 24) as u8,
            }
        }
    }

    #[derive(Debug)]
    pub struct PciCap {
        pub pcie_cap_on : u8,
        pub pcie_cap_capability_id : u8,
        pub pcie_cap_nextptr : u8,
        pub pcie_base_ptr : u8,
        pub pcie_cap_capability_version : u8,
        pub pcie_cap_device_port_type : u8,
        pub pcie_cap_slot_implemented : u8,
        pub pcie_cap_interrupt_message_number : u8,
        pub dev_cap_max_payload_supported: u8,
        pub dev_cap_phantom_functions_support: u8,
        pub dev_cap_ext_tag_supported: u8,
        pub dev_cap_endpoint_l0s_latency: u8,
        pub dev_cap_endpoint_l1_latency: u8,
        pub dev_cap_role_based_error: u8,
        pub dev_cap_enable_slot_pwr_limit_value: u8,
        pub dev_cap_enable_slot_pwr_limit_scale: u8,
        pub dev_cap_function_level_reset_capable: u8,
        pub dev_ctrl_corr_err_reporting: u8,
        pub dev_ctrl_non_fatal_reporting: u8,
        pub dev_ctrl_fatal_err_reporting: u8,
        pub dev_ctrl_ur_reporting: u8,
        pub dev_ctrl_relaxed_ordering: u8,
        pub dev_ctrl_max_payload_size: u8,
        pub dev_ctrl_ext_tag_default: u8,
        pub dev_ctrl_phantom_func_enable: u8,
        pub dev_ctrl_aux_power_enable: u8,
        pub dev_ctrl_enable_no_snoop: u8,
        pub dev_ctrl_max_read_request_size: u8,
        pub dev_ctrl_cfg_retry_status_enable: u8,
        pub cpl_timeout_ranges_supported: u8,
        pub cpl_timeout_disable_supported: u8,
        pub dev_ctrl2_completiontimeoutvalue: u8,
        pub dev_ctrl2_completiontimeoutdisable: u8,
        pub link_cap_max_link_speed: u8,
        pub link_cap_max_link_width: u8,
        pub link_cap_aspm_support: u8,
        pub link_cap_l0s_exit_latency: u8,
        pub link_cap_l1_exit_latency: u8,
        pub link_cap_clock_power_management: u8,
        pub link_cap_aspm_optionality: u8,
        pub link_cap_rsvd_23: u8,
        pub link_control_rcb: u8,
        pub link_status_slot_clock_config: u8,
        pub link_status_link_speed: u8,
        pub link_status_link_width: u8,
        pub link_cap2_linkspeedssupported: u8,
        pub link_ctrl2_target_link_speed: u8,
        pub link_ctrl2_entercompliance: u8,
        pub link_ctrl2_hw_autonomous_speed_disable: u8,
        pub link_ctrl2_deemphasis: u8,
        pub link_ctrl2_transmitmargin: u8,
        pub link_ctrl2_entermodifiedcompliance: u8,
        pub link_ctrl2_compliancesos: u8,
        pub link_status2_deemphasis: u8,
        pub link_status2_deemphasislvl: u8,
        pub link_status2_equalizationcomplete: u8,
        pub link_status2_equalizationphase1successful: u8,
        pub link_status2_equalizationphase2successful: u8,
        pub link_status2_equalizationphase3successful: u8,
        pub link_status2_linkequalizationrequest: u8,
    }
    impl PciCap
    {
        pub fn new(pci: u32, dev: u64, dev2: u64, link: u64, link2: u64, pcie_base_ptr : u8) -> Self {
            Self {
                pcie_cap_on: (pci != 0) as u8,
                pcie_cap_capability_id: get_bits(pci, 7, 0) as u8,
                pcie_cap_nextptr: get_bits(pci, 15, 8) as u8,
                pcie_base_ptr,
                pcie_cap_capability_version: get_bits(pci, 19, 16) as u8,
                pcie_cap_device_port_type: get_bits(pci, 23, 20) as u8,
                pcie_cap_slot_implemented: get_bit(pci, 24) as u8,
                pcie_cap_interrupt_message_number: get_bits(pci, 29, 25) as u8,
                dev_cap_max_payload_supported: get_bits((dev & 0xFFFFFFFF) as u32, 2, 0) as u8,
                dev_cap_phantom_functions_support: get_bits((dev & 0xFFFFFFFF) as u32, 4, 3) as u8,
                dev_cap_ext_tag_supported: get_bit((dev & 0xFFFFFFFF) as u32, 5) as u8,
                dev_cap_endpoint_l0s_latency: get_bits((dev & 0xFFFFFFFF) as u32, 8, 6) as u8,
                dev_cap_endpoint_l1_latency: get_bits((dev & 0xFFFFFFFF) as u32, 11, 9) as u8,
                dev_cap_role_based_error: get_bit((dev & 0xFFFFFFFF) as u32, 15) as u8,
                dev_cap_enable_slot_pwr_limit_value: get_bits((dev & 0xFFFFFFFF) as u32, 25, 18) as u8,
                dev_cap_enable_slot_pwr_limit_scale: get_bits((dev & 0xFFFFFFFF) as u32, 27, 26) as u8,
                dev_cap_function_level_reset_capable: get_bit((dev & 0xFFFFFFFF) as u32, 28) as u8,
                dev_ctrl_corr_err_reporting: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 0) as u8,
                dev_ctrl_non_fatal_reporting: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 1) as u8,
                dev_ctrl_fatal_err_reporting: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 2) as u8,
                dev_ctrl_ur_reporting: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 3) as u8,
                dev_ctrl_relaxed_ordering: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 4) as u8,
                dev_ctrl_max_payload_size: get_bits(((dev >> 32) & 0xFFFFFFFF) as u32, 7, 5) as u8,
                dev_ctrl_ext_tag_default: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 8) as u8,
                dev_ctrl_phantom_func_enable: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 9) as u8,
                dev_ctrl_aux_power_enable: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 10) as u8,
                dev_ctrl_enable_no_snoop: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 11) as u8,
                dev_ctrl_max_read_request_size: get_bits(((dev >> 32) & 0xFFFFFFFF) as u32, 14, 12) as u8,
                dev_ctrl_cfg_retry_status_enable: get_bit(((dev >> 32) & 0xFFFFFFFF) as u32, 15) as u8,
                cpl_timeout_ranges_supported: get_bits((dev2 & 0xFFFFFFFF) as u32, 3, 0) as u8,
                cpl_timeout_disable_supported: get_bit((dev2 & 0xFFFFFFFF) as u32, 4) as u8,
                dev_ctrl2_completiontimeoutvalue: get_bits(((dev2 >> 32) & 0xFFFFFFFF) as u32, 3, 0) as u8,
                dev_ctrl2_completiontimeoutdisable: get_bit(((dev2 >> 32) & 0xFFFFFFFF) as u32, 4) as u8,
                link_cap_max_link_speed: get_bits((link & 0xFFFFFFFF) as u32, 3, 0) as u8,
                link_cap_max_link_width: get_bits((link & 0xFFFFFFFF) as u32, 9, 4) as u8,
                link_cap_aspm_support: get_bits((link & 0xFFFFFFFF) as u32, 11, 10) as u8,
                link_cap_l0s_exit_latency: get_bits((link & 0xFFFFFFFF) as u32, 14, 12) as u8,
                link_cap_l1_exit_latency: get_bits((link & 0xFFFFFFFF) as u32, 17, 15) as u8,
                link_cap_clock_power_management: get_bits((link & 0xFFFFFFFF) as u32, 19, 18) as u8,
                link_cap_aspm_optionality: get_bit((link & 0xFFFFFFFF) as u32, 22) as u8,
                link_cap_rsvd_23: get_bits((link & 0xFFFFFFFF) as u32, 23, 19) as u8,
                link_control_rcb: get_bit(((link >> 32) & 0xFFFFFFFF) as u32, 3) as u8,
                link_status_slot_clock_config: get_bit(((link >> 32) & 0xFFFFFFFF) as u32, 28) as u8,
                link_status_link_speed: get_bits(((link >> 32) & 0xFFFFFFFF) as u32, 19, 16) as u8,
                link_status_link_width: get_bits(((link >> 32) & 0xFFFFFFFF) as u32, 25, 20) as u8,
                link_cap2_linkspeedssupported: get_bits((link2 & 0xFFFFFFFF) as u32, 3, 1) as u8,
                link_ctrl2_target_link_speed: get_bits(((link2 >> 32) & 0xFFFFFFFF) as u32, 3, 0) as u8,
                link_ctrl2_entercompliance: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 4) as u8,
                link_ctrl2_hw_autonomous_speed_disable: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 5) as u8,
                link_ctrl2_deemphasis: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 6) as u8,
                link_ctrl2_transmitmargin: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 7) as u8,
                link_ctrl2_entermodifiedcompliance: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 10) as u8,
                link_ctrl2_compliancesos: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 11) as u8,
                link_status2_deemphasis: get_bits(((link2 >> 32) & 0xFFFFFFFF) as u32, 15, 12) as u8,
                link_status2_deemphasislvl: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 16) as u8,
                link_status2_equalizationcomplete: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 17) as u8,
                link_status2_equalizationphase1successful: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 18) as u8,
                link_status2_equalizationphase2successful: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 19) as u8,
                link_status2_equalizationphase3successful: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 20) as u8,
                link_status2_linkequalizationrequest: get_bit(((link2 >> 32) & 0xFFFFFFFF) as u32, 21) as u8,
            }
        }
    }

    #[derive(Debug)]
    pub struct DsnCap {
        pub dsn_cap_on : u8,
        pub dsn_cap_next_ptr : u16,
        pub dsn_base_ptr : u16,
        pub dsn_cap_id : u8,
        pub dsn_serial : u64
    }

    impl DsnCap {
        pub fn new(cap : u32, sn: u64, dsn_base_ptr : u16) -> Self
        {
            Self {
                dsn_cap_on: (cap != 0) as u8,
                dsn_cap_next_ptr: get_bits(cap, 31, 20) as u16,
                dsn_base_ptr,
                dsn_cap_id:  get_bits(cap, 7, 0) as u8,
                dsn_serial: sn
            }
        }
    }

    #[derive(Debug)]
    pub struct EmptyExtCap {
        pub cap_on : u8,
        pub cap_next_ptr : u16,
        pub cap_id : u8,
        pub base_ptr : u16,
    }

    impl EmptyExtCap {
        pub fn new(cap : u32, base_ptr : u16) -> Self
        {
            Self {
                cap_on: (cap != 0) as u8,
                cap_next_ptr: get_bits(cap, 31, 20) as u16,
                cap_id:  get_bits(cap, 7, 0) as u8,
                base_ptr
            }
        }
    }
}

impl Pci {
    pub const MAX_CAPABILITIES: u8 = 0x16;
    pub const MAX_EXTENDED_CAPABILITIES: u16 = 0x2F;

    fn read_field<T: Clone>(buffer: &[u8], offset: isize) -> T
    {
        let ptr = unsafe { buffer.as_ptr().offset(offset) as *const T };
        unsafe { (*ptr).clone() }
    }

    pub fn read<T: Clone>(&self, offset: isize) -> T
    {
        return Pci::read_field(&self.cfg, offset)
    }

    pub fn new(cfg: &[u8], size: usize) -> Self
    {
        let mut cfg_buffer: [u8; 0x1000] = [0; 0x1000];
        cfg_buffer[..size].copy_from_slice(&cfg[..size]);

        let vendor_id = Pci::read_field::<u16>(cfg, 0x00);
        let device_id = Pci::read_field::<u16>(cfg, 0x02);
        
        let subsystem_vendor_id = Pci::read_field::<u16>(cfg, 0x2C);
        let subsystem_device_id = Pci::read_field::<u16>(cfg, 0x2E);

        let command = Pci::read_field::<u16>(cfg, 0x04);
        let status =  Pci::read_field::<u16>(cfg, 0x06);

        let header_type = Pci::read_field::<u8>(cfg, 0x0E);

        let interrupt_line = Pci::read_field::<u8>(cfg, 0x3C);
        let interrupt_pin = Pci::read_field::<u8>(cfg, 0x3D);
        

        let capabilities_ptr =
            if !get_bit(status as u32, 4) {
                0 as u8
            } else {
                Pci::read_field::<u8>(cfg, 0x34)
            };

        let bus_number : u8;
        let secondary_bus : u8;
        let subordinate_bus : u8;
        if get_bits(header_type as u32, 6, 0) == 1 {
            bus_number      = Pci::read_field(cfg, 0x18);
            secondary_bus   = Pci::read_field(cfg, 0x19);
            subordinate_bus = Pci::read_field(cfg, 0x1A);
        } else {
            bus_number = 0;
            secondary_bus = 0;
            subordinate_bus = 0;
        }
        
        let class_code =
            {
                let a0 = Pci::read_field::<u8>(cfg, 0x09 + 0) as u32;
                let a1 = Pci::read_field::<u8>(cfg, 0x09 + 1) as u32;
                let a2 = Pci::read_field::<u8>(cfg, 0x09 + 2) as u32;
                (a2 << 16) | (a1 << 8) | a0
            };

        Self{
            cfg: cfg_buffer,
            vendor_id,
            device_id,
            subsystem_vendor_id,
            subsystem_device_id,
            command,
            status,
            header_type,
            interrupt_line,
            interrupt_pin,
            capabilities_ptr,
            bus_number,
            secondary_bus,
            subordinate_bus,
            class_code
        }
    }

    pub fn get_capability_by_id(&self, id: u8) -> u8
    {
        let mut off = self.capabilities_ptr;
        if off == 0 {
            return 0;
        }
        
        loop {
            let cap = self.read::<u16>(off as isize);
            if get_bits(cap as u32, 7, 0) as u8 == id
            {
                return off;
            }

            let next = get_bits(cap as u32, 15, 8);
            if next == 0
            {
                break;
            }
            off = next as u8;
        }
        return 0;
    }

    pub fn get_ext_capability_by_id(&self, id: u16) -> u16
    {
        let mut off = 0x100 as u16;
        
        loop {
            let cap = self.read::<u32>(off as isize);
            if get_bits(cap, 7, 0) as u16 == id
            {
                return off;
            }

            let next = get_bits(cap, 31, 20);
            if next == 0
            {
                break;
            }
            off = next as u16;
        }
        return 0;
    }
    
    pub fn get_pm(&self) -> cap::PmCap {
        let cap = self.get_capability_by_id(0x01);
        if cap != 0 {
            return cap::PmCap::new(self.read(cap as isize), cap);
        }
        return cap::PmCap::new(0, 0);
    }

    pub fn get_msi(&self) -> cap::MsiCap {
        let cap = self.get_capability_by_id(0x05);
        if cap != 0 {
            return cap::MsiCap::new(self.read(cap as isize), cap);
        }
        return cap::MsiCap::new(0, 0);
    }

    pub fn get_pci(&self) -> cap::PciCap {
        let cap = self.get_capability_by_id(0x10) as isize;
        if cap != 0 {
            return cap::PciCap::new(
                self.read::<u32>(cap), // pci

                self.read::<u64>(cap + 0x04), // dev
                self.read::<u64>(cap + 0x04 + 0x20), // dev2

                self.read::<u64>(cap + 0x0C), // link
                self.read::<u64>(cap + 0x0C + 0x20), // link2
                cap as u8
            );
        }
        return cap::PciCap::new(0, 0, 0, 0, 0, 0);
    }

    pub fn get_dsn(&self) -> cap::DsnCap {
        let cap = self.get_ext_capability_by_id(0x03) as isize;
        if cap != 0 {
            return cap::DsnCap::new(
                self.read::<u32>(cap),
                self.read::<u64>(cap + 0x04),
                cap as u16
            );
        }
        return cap::DsnCap::new(0, 0, 0);
    }

    pub fn get_empty_extended_cap(&self, id: u16) -> cap::EmptyExtCap {
        let cap = self.get_ext_capability_by_id(id);
        if cap != 0 {
            return cap::EmptyExtCap::new(
                self.read::<u32>(cap as isize), cap
            );
        }
        return cap::EmptyExtCap::new(0, 0);
    }
}

}

