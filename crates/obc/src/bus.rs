//! The `Bus` abstraction the CPU core will use for every load and store.
//!
//! A `Bus` implementation owns the meaning of an out-of-range or misaligned access; callers never
//! check bounds or alignment themselves.

/// A memory-mapped bus, read and written in units of 1, 2 or 4 bytes.
///
/// `size` is always 1, 2 or 4 (bytes). `read` zero-extends the accessed bytes, little-endian, into
/// the returned `u32`. `write` stores the low `size` bytes of `value`, little-endian. Accesses need
/// not be aligned to `size`; an implementation decides what an out-of-range access means.
pub trait Bus {
    fn read(&self, addr: u32, size: u8) -> u32;
    fn write(&mut self, addr: u32, size: u8, value: u32);
}

/// A fixed-size, byte-addressed RAM region.
///
/// Reads and writes are little-endian and need not be aligned. An access that falls outside
/// `[0, len)`, in whole or in part, reads `0` for the out-of-range bytes and drops the
/// out-of-range bytes of a write, byte by byte.
#[derive(Debug)]
pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    /// Creates a zero-filled RAM region of `size` bytes.
    pub fn new(size: usize) -> Self {
        Self {
            data: vec![0; size],
        }
    }
}

impl Bus for Ram {
    fn read(&self, addr: u32, size: u8) -> u32 {
        let mut value: u32 = 0;
        for offset in 0..u32::from(size) {
            let byte = addr
                .checked_add(offset)
                .and_then(|a| usize::try_from(a).ok())
                .and_then(|i| self.data.get(i))
                .copied()
                .unwrap_or(0);
            value |= u32::from(byte).wrapping_shl(offset.wrapping_mul(8));
        }
        value
    }

    fn write(&mut self, addr: u32, size: u8, value: u32) {
        for offset in 0..u32::from(size) {
            let byte = value.wrapping_shr(offset.wrapping_mul(8)).to_le_bytes()[0];
            if let Some(index) = addr
                .checked_add(offset)
                .and_then(|a| usize::try_from(a).ok())
                && let Some(slot) = self.data.get_mut(index)
            {
                *slot = byte;
            }
        }
    }
}

/// One region mapped into an [`Mmio`] bus.
struct Region {
    base: u32,
    len: u32,
    bus: Box<dyn Bus>,
}

impl Region {
    fn contains(&self, addr: u32) -> bool {
        addr >= self.base && addr < self.base.saturating_add(self.len)
    }
}

/// Multiplexes several [`Bus`] regions, each mapped at its own base address, behind one `Bus`.
///
/// An address inside no mapped region behaves like an out-of-range [`Ram`] access: reads return
/// `0` and writes are ignored. A region's own bus sees addresses relative to its `base`.
#[derive(Default)]
pub struct Mmio {
    regions: Vec<Region>,
}

impl Mmio {
    /// Creates an `Mmio` bus with no regions mapped.
    pub fn new() -> Self {
        Self::default()
    }

    /// Maps `bus` into the address range `[base, base + len)`.
    pub fn map(&mut self, base: u32, len: u32, bus: Box<dyn Bus>) {
        self.regions.push(Region { base, len, bus });
    }
}

impl Bus for Mmio {
    fn read(&self, addr: u32, size: u8) -> u32 {
        self.regions
            .iter()
            .find(|region| region.contains(addr))
            .map_or(0, |region| {
                region.bus.read(addr.wrapping_sub(region.base), size)
            })
    }

    fn write(&mut self, addr: u32, size: u8, value: u32) {
        if let Some(region) = self.regions.iter_mut().find(|region| region.contains(addr)) {
            let offset = addr.wrapping_sub(region.base);
            region.bus.write(offset, size, value);
        }
    }
}

#[cfg(test)]
mod tests {
    /// Ram's unit tests
    mod ram {
        use crate::bus::{Bus, Ram};

        /// A 4-byte write and read round-trips, little-endian
        #[test]
        fn write_then_read_round_trips_little_endian() {
            let mut ram = Ram::new(16);
            ram.write(0, 4, 0x1122_3344);
            assert_eq!(ram.read(0, 4), 0x1122_3344);
            assert_eq!(ram.read(0, 1), 0x44);
            assert_eq!(ram.read(1, 1), 0x33);
            assert_eq!(ram.read(2, 1), 0x22);
            assert_eq!(ram.read(3, 1), 0x11);
        }

        /// A 2-byte write and read round-trips at an unaligned address
        #[test]
        fn unaligned_access_round_trips() {
            let mut ram = Ram::new(16);
            ram.write(1, 2, 0xABCD);
            assert_eq!(ram.read(1, 2), 0xABCD);
        }

        /// Reading entirely out of range returns 0
        #[test]
        fn out_of_range_read_returns_zero() {
            let ram = Ram::new(4);
            assert_eq!(ram.read(100, 4), 0);
        }

        /// Writing entirely out of range is a silent no-op
        #[test]
        fn out_of_range_write_is_a_no_op() {
            let mut ram = Ram::new(4);
            ram.write(100, 4, 0xDEAD_BEEF);
            assert_eq!(ram.read(0, 4), 0);
        }

        /// A read straddling the end of RAM returns the in-range bytes and zero for the rest
        #[test]
        fn partially_out_of_range_read_zero_fills_the_tail() {
            let mut ram = Ram::new(4);
            ram.write(0, 4, 0x1122_3344);
            // Bytes at addr 3 and 4: addr 3 is in range (0x11), addr 4 is out of range (0).
            assert_eq!(ram.read(3, 2), 0x0011);
        }

        /// A write straddling the end of RAM stores the in-range bytes and drops the rest
        #[test]
        fn partially_out_of_range_write_drops_the_tail() {
            let mut ram = Ram::new(4);
            ram.write(3, 2, 0xBBAA);
            assert_eq!(ram.read(3, 1), 0xAA);
        }
    }

    /// Mmio's unit tests
    mod mmio {
        use crate::bus::{Bus, Mmio, Ram};

        /// Reads and writes at each region's base address are routed to that region
        #[test]
        fn routes_accesses_to_the_mapped_region() {
            let mut mmio = Mmio::new();
            mmio.map(0, 4, Box::new(Ram::new(4)));
            mmio.map(0x1000, 4, Box::new(Ram::new(4)));

            mmio.write(0, 4, 0x1111_1111);
            mmio.write(0x1000, 4, 0x2222_2222);

            assert_eq!(mmio.read(0, 4), 0x1111_1111);
            assert_eq!(mmio.read(0x1000, 4), 0x2222_2222);
        }

        /// An address inside a region is translated relative to that region's base
        #[test]
        fn translates_the_address_relative_to_the_regions_base() {
            let mut mmio = Mmio::new();
            mmio.map(0x1000, 4, Box::new(Ram::new(4)));

            mmio.write(0x1002, 2, 0xABCD);

            assert_eq!(mmio.read(0x1002, 2), 0xABCD);
        }

        /// An address inside no mapped region reads 0
        #[test]
        fn unmapped_read_returns_zero() {
            let mut mmio = Mmio::new();
            mmio.map(0, 4, Box::new(Ram::new(4)));

            assert_eq!(mmio.read(0x1000, 4), 0);
        }

        /// A write to an address inside no mapped region is a silent no-op
        #[test]
        fn unmapped_write_is_a_no_op() {
            let mut mmio = Mmio::new();
            mmio.map(0, 4, Box::new(Ram::new(4)));

            mmio.write(0x1000, 4, 0xDEAD_BEEF);

            assert_eq!(mmio.read(0, 4), 0);
        }
    }
}
