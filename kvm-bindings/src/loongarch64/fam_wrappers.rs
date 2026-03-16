// Copyright (C) 2020-2026 Loongson Technology Corporation Limited
// SPDX-License-Identifier: Apache-2.0

use vmm_sys_util::fam::{FamStruct, FamStructWrapper};
use vmm_sys_util::generate_fam_struct_impl;

use super::bindings::*;

// Implement the FamStruct trait for kvm_irq_routing
generate_fam_struct_impl!(
    kvm_irq_routing,
    kvm_irq_routing_entry,
    entries,
    u32,
    nr,
    1024
);

// Implement the PartialEq trait for kvm_irq_routing.
impl PartialEq for kvm_irq_routing {
    fn eq(&self, other: &kvm_irq_routing) -> bool {
        // No need to call entries's eq, FamStructWrapper's PartialEq will do it for you
        self.nr == other.nr && self.flags == other.flags
    }
}

pub type KvmIrqRouting = FamStructWrapper<kvm_irq_routing>;

#[cfg(test)]
mod tests {
    use super::KvmIrqRouting;
    use vmm_sys_util::fam::FamStructWrapper;

    #[test]
    fn test_kvm_irq_routing_equality() {
        let mut wrapper = KvmIrqRouting::new(1).unwrap();
        assert_eq!(wrapper.as_slice().len(), 1);
        assert_eq!(wrapper.as_fam_struct_ref().nr, 1);
        assert_eq!(wrapper.as_fam_struct_ref().len(), 1);
    }
}