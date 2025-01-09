//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

pub const MAX_CONTAINERS: usize = 1; // todo fix

#[allow(improper_ctypes)]
extern "C" {
    static mut __SYSTEM_CONFIG_START: SystemConfig<MAX_CONTAINERS>;
}

/* 
 * Configuration about the entire system
 */
pub struct SystemConfig<const NUM_OF_CONTAINERS: usize> {
    pub num_of_cpus: usize,
    pub num_of_containers: usize,
    pub container: [ContainerConfig; NUM_OF_CONTAINERS],
}

/* 
 * Configuration referenced when creating a container
 */
pub struct ContainerConfig {
    pub id: usize,          // Container ID
    pub num_of_cpus: usize, // Number of CPUs in Container
    pub cores: u64,         // Bitmap of cores
    pub bsp: usize,         // BootStrap Processor in Container
}

/* 
 * Get Container ID from Core ID
 * notice: Use only when creating a container
 */
pub fn get_container_id(core_id: usize) -> usize {
    unsafe {
        let config = &__SYSTEM_CONFIG_START;
        match config.container
            .iter()
            .find(|c| (c.cores & (1 << core_id as u64) != 0)) {
                Some(c) => c.id,
                None => 0,
            }
    }
}

pub fn get_container_bsp(container_id: usize) -> usize {
    unsafe {
        let config = &__SYSTEM_CONFIG_START;
        config.container[container_id-1].bsp // todo fix
    }
}

pub fn get_num_of_cpus() -> usize {
    unsafe {
        let config = &__SYSTEM_CONFIG_START;
        config.num_of_cpus
    }
}