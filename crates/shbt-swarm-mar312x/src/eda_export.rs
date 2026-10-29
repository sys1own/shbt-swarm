//! `crates/shbt-swarm-mar312x/src/eda_export.rs`
//! Photonic & RF EDA Exporter for 8×8 InP/InGaAs HBT & PIC Boundary Character Rotator.
//!
//! Generates deterministic GDSII binary streams (`pic8x8.gds`), ISO 10303-21 STEP models
//! (`waveguide.step`), and Touchstone 2.0 S2P multi-port RF network profiles (`interposer.s2p`)
//! in freestanding, `#![no_std]`, zero-allocation environments.

#![forbid(unsafe_op_in_unsafe_fn)]

/// Target characteristic impedance of the 12-layer Rogers RO4350B interposer: 50.12 Ohm.
pub const INTERPOSER_TARGET_Z0: f64 = 50.12;

/// Characteristic impedance tolerance band: +/- 0.80 Ohm.
pub const INTERPOSER_TOLERANCE_OHM: f64 = 0.80;

/// Return loss threshold: |S11| <= -24.5 dB.
pub const MAX_S11_RETURN_LOSS_DB: f64 = -24.5;

/// Number of interposer RF layers: 12.
pub const INTERPOSER_LAYERS: usize = 12;

/// Emitter array dimensions: 8x8 (64 elements).
pub const PIC_ARRAY_DIM: usize = 8;
pub const NUM_EMITTERS: usize = 64;

/// Waveguide array pitch: 127.0 um (127,000 nm).
pub const WAVEGUIDE_PITCH_UM: f64 = 127.0;

/// Waveguide stripe width: 5.0 um (5,000 nm).
pub const WAVEGUIDE_WIDTH_UM: f64 = 5.0;

/// Waveguide stripe length: 350.0 um (350,000 nm).
pub const WAVEGUIDE_LENGTH_UM: f64 = 350.0;

/// GDSII database unit in nanometers (1 nm = 1e-9 m; grid resolution 1.0 pm).
pub const GDS_DATABASE_UNIT_NM: i32 = 1;

/// EDA export error codes.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EdaExportError {
    BufferTooSmall,
    ImpedanceOutOfTolerance,
    InvalidFrequencyList,
}

/// Helper struct for writing into a fixed mutable byte slice without dynamic allocation.
struct SliceWriter<'a> {
    buf: &'a mut [u8],
    offset: usize,
}

impl<'a> SliceWriter<'a> {
    #[inline]
    fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, offset: 0 }
    }

    #[inline]
    fn write_bytes(&mut self, data: &[u8]) -> Result<(), EdaExportError> {
        if self.offset + data.len() > self.buf.len() {
            return Err(EdaExportError::BufferTooSmall);
        }
        self.buf[self.offset..self.offset + data.len()].copy_from_slice(data);
        self.offset += data.len();
        Ok(())
    }

    #[inline]
    fn write_be_u16(&mut self, val: u16) -> Result<(), EdaExportError> {
        self.write_bytes(&val.to_be_bytes())
    }

    #[inline]
    fn write_be_u32(&mut self, val: u32) -> Result<(), EdaExportError> {
        self.write_bytes(&val.to_be_bytes())
    }

    #[inline]
    fn write_be_u64(&mut self, val: u64) -> Result<(), EdaExportError> {
        self.write_bytes(&val.to_be_bytes())
    }
}

// ------------------------------------------------------------------
// GDSII Stream Binary Serialization
// ------------------------------------------------------------------

fn gds_write_record_u16(
    w: &mut SliceWriter,
    record: u8,
    data_type: u8,
    vals: &[u16],
) -> Result<(), EdaExportError> {
    let len = 4 + 2 * vals.len() as u16;
    w.write_be_u16(len)?;
    w.write_bytes(&[record, data_type])?;
    for &v in vals {
        w.write_be_u16(v)?;
    }
    Ok(())
}

fn gds_write_record_u32(
    w: &mut SliceWriter,
    record: u8,
    data_type: u8,
    vals: &[u32],
) -> Result<(), EdaExportError> {
    let len = 4 + 4 * vals.len() as u16;
    w.write_be_u16(len)?;
    w.write_bytes(&[record, data_type])?;
    for &v in vals {
        w.write_be_u32(v)?;
    }
    Ok(())
}

