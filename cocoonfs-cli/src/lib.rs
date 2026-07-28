// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Red Hat, LLC
// Author: Arun Menon <armenon@redhat.com>

//! Cocoonfs CLI utility library exports.
//! Exposes host-based block device and sync drivers.

pub mod std_sync_types;
pub mod std_file_nvblkdev;

pub(crate) use cocoon_tpm_storage as storage;
pub(crate) use cocoon_tpm_utils_async as utils_async;
pub(crate) use cocoon_tpm_utils_common as utils_common;
