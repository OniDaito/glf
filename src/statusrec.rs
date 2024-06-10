//!    ___  __    ____ 
//!   / __)(  )  (  __)
//!  ( (_ \/ (_/\ ) _) 
//!   \___/\____/(__) 
//!   
//! # StatusRecord
//! An struct that holds the status record - details of how the sonar was peforming
//! at the time an image was taken.

use byteorder::{ByteOrder, LittleEndian};
use crate::CIHeader;


/// The Status Record. Holds information on the status of the sonar at this
/// particular time.
#[derive(Copy, Clone)]
pub struct StatusRecord {
    /// The CIHeader.
    pub header: CIHeader,
    /// BF Version.
    pub bf_version: u16,
    /// DA Version.
    pub da_version: u16,
    /// Flags.
    pub flags: u16,
    /// The Sonar ID.
    pub device_id: u16,
    /// XD Selected.
    pub xd_selected: u8,
    /// MK2: FPGA PCB temperature.
    pub vga_t1: f64,
    /// MK2: HSC PCB temperature.
    pub vga_t2: f64,
    /// MK2: DA FPGA.
    pub vga_t3: f64,
    /// VGA Transducer temperature.
    pub vga_t4: f64,
    /// PSU Temperature.
    pub psu_t: f64,
    /// Die temperature.
    pub die_t: f64,
    /// Transmit temperature.
    pub tx_t: f64,
    /// AFE0 Top temperature.
    pub afe0_top_temp: f64,
    /// AFE0 Bottom temperature.
    pub afe0_bot_temp: f64,
    /// AFE1 Top temperature.
    pub afe1_top_temp: f64,
    /// AFE1 Bottom temperature.
    pub afe1_bot_temp: f64,
    /// AFE2 Top temperature.
    pub afe2_top_temp: f64,
    /// AFE2 Bottom temperature.
    pub afe2_bot_temp: f64,
    /// AFE3 Top temperature.
    pub afe3_top_temp: f64,
    /// AFE3 Bottom temperature.
    pub afe3_bot_temp: f64,
    /// Link type (see 0716-SDS-00001-005 (Genesis Log File Format).pdf).
    pub link_type: u16,
    /// Uplink speed.
    pub uplink_speed: f64,
    /// Downlink spee.
    pub downlink_speed: f64,
    /// Link quality as percentage.
    pub link_quality: u16,
    /// The packet count (tx and rx).
    pub packet_count: u32,
    /// Received error count.
    pub recv_error: u32,
    /// Number of packets resent.
    pub resent_packet_count: u32,
    /// Number of dropped packets.
    pub dropped_packet_count: u32,
    /// Number of unknown packets - NOT USED.
    pub unknown_packet_count: u32,
    /// Lost line count.
    pub lost_line_count: u32,
    /// Packet count for all devices.
    pub general_count: u32,
    /// Alternative IP Address.
    pub sonar_alt_ip: u32,
    /// Currently connected surface PC IP Address.
    pub surface_ip: u32,
    /// The subnet mask.
    pub subnet_mask: [u8; 4],
    /// Current MAC Address.
    pub mac_addr: [u8; 6],
    /// INTERNAL USAGE.
    pub boot_sts_register: u32,
    /// INTERNAL USAGE.
    pub boot_sts_register_da: u32,
    /// Internal FPGA timestamp.
    pub fpga_time: u64,
    /// INTERNAL USAGE.
    pub dip_switch: u16,
    /// Shutdown reason (0 temperature, 1 out of water, 2 out of water indicator).
    pub shutdown_status: u16,
    /// Adaptor found?
    pub net_adap_found: bool,
    // Not parsing subsea internal temp or subsea cpu temp for now. 
}