fn gds_write_record_str(
    w: &mut SliceWriter,
    record: u8,
    s: &str,
) -> Result<(), EdaExportError> {
    let bytes = s.as_bytes();
    let pad = bytes.len() % 2;
    let len = (4 + bytes.len() + pad) as u16;
    w.write_be_u16(len)?;
    w.write_bytes(&[record, 0x06])?;
    w.write_bytes(bytes)?;
    if pad == 1 {
        w.write_bytes(&[0x00])?;
    }
    Ok(())
}

/// Generates a valid GDSII binary stream for the 8×8 InP/InGaAsP PIC array.
/// Returns the number of bytes written to `out`.
pub fn export_gdsii_buf(out: &mut [u8]) -> Result<usize, EdaExportError> {
    let mut w = SliceWriter::new(out);

    // HEADER (v600)
    gds_write_record_u16(&mut w, 0x00, 0x02, &[600])?;

    // BGNLIB (Year, Month, Day, Hour, Min, Sec twice)
    gds_write_record_u16(&mut w, 0x01, 0x01, &[2026, 10, 27, 0, 0, 0, 2026, 10, 27, 0, 0, 0])?;

    // LIBNAME
    gds_write_record_str(&mut w, 0x02, "SHBT_SWARM_PIC")?;

    // UNITS: user=1e-3, db=1e-9 (meter). IEEE-754 converted to GDS format constants.
    w.write_be_u16(20)?;
    w.write_bytes(&[0x03, 0x05])?;
    w.write_be_u64(0x3E4189374BC6A7EF)?;
    w.write_be_u64(0x3944B82FA09B5A54)?;

    // BGNSTR
    gds_write_record_u16(&mut w, 0x05, 0x02, &[0])?;

    // STRNAME
    gds_write_record_str(&mut w, 0x06, "PIC8X8")?;

    // 8×8 array of 5 um x 350 um waveguide stripes at 127 um pitch (in nm: 127_000)
    let pitch_nm = (WAVEGUIDE_PITCH_UM * 1000.0) as i32;
    let width_nm = (WAVEGUIDE_WIDTH_UM * 1000.0) as i32;
    let length_nm = (WAVEGUIDE_LENGTH_UM * 1000.0) as i32;

    for row in 0..PIC_ARRAY_DIM {
        for col in 0..PIC_ARRAY_DIM {
            // BOUNDARY
            gds_write_record_u16(&mut w, 0x08, 0x00, &[])?;
            // LAYER 1
            gds_write_record_u16(&mut w, 0x0D, 0x02, &[1])?;
            // DATATYPE 0
            gds_write_record_u16(&mut w, 0x0E, 0x02, &[0])?;

            let x0 = (col as i32) * pitch_nm;
            let y0 = (row as i32) * pitch_nm;
            let x1 = x0 + width_nm;
            let y1 = y0 + length_nm;

            // 5 closed boundary vertices: (x0,y0) -> (x1,y0) -> (x1,y1) -> (x0,y1) -> (x0,y0)
            let xy: [u32; 10] = [
                x0 as u32, y0 as u32,
                x1 as u32, y0 as u32,
                x1 as u32, y1 as u32,
                x0 as u32, y1 as u32,
                x0 as u32, y0 as u32,
            ];
            gds_write_record_u32(&mut w, 0x10, 0x03, &xy)?;

            // ENDEL
            gds_write_record_u16(&mut w, 0x11, 0x00, &[])?;
        }
    }

    // ENDSTR
    gds_write_record_u16(&mut w, 0x07, 0x00, &[])?;
    // ENDLIB
    gds_write_record_u16(&mut w, 0x04, 0x00, &[])?;

    Ok(w.offset)
}

// ------------------------------------------------------------------
// ISO 10303-21 STEP Waveguide Model Generation
// ------------------------------------------------------------------

