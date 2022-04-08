use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

use gca_runtime::Memory;
pub use pwasm_utils::rules::Rules;

#[derive(Debug)]
pub enum Error {
    ParityWasmError(parity_wasm::elements::Error),
    InjectError,
}

impl From<parity_wasm::elements::Module> for Error {
    fn from(_: parity_wasm::elements::Module) -> Self {
        Error::InjectError
    }
}

impl From<parity_wasm::elements::Error> for Error {
    fn from(e: parity_wasm::elements::Error) -> Error {
        Error::ParityWasmError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn inject_gas(code: &[u8], rules: impl Rules) -> Result<Vec<u8>> {
    let module = parity_wasm::deserialize_buffer(code)?;

    let module = pwasm_utils::inject_gas_counter(module, &rules, "_gca_gas")?;

    Ok(parity_wasm::serialize(module)?)
}

pub struct GcaMeasurerHost<M> {
    gas: u64,
    gas_limit: u64,
    func_def: Vec<gca_runtime::FuncDefine>,
    marker_b: PhantomData<M>,
}

impl<M> GcaMeasurerHost<M> {
    pub fn new(gas_limit: u64) -> Self {
        let func_def = vec![];

        Self {
            gas: 0,
            gas_limit,
            func_def,
            marker_b: PhantomData,
        }
    }

    pub fn gas(&self) -> u64 {
        self.gas
    }
}

#[derive(Debug)]
pub enum GcaMeasurerHostError {
    ErrArgumentsFormat,
    ErrCalledName,
    ExccedGasLimit,
}

impl From<GcaMeasurerHostError> for Box<dyn Debug + Send + Sync> {
    fn from(e: GcaMeasurerHostError) -> Self {
        Box::new(e)
    }
}

impl Display for GcaMeasurerHostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl<M: Memory + 'static> gca_runtime::Host<M> for GcaMeasurerHost<M> {
    fn resolve_functions(&self) -> &[gca_runtime::FuncDefine] {
        &self.func_def
    }

    fn set_memory(&mut self, _memory: M) {}

    fn call_func(
        &mut self,
        name: &str,
        args: &[gca_runtime::Val],
    ) -> std::result::Result<Option<gca_runtime::Val>, Box<dyn Debug + Send + Sync>> {
        if name != "gas" {
            return Err(GcaMeasurerHostError::ErrCalledName.into());
        }

        if args.len() != 1 {
            return Err(GcaMeasurerHostError::ErrArgumentsFormat.into());
        }

        if let Some(gca_runtime::Val::I32(i)) = args.get(0) {
            // TODO: Add exccess to exit Execute.
            let step_gas = *i as u64;

            self.gas += step_gas;

            if self.gas > self.gas_limit {
                return Err(GcaMeasurerHostError::ExccedGasLimit.into());
            }
        } else {
            return Err(GcaMeasurerHostError::ErrArgumentsFormat.into());
        }

        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gas() {

    }
}
