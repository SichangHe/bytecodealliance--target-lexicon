use vstd::prelude::*;

fn main() {}

verus! {

pub enum LinuxGnuTriple {
    X86_64Gnu,
    X86_64Gnux32,
    Aarch64Gnu,
    Aarch64GnuIlp32,
    Aarch64BeGnu,
    Aarch64BeGnuIlp32,
}

pub enum ArchTag {
    X86_64,
    Aarch64,
    Aarch64Be,
}

pub enum EnvTag {
    Gnu,
    Gnux32,
    GnuIlp32,
}

pub open spec fn arch_of(case: LinuxGnuTriple) -> ArchTag {
    match case {
        LinuxGnuTriple::X86_64Gnu | LinuxGnuTriple::X86_64Gnux32 => ArchTag::X86_64,
        LinuxGnuTriple::Aarch64Gnu | LinuxGnuTriple::Aarch64GnuIlp32 => ArchTag::Aarch64,
        LinuxGnuTriple::Aarch64BeGnu | LinuxGnuTriple::Aarch64BeGnuIlp32 => ArchTag::Aarch64Be,
    }
}

pub open spec fn env_of(case: LinuxGnuTriple) -> EnvTag {
    match case {
        LinuxGnuTriple::X86_64Gnu
        | LinuxGnuTriple::Aarch64Gnu
        | LinuxGnuTriple::Aarch64BeGnu => EnvTag::Gnu,
        LinuxGnuTriple::X86_64Gnux32 => EnvTag::Gnux32,
        LinuxGnuTriple::Aarch64GnuIlp32 | LinuxGnuTriple::Aarch64BeGnuIlp32 => EnvTag::GnuIlp32,
    }
}

pub open spec fn arch_bytes(arch: ArchTag) -> Seq<u8> {
    match arch {
        ArchTag::X86_64 => seq![120u8, 56u8, 54u8, 95u8, 54u8, 52u8],
        ArchTag::Aarch64 => seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8],
        ArchTag::Aarch64Be => seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 95u8, 98u8, 101u8],
    }
}

pub open spec fn env_bytes(env: EnvTag) -> Seq<u8> {
    match env {
        EnvTag::Gnu => seq![103u8, 110u8, 117u8],
        EnvTag::Gnux32 => seq![103u8, 110u8, 117u8, 120u8, 51u8, 50u8],
        EnvTag::GnuIlp32 => seq![103u8, 110u8, 117u8, 95u8, 105u8, 108u8, 112u8, 51u8, 50u8],
    }
}

pub open spec fn middle_bytes() -> Seq<u8> {
    seq![
        45u8, 117u8, 110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8,
        120u8, 45u8,
    ]
}

pub open spec fn elf_suffix() -> Seq<u8> {
    seq![45u8, 101u8, 108u8, 102u8]
}

pub open spec fn canonical_bytes(case: LinuxGnuTriple) -> Seq<u8> {
    arch_bytes(arch_of(case)) + middle_bytes() + env_bytes(env_of(case))
}

pub open spec fn explicit_elf_bytes(case: LinuxGnuTriple) -> Seq<u8> {
    canonical_bytes(case) + elf_suffix()
}

pub proof fn canonical_table()
    ensures
        canonical_bytes(LinuxGnuTriple::X86_64Gnu)
            == seq![120u8, 56u8, 54u8, 95u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
                111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8],
        canonical_bytes(LinuxGnuTriple::X86_64Gnux32)
            == seq![120u8, 56u8, 54u8, 95u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
                111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 120u8, 51u8, 50u8],
        canonical_bytes(LinuxGnuTriple::Aarch64Gnu)
            == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
                111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8],
        canonical_bytes(LinuxGnuTriple::Aarch64GnuIlp32)
            == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
                111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 95u8, 105u8, 108u8, 112u8, 51u8, 50u8],
        canonical_bytes(LinuxGnuTriple::Aarch64BeGnu)
            == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 95u8, 98u8, 101u8, 45u8, 117u8,
                110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8],
        canonical_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32)
            == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 95u8, 98u8, 101u8, 45u8, 117u8,
                110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 95u8, 105u8, 108u8, 112u8, 51u8, 50u8],
{
    assert(canonical_bytes(LinuxGnuTriple::X86_64Gnu)
        == seq![120u8, 56u8, 54u8, 95u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8, 111u8,
            119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8]);
    assert(canonical_bytes(LinuxGnuTriple::X86_64Gnux32)
        == seq![120u8, 56u8, 54u8, 95u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8, 111u8,
            119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 120u8, 51u8, 50u8]);
    assert(canonical_bytes(LinuxGnuTriple::Aarch64Gnu)
        == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
            111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8]);
    assert(canonical_bytes(LinuxGnuTriple::Aarch64GnuIlp32)
        == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 45u8, 117u8, 110u8, 107u8, 110u8,
            111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 95u8, 105u8, 108u8, 112u8, 51u8, 50u8]);
    assert(canonical_bytes(LinuxGnuTriple::Aarch64BeGnu)
        == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 95u8, 98u8, 101u8, 45u8, 117u8,
            110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8]);
    assert(canonical_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32)
        == seq![97u8, 97u8, 114u8, 99u8, 104u8, 54u8, 52u8, 95u8, 98u8, 101u8, 45u8, 117u8,
            110u8, 107u8, 110u8, 111u8, 119u8, 110u8, 45u8, 108u8, 105u8, 110u8, 117u8, 120u8, 45u8, 103u8, 110u8, 117u8, 95u8, 105u8, 108u8, 112u8, 51u8, 50u8]);
}