/// Generates an ISO 10303-21 STEP CAD stub for the sapphire/InP waveguide.
/// Parameterized by length, width, and height in millimeters.
pub fn export_step_buf(
    out: &mut [u8],
    length_mm: f64,
    width_mm: f64,
    height_mm: f64,
) -> Result<usize, EdaExportError> {
    let mut w = SliceWriter::new(out);

    w.write_bytes(b"ISO-10303-21;\n")?;
    w.write_bytes(b"HEADER;\n")?;
    w.write_bytes(b"FILE_DESCRIPTION(('SHBT-SWARM Sapphire Waveguide Array'),'2;1');\n")?;
    w.write_bytes(b"FILE_NAME('waveguide.step','2026-10-27T00:00:00',('Antigravity'),('SHBT-SWARM'),'','','');\n")?;
    w.write_bytes(b"FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\n")?;
    w.write_bytes(b"ENDSEC;\n")?;
    w.write_bytes(b"DATA;\n")?;
    w.write_bytes(b"#1=CARTESIAN_POINT('ORIGIN',(0.,0.,0.));\n")?;
    w.write_bytes(b"#2=DIRECTION('Z',(0.,0.,1.));\n")?;
    w.write_bytes(b"#3=DIRECTION('X',(1.,0.,0.));\n")?;
    w.write_bytes(b"#4=AXIS2_PLACEMENT_3D('',#1,#2,#3);\n")?;

    // Format BLOCK parameters as fixed-point decimal in ASCII.
    w.write_bytes(b"#5=BLOCK('WAVEGUIDE',#4,")?;
    write_f64_ascii(&mut w, length_mm)?;
    w.write_bytes(b",")?;
    write_f64_ascii(&mut w, width_mm)?;
    w.write_bytes(b",")?;
    write_f64_ascii(&mut w, height_mm)?;
    w.write_bytes(b");\n")?;

    w.write_bytes(b"ENDSEC;\n")?;
    w.write_bytes(b"END-ISO-10303-21;\n")?;

    Ok(w.offset)
}

// ------------------------------------------------------------------
// Touchstone 2.0 S2P Multi-Port RF Network Export
// ------------------------------------------------------------------

/// Computes S11 return loss in dB for load characteristic impedance `z0`
/// matched against nominal 50.0 Ohm reference.
#[inline]
pub fn s11_return_loss_db(z0: f64) -> f64 {
    let gamma = ((z0 - 50.0) / (z0 + 50.0)).abs();
    if gamma <= 1.0e-12 {
        -240.0
    } else {
        // Log10 approximation using ln
        20.0 * (natural_log(gamma) / core::f64::consts::LN_10)
    }
}

/// Natural log approximation for #![no_std] using series expansion around 1.0.
fn natural_log(x: f64) -> f64 {
    if x <= 0.0 {
        return -100.0;
    }
    let mut val = x;
    let mut k = 0;
    while val > 2.0 {
        val /= core::f64::consts::E;
        k += 1;
    }
    while val < 0.5 {
        val *= core::f64::consts::E;
        k -= 1;
    }
    let y = (val - 1.0) / (val + 1.0);
    let y2 = y * y;
    let mut sum = y;
    let mut term = y;
    for i in 1..8 {
        term *= y2;
        sum += term / (2 * i + 1) as f64;
    }
    2.0 * sum + (k as f64)
}

/// Generates Touchstone 2.0 S2P 2-port network parameters for the 12-layer interposer.
pub fn export_s2p_buf(
    out: &mut [u8],
    z0: f64,
    freqs_ghz: &[f64],
) -> Result<usize, EdaExportError> {
    if (z0 - INTERPOSER_TARGET_Z0).abs() > INTERPOSER_TOLERANCE_OHM {
        return Err(EdaExportError::ImpedanceOutOfTolerance);
    }
    if freqs_ghz.is_empty() {
        return Err(EdaExportError::InvalidFrequencyList);
    }

    let gamma = (z0 - 50.0) / (z0 + 50.0);
    let mut w = SliceWriter::new(out);

    w.write_bytes(b"! SHBT-SWARM 12-layer Rogers RO4350B interposer\n")?;
    w.write_bytes(b"! Target Z0 = 50.12 ohm, layers = 12\n")?;
    w.write_bytes(b"# GHZ S RI R 50\n")?;

    for &fg in freqs_ghz {
        // Line format: Freq S11_Re S11_Im S21_Re S21_Im S12_Re S12_Im S22_Re S22_Im
        write_f64_ascii(&mut w, fg)?;
        w.write_bytes(b" ")?;
        write_f64_ascii(&mut w, gamma)?;
        w.write_bytes(b" 0.000000 0.999500 0.000000 0.999500 0.000000 ")?;
        write_f64_ascii(&mut w, gamma)?;
        w.write_bytes(b" 0.000000\n")?;
    }

    Ok(w.offset)
}

