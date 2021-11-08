use crate::x86::register::Register;
use crate::x86::operand::Operand;
use crate::x86::memory::Scale;
use crate::x86::register::GPReg32;
use crate::x86::memory::Memory;
use crate::x86::immediate::Immediate;
use crate::codegen::Codegen;

pub struct ExeWriter {

}

impl ExeWriter {
    pub fn new() -> Self {
        ExeWriter {}
    }

    pub fn write(&mut self) {
        let mut out_data = Vec::new();

        let mut writer = object::write::pe::Writer::new(
            false,
            0x1000, 
            0x1000,
            &mut out_data
        );

        writer.reserve_dos_header_and_stub();
        writer.reserve_nt_headers(16);
        writer.reserve_section_headers(1);

        writer.reserve_virtual_until(0x1000);
        let text_range = writer.reserve_text_section(0x1000);

        //writer.reserve_virtual_until(0x2000);
        //let data_range = writer.reserve_data_section(0x1000, 0x1000);

        writer.write_empty_dos_header()
            .unwrap();

        writer.write_nt_headers(object::write::pe::NtHeaders {
            machine: object::pe::IMAGE_FILE_MACHINE_I386,
            time_date_stamp: 0,
            characteristics: object::pe::IMAGE_FILE_EXECUTABLE_IMAGE |
                            object::pe::IMAGE_FILE_LINE_NUMS_STRIPPED |
                            object::pe::IMAGE_FILE_LOCAL_SYMS_STRIPPED | 
                            object::pe::IMAGE_FILE_32BIT_MACHINE,
            major_linker_version: 0,
            minor_linker_version: 0,
            address_of_entry_point: text_range.virtual_address as u32,
            image_base: 0x00300000,
            major_operating_system_version: 0,
            minor_operating_system_version: 0,
            major_image_version: 0,
            minor_image_version: 0,
            major_subsystem_version: 5,
            minor_subsystem_version: 1,
            subsystem: object::pe::IMAGE_SUBSYSTEM_WINDOWS_CUI,
            dll_characteristics: 0,
            size_of_stack_reserve: 0,
            size_of_stack_commit: 0,
            size_of_heap_reserve: 0,
            size_of_heap_commit: 0,
        });

        writer.write_section_headers();

        writer.pad_until(text_range.file_offset);

        let mut codegen = Codegen::new();

        codegen.mov(
            Operand::Memory(Memory::BaseIndexScaleDisplacement(
                Register::GPR32(GPReg32::EDI),
                Register::GPR32(GPReg32::ESI),
                Scale::X4,
                Immediate::U32(0x12345678)
            )),
            Operand::Register(Register::GPR32(GPReg32::EBP))
        );

        writer.write_section(text_range.file_offset, codegen.get_bytes() /*&[0xEB, 0xFE, 0xC3]*/);

        //writer.pad_until(data_range.file_offset);
        //writer.write_section(data_range.file_offset, &[0xAB, 0xCD, 0xEF]);
        

        
        
        std::fs::write("compiled.exe", out_data).unwrap();
    }
}
