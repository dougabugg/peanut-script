use super::{BytesIO, BytesReadError, Module};

use crate::datamodel::{Record, Value};

pub struct Program {
    modules: Vec<Module>,
}

impl Program {
    pub fn into_record(self) -> Record {
        let len = self.modules.len();
        let record = Record::empty(len);
        let mut refs = Vec::new();
        for (i, module) in self.modules.into_iter().enumerate() {
            let (module, mrefs) = module.into_record();
            refs.push((module.clone(), mrefs));
            record.set(i, module.into());
        }
        for (module, mrefs) in refs {
            for (l, r) in mrefs {
                module.set(l, record.get(r).unwrap_or(Value::None));
            }
        }
        record
    }
}

impl BytesIO for Program {
    fn read<'a>(b: &'a [u8]) -> Result<(&'a [u8], Self), BytesReadError<'a>> {
        let (b, modules) = <Vec<Module> as BytesIO>::read(b)?;
        Ok((b, Program { modules }))
    }
    fn write<'a>(t: &Self, b: &'a mut [u8]) -> Option<&'a mut [u8]> {
        <Vec<Module> as BytesIO>::write(&t.modules, b)
    }
}
