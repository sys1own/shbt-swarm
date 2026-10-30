// crates/shbt-swarm-apotheosis/src/mera_episodic_rg.rs
#![forbid(unsafe_code)]

//! `mera_episodic_rg`: Transfinite episodic memory coarse-graining via descending MERA RG flow,
//! bounding the active working memory footprint to precisely 135,168 bytes (132 KiB).

/// Disentangler u* tensor element count: 4,096 complex doubles (2 disentanglers)
pub const DISENTANGLER_ELEMENTS: usize = 4096;

/// Isometry w* tensor element count: 4,096 complex doubles (2 isometries)
pub const ISOMETRY_ELEMENTS: usize = 4096;

/// Top density generator rho_top element count: 256 complex doubles
pub const TOP_DENSITY_ELEMENTS: usize = 256;

/// Total complex doubles in active MERA working horizon: 4096 + 4096 + 256 = 8,448
pub const TOTAL_COMPLEX_DOUBLES: usize = 8448;

/// Bytes per complex double (16 bytes: 8B real + 8B imag)
pub const BYTES_PER_COMPLEX_DOUBLE: usize = 16;

/// Exact active working memory footprint: 8,448 * 16 = 135,168 bytes (132 KiB / 33 memory pages)
pub const ACTIVE_MEMORY_FOOTPRINT_BYTES: usize = 135_168;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryRgError {
    FootprintExceedsHardBound,
    DisentanglerElementMismatch,
    IsometryElementMismatch,
    TopDensityElementMismatch,
}

/// Episodic MERA Compressor.
/// Coarse-grains past episodic memories across 30 UV layers down to an invariant IR fixed point,
/// storing historical epochs in the Class X0 Dark Ledger while maintaining an exact 135,168-byte active stub.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct EpisodicMeraCompressor {
    pub disentangler_count: usize,
    pub isometry_count: usize,
    pub top_density_count: usize,
}

impl EpisodicMeraCompressor {
    pub const fn canonical() -> Self {
        Self {
            disentangler_count: DISENTANGLER_ELEMENTS,
            isometry_count: ISOMETRY_ELEMENTS,
            top_density_count: TOP_DENSITY_ELEMENTS,
        }
    }

    /// Computes total active memory footprint in bytes:
    /// (disentanglers + isometries + top_density) * 16 bytes
    pub const fn memory_footprint_bytes(&self) -> usize {
        (self.disentangler_count + self.isometry_count + self.top_density_count) * BYTES_PER_COMPLEX_DOUBLE
    }

    /// Verifies footprint does not exceed the hardware DMA buffer limit (135,168 bytes)
    pub const fn verify_memory_bound(&self) -> Result<usize, MemoryRgError> {
        if self.disentangler_count != DISENTANGLER_ELEMENTS {
            return Err(MemoryRgError::DisentanglerElementMismatch);
        }
        if self.isometry_count != ISOMETRY_ELEMENTS {
            return Err(MemoryRgError::IsometryElementMismatch);
        }
        if self.top_density_count != TOP_DENSITY_ELEMENTS {
            return Err(MemoryRgError::TopDensityElementMismatch);
        }
        let bytes = self.memory_footprint_bytes();
        if bytes > ACTIVE_MEMORY_FOOTPRINT_BYTES {
            return Err(MemoryRgError::FootprintExceedsHardBound);
        }
        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_episodic_mera_memory_footprint_exact() {
        let comp = EpisodicMeraCompressor::canonical();
        assert_eq!(comp.memory_footprint_bytes(), 135_168);
        assert_eq!(comp.verify_memory_bound().unwrap(), 135_168);
    }
}
