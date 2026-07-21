use std::str::FromStr;

use target_lexicon::{
    Aarch64Architecture, Architecture, BinaryFormat, Environment, OperatingSystem, Triple, Vendor,
};

#[derive(Clone, Copy, Debug)]
struct Case {
    canonical: &'static str,
    architecture: Architecture,
    environment: Environment,
}

const CASES: &[Case] = &[
    Case {
        canonical: "x86_64-unknown-linux-gnu",
        architecture: Architecture::X86_64,
        environment: Environment::Gnu,
    },
    Case {
        canonical: "x86_64-unknown-linux-gnux32",
        architecture: Architecture::X86_64,
        environment: Environment::Gnux32,
    },
    Case {
        canonical: "aarch64-unknown-linux-gnu",
        architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
        environment: Environment::Gnu,
    },
    Case {
        canonical: "aarch64-unknown-linux-gnu_ilp32",
        architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64),
        environment: Environment::GnuIlp32,
    },
    Case {
        canonical: "aarch64_be-unknown-linux-gnu",
        architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64be),
        environment: Environment::Gnu,
    },
    Case {
        canonical: "aarch64_be-unknown-linux-gnu_ilp32",
        architecture: Architecture::Aarch64(Aarch64Architecture::Aarch64be),
        environment: Environment::GnuIlp32,
    },
];

fn assert_linux_gnu_case(case: Case) {
    let parsed = Triple::from_str(case.canonical).expect("canonical Linux/GNU triple parses");
    assert_eq!(parsed.architecture, case.architecture, "{case:?}");
    assert_eq!(parsed.vendor, Vendor::Unknown, "{case:?}");
    assert_eq!(parsed.operating_system, OperatingSystem::Linux, "{case:?}");
    assert_eq!(parsed.environment, case.environment, "{case:?}");
    assert_eq!(parsed.binary_format, BinaryFormat::Elf, "{case:?}");
    assert_eq!(parsed.to_string(), case.canonical, "{case:?}");

    let explicit_elf = format!("{}-elf", case.canonical);
    let parsed_explicit =
        Triple::from_str(&explicit_elf).expect("explicit ELF variant also parses");
    assert_eq!(parsed_explicit, parsed, "{case:?}");
    assert_eq!(parsed_explicit.to_string(), case.canonical, "{case:?}");
}

#[test]
fn linux_gnu_family_bridge_cases() {
    for &case in CASES {
        assert_linux_gnu_case(case);
    }
}

#[test]
fn linux_gnu_family_bounded_adjacent_search() {
    let inputs = [
        "x86_64-unknown-linux-gnu",
        "x86_64-unknown-linux-gnu-elf",
        "x86_64-unknown-linux-gnux32",
        "x86_64-unknown-linux-gnux32-elf",
        "x86_64-unknown-linux-gnu_ilp32",
        "x86_64-unknown-linux-gnu_ilp32-elf",
        "aarch64-unknown-linux-gnu",
        "aarch64-unknown-linux-gnu-elf",
        "aarch64-unknown-linux-gnux32",
        "aarch64-unknown-linux-gnux32-elf",
        "aarch64-unknown-linux-gnu_ilp32",
        "aarch64-unknown-linux-gnu_ilp32-elf",
        "aarch64_be-unknown-linux-gnu",
        "aarch64_be-unknown-linux-gnu-elf",
        "aarch64_be-unknown-linux-gnux32",
        "aarch64_be-unknown-linux-gnux32-elf",
        "aarch64_be-unknown-linux-gnu_ilp32",
        "aarch64_be-unknown-linux-gnu_ilp32-elf",
    ];

    for input in inputs {
        let parsed = Triple::from_str(input).expect("bounded adjacent search input parses");
        assert_eq!(parsed.vendor, Vendor::Unknown, "{input}");
        assert_eq!(parsed.operating_system, OperatingSystem::Linux, "{input}");
        assert_eq!(parsed.binary_format, BinaryFormat::Elf, "{input}");

        let canonical = parsed.to_string();
        let reparsed = Triple::from_str(&canonical).expect("rendered triple reparses");
        assert_eq!(reparsed, parsed, "{input}");
    }

    println!(
        "bounded adjacent search checked {} Linux/GNU-family inputs",
        inputs.len()
    );
}
