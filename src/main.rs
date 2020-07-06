use sleigh::{preprocess, Spec, State};

fn main() {
    for (dir, file) in &[
        (
            "/opt/ghidra/ghidra_9.1.2_PUBLIC/Ghidra/Processors/x86/data/languages/",
            "x86-64.slaspec",
        ),
        (
            "/opt/ghidra/ghidra_9.1.2_PUBLIC/Ghidra/Processors/x86/data/languages/",
            "x86.slaspec",
        ),
        (
            "/opt/ghidra/ghidra_9.1.2_PUBLIC/Ghidra/Processors/ARM/data/languages/",
            "ARM8_le.slaspec",
        ),
        (
            "/opt/ghidra/ghidra_9.1.2_PUBLIC/Ghidra/Processors/AARCH64/data/languages/",
            "AARCH64.slaspec",
        ),
        (
            "/opt/ghidra/ghidra_9.1.2_PUBLIC/Ghidra/Processors/Dalvik/data/languages/",
            "Dalvik.slaspec",
        ),
    ] {
        println!("{}{}", dir, file);
        let res = preprocess(dir, file);
        let spec = Spec::parse(&res);

        macro_rules! test_disassembly {
            ($data:expr) => {
                let state = State::new(&spec, $data);
                let c = state.match_constructor(None);
                if let Some(c) = c {
                    println!("{}:{}", c.header.table, c.header.mnemonic);
                }
            };
        }

        test_disassembly!(&[0x90]);
        test_disassembly!(&[0x50]);
        test_disassembly!(&[0x51]);
        test_disassembly!(&[0x0f, 0x05]);
        test_disassembly!(&[0x48, 0xB8, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }
}
