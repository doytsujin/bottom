#[derive(Debug, Clone)]
pub struct MemHarvest {
    pub mem_total_in_mb: u64,
    pub mem_used_in_mb: u64,
}

impl Default for MemHarvest {
    fn default() -> Self {
        MemHarvest {
            mem_total_in_mb: 0,
            mem_used_in_mb: 0,
        }
    }
}

/// Meant for ARM use.
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub async fn get_sysinfo_mem_data_list(
    sys: &sysinfo::System, actually_get: bool,
) -> crate::utils::error::Result<Option<MemHarvest>> {
    use sysinfo::SystemExt;
    if !actually_get {
        return Ok(None);
    }

    Ok(Some(MemHarvest {
        mem_total_in_mb: sys.get_total_memory() / 1024,
        mem_used_in_mb: sys.get_used_memory() / 1024,
    }))
}

/// Meant for ARM use.
#[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
pub async fn get_sysinfo_swap_data_list(
    sys: &sysinfo::System, actually_get: bool,
) -> crate::utils::error::Result<Option<MemHarvest>> {
    use sysinfo::SystemExt;
    if !actually_get {
        return Ok(None);
    }

    Ok(Some(MemHarvest {
        mem_total_in_mb: sys.get_total_swap() / 1024,
        mem_used_in_mb: sys.get_used_swap() / 1024,
    }))
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
pub async fn get_heim_mem_data_list(
    actually_get: bool,
) -> crate::utils::error::Result<Option<MemHarvest>> {
    if !actually_get {
        return Ok(None);
    }

    let memory = heim::memory::memory().await?;

    Ok(Some(MemHarvest {
        mem_total_in_mb: memory.total().get::<heim::units::information::megabyte>(),
        mem_used_in_mb: memory.total().get::<heim::units::information::megabyte>()
            - memory
                .available()
                .get::<heim::units::information::megabyte>(),
    }))
}

#[cfg(not(any(target_arch = "aarch64", target_arch = "arm")))]
pub async fn get_heim_swap_data_list(
    actually_get: bool,
) -> crate::utils::error::Result<Option<MemHarvest>> {
    if !actually_get {
        return Ok(None);
    }

    let memory = heim::memory::swap().await?;

    Ok(Some(MemHarvest {
        mem_total_in_mb: memory.total().get::<heim::units::information::megabyte>(),
        mem_used_in_mb: memory.used().get::<heim::units::information::megabyte>(),
    }))
}
