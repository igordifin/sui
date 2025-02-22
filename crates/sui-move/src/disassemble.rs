// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use move_binary_format::{binary_views::BinaryIndexedView, CompiledModule};
use move_cli::base;
use move_disassembler::disassembler::Disassembler;
use move_ir_types::location::Spanned;
use move_package::BuildConfig;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Disassemble {
    /// Path to a .mv file to disassemble
    #[clap(name = "module_path")]
    module_path: PathBuf,
}

impl Disassemble {
    pub fn execute(
        self,
        package_path: Option<PathBuf>,
        build_config: BuildConfig,
    ) -> anyhow::Result<()> {
        if base::reroot_path(Some(self.module_path.clone())).is_ok() {
            // disassembling bytecode inside the source package that produced it--use the source info
            let module_name = self
                .module_path
                .file_stem()
                .expect("Bad module path")
                .to_str()
                .expect("Cannot convert module name to string")
                .to_owned();
            move_cli::base::disassemble::Disassemble {
                interactive: false,
                package_name: None,
                module_or_script_name: module_name,
                debug: false,
            }
            .execute(package_path, build_config)?
        } else {
            // disassembling a bytecode file with no source info
            assert!(
                Path::new(&self.module_path).exists(),
                "Bath path to .mv file"
            );
            let mut bytes = Vec::new();
            let mut file = BufReader::new(File::open(self.module_path)?);
            file.read_to_end(&mut bytes)?;
            let module = CompiledModule::deserialize(&bytes)?;
            let view = BinaryIndexedView::Module(&module);
            let d = Disassembler::from_view(view, Spanned::unsafe_no_loc(()).loc)?;
            println!("{}", d.disassemble()?);
        }
        Ok(())
    }
}
