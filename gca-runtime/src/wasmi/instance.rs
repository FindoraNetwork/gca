use crate::{Instance, Result, Val};

use super::{WasmiExternal, WasmiMemory, WasmiModule};

pub struct WasmiInstance {
    pub(crate) instance: wasmi::ModuleRef,
    pub(crate) external: WasmiExternal,
}

impl Instance for WasmiInstance {
    type Memory = WasmiMemory;

    type Module = WasmiModule;

    fn call_func(&mut self, name: &str, parmas: &[Val]) -> Result<Option<Val>> {
        let args: Vec<wasmi::RuntimeValue> = parmas
            .iter()
            .map(|e| wasmi::RuntimeValue::from(e.clone()))
            .collect();

        self.instance
            .invoke_export(name, &args, &mut self.external)?;

        Ok(None)
    }

    fn get_memory(&self, name: &str) -> Option<Self::Memory> {
        if let Some(wasmi::ExternVal::Memory(m)) = self.instance.export_by_name(name) {
            Some(WasmiMemory { m })
        } else {
            None
        }
    }
}

impl From<Val> for wasmi::RuntimeValue {
    fn from(e: Val) -> Self {
        match e {
            Val::I32(i) => Self::I32(i),
            Val::I64(i) => Self::I64(i),
            Val::F32(i) => Self::F32(wasmi::nan_preserving_float::F32::from_bits(i)),
            Val::F64(i) => Self::F64(wasmi::nan_preserving_float::F64::from_bits(i)),
        }
    }
}

impl From<wasmi::RuntimeValue> for Val {
    fn from(e: wasmi::RuntimeValue) -> Self {
        match e {
            wasmi::RuntimeValue::I32(i) => Val::I32(i),
            wasmi::RuntimeValue::I64(i) => Val::I64(i),
            wasmi::RuntimeValue::F32(i) => Val::F32(i.to_bits()),
            wasmi::RuntimeValue::F64(i) => Val::F64(i.to_bits()),
        }
    }
}
