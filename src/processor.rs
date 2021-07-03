use crate::decoder::{Decoder, DecodedInst, ArgType};
use crate::execution_path::{execution_path_factory, ExecPath, ArgVal};
use crate::register::RegFile;
use std::collections::HashMap;

pub struct Processor {
    pc: usize,
    decoder: Decoder,
    paths: HashMap<String, Box<dyn ExecPath>>,
    regfile: RegFile,
    // resultbus
}

impl Processor {
    pub fn new() -> Self {
        Self {
            pc: 0,
            decoder: Decoder::new(),
            paths: HashMap::new(),
            regfile: RegFile::new(),
        }
    }
    /// Add an execution path to the processor.
    pub fn add_path(&mut self, name: String, func: String) -> Result<(), String> {
        let path = execution_path_factory(&name, &func)?;
        let insts = path.list_inst();
        if let Some(prev) = self.paths.insert(name.clone(), path) {
            let msg = format!("Already has a execution path with name {}", prev.get_name());
            Err(msg)
        } else {
            self.decoder.register(insts, name)
        }
    }
    pub fn fetch_line(&self) -> usize {
        self.pc
    }
    pub fn next_cycle(&mut self, inst: &str) -> Result<(), String> {
        let inst = self.decoder.decode(inst)?;
        let args = inst.get_args();
        let mut arg_vals = Vec::with_capacity(args.len());
        if args.len() == 0 {
            let msg = String::from("Expcet more than one argument");
            return Err(msg);
        }
        let mut start = 0;
        let mut dest = None;
        if let ArgType::Reg(idx) = args[0]{
            start = 1;
            dest = Some(idx);
        }

        // Mapping arguments from types to data
        for arg in args[start..].iter() {
            let val;
            match *arg {
                ArgType::Reg(idx) => {
                    val = self.regfile.read(idx);
                },
                ArgType::Imm(imm) => {
                    val = ArgVal::Imm(imm);
                },
            }
            arg_vals.push(val);
        }

        // Searching for a suitable station to issue the instruction
        for name in inst.get_stations().iter() {
            // Find a reservation station by name
            if let Some(station) = self.paths.get_mut(name) {
                if let Ok(tag) = station.issue(inst.get_name(), &arg_vals) {
                    if let Some(idx) = dest {
                        self.regfile.rename(idx, tag);
                    }
                }
            }
        }
        todo!();
    }
}
