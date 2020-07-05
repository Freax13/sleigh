use sleigh::{preprocess, Spec};

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
        let res = preprocess(dir, file);
        let _spec = Spec::parse(&res);
    }
}