/// Extract the status record
///
/// * `header` - the CI Header for this record.
/// * `dat_buffer` - the bytes buffer we are reading from.
/// * `file_offset` - the offset in the dat_buffer.
pub fn parse_status_record(header: &CIHeader, dat_buffer: &Vec<u8>, file_offset: &mut i64) -> StatusRecord {
    //! Parse the dat file to obtain a status record
    let mut fp: usize = *file_offset as usize;
    let bf_version = LittleEndian::read_u16(&dat_buffer[fp..(fp + 2)]);
    let da_version = LittleEndian::read_u16(&dat_buffer[(fp + 2)..(fp + 4)]);
    let flags = LittleEndian::read_u16(&dat_buffer[(fp + 4)..(fp + 6)]);
    let device_id = LittleEndian::read_u16(&dat_buffer[(fp + 6)..(fp + 8)]);
    let xd_selected = dat_buffer[8];
    fp = fp + 10;

    let vga_t1 = LittleEndian::read_f64(&dat_buffer[fp..(fp + 8)]);
    let vga_t2 = LittleEndian::read_f64(&dat_buffer[(fp + 8)..(fp + 16)]);
    let vga_t3 = LittleEndian::read_f64(&dat_buffer[(fp + 16)..(fp + 24)]);
    let vga_t4 = LittleEndian::read_f64(&dat_buffer[(fp + 24)..(fp + 32)]);
    fp = fp + 32;

    let psu_t = LittleEndian::read_f64(&dat_buffer[fp..(fp + 8)]);
    let die_t = LittleEndian::read_f64(&dat_buffer[(fp + 8)..(fp + 16)]);
    let tx_t = LittleEndian::read_f64(&dat_buffer[(fp + 16)..(fp + 24)]);
    fp = fp + 24;

    let afe0_top_temp = LittleEndian::read_f64(&dat_buffer[fp..(fp + 8)]);
    let afe0_bot_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 8)..(fp + 16)]);
    let afe1_top_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 16)..(fp + 24)]);
    let afe1_bot_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 24)..(fp + 32)]);
    let afe2_top_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 32)..(fp + 40)]);
    let afe2_bot_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 40)..(fp + 48)]);
    let afe3_top_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 48)..(fp + 56)]);
    let afe3_bot_temp: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 56)..(fp + 64)]);
    fp = fp + 64;

    let link_type = LittleEndian::read_u16(&dat_buffer[fp..(fp + 2)]);
    let uplink_speed: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 2)..(fp + 10)]);
    let downlink_speed: f64 = LittleEndian::read_f64(&dat_buffer[(fp + 10)..(fp + 18)]);
    let link_quality: u16 = LittleEndian::read_u16(&dat_buffer[(fp + 18)..(fp + 20)]);
    let packet_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 20)..(fp + 24)]);
    let recv_error_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 24)..(fp + 28)]);
    let resent_packet_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 28)..(fp + 32)]);
    let dropped_packet_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 32)..(fp + 36)]);
    let unknown_packet_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 36)..(fp + 40)]);
    fp = fp + 40;

    let lost_line_count = LittleEndian::read_u32(&dat_buffer[fp..(fp + 4)]);
    let general_count: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 4)..(fp + 8)]);
    let sonar_alt_ip: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 8)..(fp + 12)]);
    let surface_ip: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 12)..(fp + 16)]);
    let subnet_mask: [u8; 4] = dat_buffer[(fp + 16)..(fp + 20)].try_into().unwrap(); // Unwraps not ideal!
    let mac_addr: [u8; 6] = dat_buffer[(fp + 20)..(fp + 26)].try_into().unwrap();
    fp = fp + 26;

    let boot_sts_register = LittleEndian::read_u32(&dat_buffer[fp..(fp + 4)]);
    let boot_sts_register_da: u32 = LittleEndian::read_u32(&dat_buffer[(fp + 4)..(fp + 8)]);
    let fpga_time: u64 = LittleEndian::read_u64(&dat_buffer[(fp + 8)..(fp + 16)]);
    let dip_switch: u16 = LittleEndian::read_u16(&dat_buffer[(fp + 16)..(fp + 18)]);
    let shutdown_status: u16 = LittleEndian::read_u16(&dat_buffer[(fp + 18)..(fp + 20)]);
    let net_adap_found: bool = dat_buffer[20] != 0;
    fp = fp + 22; // Additional byte for some reason :/

    let record_size = fp - *file_offset as usize;

    let stat_rec = StatusRecord {
        header: *header,
        bf_version: bf_version,
        da_version: da_version,
        flags: flags,
        device_id: device_id,
        xd_selected: xd_selected,
        vga_t1: vga_t1,
        vga_t2: vga_t2,
        vga_t3: vga_t3,
        vga_t4: vga_t4,
        psu_t: psu_t,
        die_t: die_t,
        tx_t: tx_t,
        afe0_top_temp: afe0_top_temp,
        afe0_bot_temp: afe0_bot_temp,
        afe1_top_temp: afe1_top_temp,
        afe1_bot_temp: afe1_bot_temp,
        afe2_top_temp: afe2_top_temp,
        afe2_bot_temp: afe2_bot_temp,
        afe3_top_temp: afe3_top_temp,
        afe3_bot_temp: afe3_bot_temp,
        link_type: link_type,
        uplink_speed: uplink_speed,
        downlink_speed: downlink_speed,
        link_quality: link_quality,
        packet_count: packet_count,
        recv_error: recv_error_count,
        resent_packet_count: resent_packet_count,
        dropped_packet_count: dropped_packet_count,
        unknown_packet_count: unknown_packet_count,
        lost_line_count: lost_line_count,
        general_count: general_count,
        sonar_alt_ip: sonar_alt_ip,
        surface_ip: surface_ip,
        subnet_mask: subnet_mask,
        mac_addr: mac_addr,
        boot_sts_register: boot_sts_register,
        boot_sts_register_da: boot_sts_register_da,
        fpga_time: fpga_time,
        dip_switch: dip_switch,
        shutdown_status: shutdown_status,
        net_adap_found: net_adap_found,
    };

    *file_offset = *file_offset + (record_size as i64);
    stat_rec
}