pub proof fn explicit_elf_suffix_cases()
    ensures
        explicit_elf_bytes(LinuxGnuTriple::X86_64Gnu)
            == canonical_bytes(LinuxGnuTriple::X86_64Gnu) + elf_suffix(),
        explicit_elf_bytes(LinuxGnuTriple::X86_64Gnux32)
            == canonical_bytes(LinuxGnuTriple::X86_64Gnux32) + elf_suffix(),
        explicit_elf_bytes(LinuxGnuTriple::Aarch64Gnu)
            == canonical_bytes(LinuxGnuTriple::Aarch64Gnu) + elf_suffix(),
        explicit_elf_bytes(LinuxGnuTriple::Aarch64GnuIlp32)
            == canonical_bytes(LinuxGnuTriple::Aarch64GnuIlp32) + elf_suffix(),
        explicit_elf_bytes(LinuxGnuTriple::Aarch64BeGnu)
            == canonical_bytes(LinuxGnuTriple::Aarch64BeGnu) + elf_suffix(),
        explicit_elf_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32)
            == canonical_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32) + elf_suffix(),
{
    assert(explicit_elf_bytes(LinuxGnuTriple::X86_64Gnu)
        == canonical_bytes(LinuxGnuTriple::X86_64Gnu) + elf_suffix());
    assert(explicit_elf_bytes(LinuxGnuTriple::X86_64Gnux32)
        == canonical_bytes(LinuxGnuTriple::X86_64Gnux32) + elf_suffix());
    assert(explicit_elf_bytes(LinuxGnuTriple::Aarch64Gnu)
        == canonical_bytes(LinuxGnuTriple::Aarch64Gnu) + elf_suffix());
    assert(explicit_elf_bytes(LinuxGnuTriple::Aarch64GnuIlp32)
        == canonical_bytes(LinuxGnuTriple::Aarch64GnuIlp32) + elf_suffix());
    assert(explicit_elf_bytes(LinuxGnuTriple::Aarch64BeGnu)
        == canonical_bytes(LinuxGnuTriple::Aarch64BeGnu) + elf_suffix());
    assert(explicit_elf_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32)
        == canonical_bytes(LinuxGnuTriple::Aarch64BeGnuIlp32) + elf_suffix());
}

pub proof fn field_table()
    ensures
        arch_of(LinuxGnuTriple::X86_64Gnu) == ArchTag::X86_64,
        arch_of(LinuxGnuTriple::X86_64Gnux32) == ArchTag::X86_64,
        arch_of(LinuxGnuTriple::Aarch64Gnu) == ArchTag::Aarch64,
        arch_of(LinuxGnuTriple::Aarch64GnuIlp32) == ArchTag::Aarch64,
        arch_of(LinuxGnuTriple::Aarch64BeGnu) == ArchTag::Aarch64Be,
        arch_of(LinuxGnuTriple::Aarch64BeGnuIlp32) == ArchTag::Aarch64Be,
        env_of(LinuxGnuTriple::X86_64Gnu) == EnvTag::Gnu,
        env_of(LinuxGnuTriple::X86_64Gnux32) == EnvTag::Gnux32,
        env_of(LinuxGnuTriple::Aarch64Gnu) == EnvTag::Gnu,
        env_of(LinuxGnuTriple::Aarch64GnuIlp32) == EnvTag::GnuIlp32,
        env_of(LinuxGnuTriple::Aarch64BeGnu) == EnvTag::Gnu,
        env_of(LinuxGnuTriple::Aarch64BeGnuIlp32) == EnvTag::GnuIlp32,
{
    assert(arch_of(LinuxGnuTriple::X86_64Gnu) == ArchTag::X86_64);
    assert(arch_of(LinuxGnuTriple::X86_64Gnux32) == ArchTag::X86_64);
    assert(arch_of(LinuxGnuTriple::Aarch64Gnu) == ArchTag::Aarch64);
    assert(arch_of(LinuxGnuTriple::Aarch64GnuIlp32) == ArchTag::Aarch64);
    assert(arch_of(LinuxGnuTriple::Aarch64BeGnu) == ArchTag::Aarch64Be);
    assert(arch_of(LinuxGnuTriple::Aarch64BeGnuIlp32) == ArchTag::Aarch64Be);
    assert(env_of(LinuxGnuTriple::X86_64Gnu) == EnvTag::Gnu);
    assert(env_of(LinuxGnuTriple::X86_64Gnux32) == EnvTag::Gnux32);
    assert(env_of(LinuxGnuTriple::Aarch64Gnu) == EnvTag::Gnu);
    assert(env_of(LinuxGnuTriple::Aarch64GnuIlp32) == EnvTag::GnuIlp32);
    assert(env_of(LinuxGnuTriple::Aarch64BeGnu) == EnvTag::Gnu);
    assert(env_of(LinuxGnuTriple::Aarch64BeGnuIlp32) == EnvTag::GnuIlp32);
}

} // verus!
