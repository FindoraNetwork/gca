use std::collections::BTreeMap;

use wasmi::{ImportsBuilder, ModuleInstance};

use crate::{Backend, Host, ModuleInfo, Result};

use super::{ModuleHostImport, WasmiExternal, WasmiInstance, WasmiMemory, WasmiModule};

pub struct WasmiBackend {
    pub(crate) host_idxs: BTreeMap<usize, (usize, &'static str)>,
    pub(crate) hosts: Vec<Box<dyn Host<WasmiMemory>>>,
}

impl Backend for WasmiBackend {
    type Memory = WasmiMemory;

    type Module = WasmiModule;

    type Instance = WasmiInstance;

    fn new() -> Self {
        Self {
            host_idxs: BTreeMap::new(),
            hosts: Vec::new(),
        }
    }

    fn add_host(&mut self, name: &str, host: impl Host<Self::Memory>) {}

    fn instance(
        &mut self,
        module: &Self::Module,
        deps: &[ModuleInfo<'_, Self::Module>],
    ) -> Result<Self::Instance> {
        let external = WasmiExternal {};

        let imports = ImportsBuilder::new();

        for mi in deps {
            // imports.builder.push_resolver(mi.name, &mi.module.m);
        }

        let instance = ModuleInstance::new(&module.m, &imports)?.assert_no_start();

        Ok(WasmiInstance { external, instance })
    }
}