/// Helper function to write a floating point value in ASCII decimal representation into buffer.
fn write_f64_ascii(w: &mut SliceWriter, val: f64) -> Result<(), EdaExportError> {
    if val < 0.0 {
        w.write_bytes(b"-")?;
        return write_f64_ascii(w, -val);
    }
    let int_part = val as u64;
    let mut rem = val - (int_part as f64);
    if rem < 0.0 {
        rem = 0.0;
    }

    // Write integer part
    let mut digits = [0u8; 20];
    let mut d_idx = 0;
    let mut n = int_part;
    if n == 0 {
        digits[0] = b'0';
        d_idx = 1;
    } else {
        while n > 0 {
            digits[d_idx] = b'0' + (n % 10) as u8;
            n /= 10;
            d_idx += 1;
        }
    }
    for i in (0..d_idx).rev() {
        w.write_bytes(&[digits[i]])?;
    }

    w.write_bytes(b".")?;

    // Write 6 decimal places
    let mut frac_int = (rem * 1_000_000.0 + 0.5) as u64;
    if frac_int >= 1_000_000 {
        frac_int = 999_999;
    }
    let mut frac_digits = [0u8; 6];
    let mut f = frac_int;
    for i in (0..6).rev() {
        frac_digits[i] = b'0' + (f % 10) as u8;
        f /= 10;
    }
    w.write_bytes(&frac_digits)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_gdsii_generation() {
        let mut buf = [0u8; 8192];
        let bytes_written = export_gdsii_buf(&mut buf).unwrap();
        assert!(bytes_written > 1500);
        assert!(bytes_written < 8192);

        // Verify GDSII Header record: length = 6, record = 0x00, type = 0x02
        assert_eq!(buf[0..2], [0x00, 0x06]);
        assert_eq!(buf[2..4], [0x00, 0x02]);
        assert_eq!(buf[4..6], [0x02, 0x58]); // 600

        // Verify ending records: ENDLIB length = 4, record = 0x04, type = 0x00
        assert_eq!(&buf[bytes_written - 4..bytes_written], &[0x00, 0x04, 0x04, 0x00]);
    }

    #[test]
    fn test_export_step_generation() {
        let mut buf = [0u8; 2048];
        let bytes_written = export_step_buf(&mut buf, 3.20, 0.05, 0.35).unwrap();
        assert!(bytes_written > 100);

        let s = core::str::from_utf8(&buf[..bytes_written]).unwrap();
        assert!(s.starts_with("ISO-10303-21;"));
        assert!(s.contains("FILE_DESCRIPTION(('SHBT-SWARM Sapphire Waveguide Array')"));
        assert!(s.contains("#5=BLOCK('WAVEGUIDE'"));
        assert!(s.ends_with("END-ISO-10303-21;\n"));
    }

    #[test]
    fn test_export_s2p_and_s11_return_loss() {
        let rl = s11_return_loss_db(INTERPOSER_TARGET_Z0);
        assert!(rl <= MAX_S11_RETURN_LOSS_DB); // -58.4 dB <= -24.5 dB

        let mut buf = [0u8; 2048];
        let freqs = [10.0, 20.0, 30.0, 40.0];
        let bytes_written = export_s2p_buf(&mut buf, INTERPOSER_TARGET_Z0, &freqs).unwrap();
        assert!(bytes_written > 50);

        let s = core::str::from_utf8(&buf[..bytes_written]).unwrap();
        assert!(s.contains("! SHBT-SWARM 12-layer Rogers RO4350B interposer"));
        assert!(s.contains("# GHZ S RI R 50"));
        assert!(s.contains("40.000000"));

        // Out of tolerance check
        assert_eq!(
            export_s2p_buf(&mut buf, 52.0, &freqs),
            Err(EdaExportError::ImpedanceOutOfTolerance)
        );
    }
}
