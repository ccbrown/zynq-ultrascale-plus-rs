#!/usr/bin/env python3
import json
import os
import shutil
import subprocess

crate_path = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))

src_path = os.path.join(crate_path, 'src')
if os.path.isdir(src_path):
    shutil.rmtree(src_path)

in_path = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), 'modules.json')
with open(in_path, 'r') as f:
    module_types = json.load(f)

os.mkdir(src_path)


def rust_name_for_register(name):
    return name.lower().removesuffix('_register')


def rust_primitive_for_register_width(width):
    if width <= 8:
        return 'u8', 1
    elif width <= 16:
        return 'u16', 2
    elif width <= 32:
        return 'u32', 4
    elif width <= 64:
        return 'u64', 8
    raise RuntimeError(f'unsupported register width: {width}')


READ_ONLY_TYPES = set(['clronrd', 'ro', 'raz'])
WRITE_ONLY_TYPES = set(['wo'])
READ_WRITE_TYPES = set(['clronwr', 'rw', 'wtc', 'rwso'])


def rust_type_for_register(register):
    typ = register['type']
    if typ in READ_ONLY_TYPES:
        return 'ReadOnly'
    elif typ in WRITE_ONLY_TYPES:
        return 'WriteOnly'
    elif typ in READ_WRITE_TYPES:
        return 'ReadWrite'
    elif typ == 'mixed':
        has_read_only_types = False
        has_write_only_types = False
        has_read_write_types = False
        for field in register['fields']:
            if field['type'] in READ_ONLY_TYPES:
                has_read_only_types = True
            if field['type'] in WRITE_ONLY_TYPES:
                has_write_only_types = True
            if field['type'] in READ_WRITE_TYPES:
                has_read_write_types = True
        if has_read_only_types and not (has_read_write_types or has_write_only_types):
            return 'ReadOnly'
        elif has_write_only_types and not (has_read_write_types or has_read_only_types):
            return 'WriteOnly'
        elif has_read_write_types and not (has_write_only_types or has_read_only_types):
            return 'ReadWrite'
        else:
            return 'Aliased'
    raise RuntimeError(f'unsupported register type: {typ}')


def should_create_bitfield(fields, register_width):
    if len(fields) == 0:
        return False
    elif len(fields) > 1:
        return True
    return fields[0]['name'] != '_' and fields[0]['bits'] != f'{register_width-1}:0'


def rust_bitfield_type(name):
    parts = rust_name_for_register(name).split('_')
    return ''.join([p.capitalize() for p in parts])


WARNING = "// This file was automatically generated. Don't edit it directly!"


def _bitfield_registration_impl(register, read_fields=True, write_fields=True, suffix='', comma=''):
    out = []
    primitive, _ = rust_primitive_for_register_width(register['width'])
    out.append(f'    pub {rust_bitfield_type(register["name"])}{suffix} [')
    reserved_count = 0
    for field in register['fields']:
        allowed_types = READ_WRITE_TYPES
        if read_fields:
            allowed_types = allowed_types | READ_ONLY_TYPES
        if write_fields:
            allowed_types = allowed_types | WRITE_ONLY_TYPES
        if field['type'] not in allowed_types:
            continue
        bit_parts = field['bits'].split(':')
        offset = int(bit_parts[-1])
        numbits = 1
        if len(bit_parts) > 1:
            numbits = int(bit_parts[0]) - offset + 1
        field_name = field["name"].upper()
        if field_name == 'RESERVED':
            field_name += f'{reserved_count}'
            reserved_count += 1
        elif field_name[0].isdigit():
            field_name = f'_{field_name}'
        out.append(f'        {field_name} OFFSET({offset}) NUMBITS({numbits}) [],')
    out.append(f'    ]{comma}')
    return out


def bitfield_registration(register):
    primitive, _ = rust_primitive_for_register_width(register['width'])
    out = [
        'register_bitfields! [',
        f'    {primitive},',
    ]
    if rust_type_for_register(register) == 'Aliased':
        out.extend(_bitfield_registration_impl(register, read_fields=True, write_fields=False, suffix='R', comma=','))
        out.extend(_bitfield_registration_impl(register, read_fields=False, write_fields=True, suffix='W'))
    else:
        out.extend(_bitfield_registration_impl(register))
    out.append('];')
    return out


rust_modules = []

for module_type_name, module_type in module_types.items():
    out = []

    for module_name, module in module_type['modules'].items():
        desc = module['description'].strip()
        if desc != '':
            out.append('/// ' + desc.replace('\n', ' ').replace('\r', ''))
        out.append(f'pub static mut {module_name.upper()}: *mut Registers = 0x{module["base_address"]:08x} as *mut Registers;')

    out.extend([
        'register_structs! {',
        '    pub Registers {',
    ])

    rust_types_used = set()
    registers = module_type['registers']

    padding_field_count = 0
    offset = 0
    for register in registers:
        rust_type = rust_type_for_register(register)
        rust_types_used.add(rust_type)

        address = register['address']
        primitive, primitive_bytes = rust_primitive_for_register_width(register['width'])

        if should_create_bitfield(register['fields'], register['width']):
            if rust_type == 'Aliased':
                rust_type = f'{rust_type}<{primitive}, {rust_bitfield_type(register["name"])}R::Register, {rust_bitfield_type(register["name"])}W::Register>'
            else:
                rust_type = f'{rust_type}<{primitive}, {rust_bitfield_type(register["name"])}::Register>'
        else:
            rust_type = f'{rust_type}<{primitive}>'

        if address < offset:
            raise RuntimeError(f'address of {module_type_name}.{register["name"]} is lower than expected')
        elif address > offset:
            out.append(f'        (0x{offset:08x} => _padding{offset}),')
            offset = address
            padding_field_count += 1

        desc = register['description'].strip()
        if desc != '':
            out.append('        /// ' + desc)

        out.append(f'        (0x{offset:08x} => pub {rust_name_for_register(register["name"])}: {rust_type}),')

        offset += primitive_bytes

    field_count = padding_field_count + len(registers)
    if field_count > 100:
        # Taking away this check crashes rustc. ;_;
        print(f'skipping {module_type_name} because it has too many fields to handle ({field_count})')
        continue

    out.extend([
        f'        (0x{offset:08x} => @END),',
        '    }',
        '}',
    ])

    for register in registers:
        if should_create_bitfield(register['fields'], register['width']):
            out.extend(bitfield_registration(register))

    out.insert(0, f'use tock_registers::registers::{{{",".join(rust_types_used)}}};')
    out.insert(0, WARNING)

    rust_module = module_type_name.lower()
    with open(os.path.join(src_path, rust_module + '.rs'), 'w') as f:
        f.write('\n'.join(out))
    rust_modules.append(rust_module)

out = [
    WARNING,
    '#![no_std]',
    '#![recursion_limit = "512"]',
    '#[macro_use] extern crate tock_registers;',
]

for name in rust_modules:
    out.append(f'pub mod {name};')

with open(os.path.join(src_path, 'lib.rs'), 'w') as f:
    f.write('\n'.join(out))

subprocess.check_call(['cargo', 'fmt'], cwd=crate_path)
subprocess.check_call(['cargo', 'check'], cwd=crate_path)
