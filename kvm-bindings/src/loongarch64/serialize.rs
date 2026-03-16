// Copyright (C) 2020-2026 Loongson Technology Corporation Limited
// SPDX-License-Identifier: Apache-2.0

use super::bindings::{
    kvm_fpu, kvm_fpu_kvm_fpureg, kvm_iocsr_entry, kvm_irq_routing_entry,
    kvm_irq_routing_entry__bindgen_ty_1, kvm_irq_routing_msi__bindgen_ty_1,
    kvm_mp_state, kvm_one_reg, kvm_regs,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use zerocopy::{IntoBytes, transmute};

serde_impls! {
    kvm_regs,
    kvm_mp_state,
    kvm_one_reg,
    kvm_iocsr_entry,
    kvm_irq_routing_entry
}

#[derive(Serialize, Deserialize)]
struct KvmFpuSerde {
    fcsr: u32,
    fcc: u64,
    fpr: [[u64; 4]; 32],
}

impl Serialize for kvm_fpu {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let helper = KvmFpuSerde {
            fcsr: self.fcsr,
            fcc: self.fcc,
            fpr: self.fpr.map(|reg| reg.val64),
        };
        helper.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for kvm_fpu {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = KvmFpuSerde::deserialize(deserializer)?;
        Ok(kvm_fpu {
            fcsr: helper.fcsr,
            fcc: helper.fcc,
            fpr: helper.fpr.map(|val64| kvm_fpu_kvm_fpureg { val64 }),
        })
    }
}

// SAFETY: zerocopy's derives explicitly disallow deriving for unions where
// the fields have different sizes, due to the smaller fields having padding.
// Miri however does not complain about these implementations (e.g. about
// reading the "padding" for one union field as valid data for a bigger one)
unsafe impl IntoBytes for kvm_irq_routing_msi__bindgen_ty_1 {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }
}

// SAFETY: same reasoning.
unsafe impl IntoBytes for kvm_irq_routing_entry__bindgen_ty_1 {
    fn only_derive_is_allowed_to_implement_this_trait()
    where
        Self: Sized,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    fn is_serde<T: Serialize + for<'de> Deserialize<'de> + Default>() {
        let config = bincode::config::standard();
        let serialized = bincode::serde::encode_to_vec(T::default(), config).unwrap();
        let (deserialized, _): (T, _) =
            bincode::serde::decode_from_slice(&serialized, config).unwrap();
        let serialized_again = bincode::serde::encode_to_vec(deserialized, config).unwrap();
        assert_eq!(serialized, serialized_again);
    }

    #[test]
    fn static_assert_serde_impls() {
        is_serde::<kvm_regs>();
        is_serde::<kvm_fpu>();
        is_serde::<kvm_mp_state>();
        is_serde::<kvm_one_reg>();
        is_serde::<kvm_iocsr_entry>();
        is_serde::<kvm_irq_routing>();
        is_serde::<kvm_irq_routing_entry>();
    }
}