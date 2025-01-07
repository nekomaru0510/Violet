//SPDX-License-Identifier: MIT 
//SPDX-FileCopyrightText: 2025 Ryosuke Yamamoto <yama05rymy@gmail.com> 

pub const MAX_CONTAINERS: usize = 1; // todo fix
pub const MAX_CORES: usize = 2; // todo delete

#[allow(improper_ctypes)]
extern "C" {
    static mut __SYSTEM_CONFIG_START: SystemConfig<MAX_CONTAINERS, MAX_CORES>;
}

/* 
 * システム全体に関する情報
 */
pub struct SystemConfig<const NUM_OF_CONTAINERS: usize, const NUM_OF_CORES: usize> {
    pub num_of_cpus: usize,
    pub num_of_containers: usize,
    pub core2container: [usize; NUM_OF_CORES],              // todo delete
    pub container: [ContainerConfig; NUM_OF_CONTAINERS],
}

/* 
 * コンテナ生成時に参照される情報 
 */
pub struct ContainerConfig {
    pub id: usize,          // Container ID
    pub num_of_cpus: usize, // Number of CPUs in Container
    pub cores: u64,         // Bitmap of cores
    pub bsp: usize,         // BootStrap Processor in Container
}

pub fn get_container_id(core_id: usize) -> usize {
    unsafe {
        let config = &__SYSTEM_CONFIG_START;
        config.core2container[core_id]
    }
}

pub fn get_container_bsp(container_id: usize) -> usize {
    unsafe {
        let config = &__SYSTEM_CONFIG_START;
        config.container[container_id-1].bsp // todo fix
    }
}



