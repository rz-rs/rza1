#!/usr/bin/env python3
import re, sys, typing, xml.dom
from typing import NamedTuple, Dict, List

header_files = sys.argv[1:]

class Peripheral(NamedTuple):
    name: str
    st_name: str
    base: str

class Field(NamedTuple):
    name: str
    offset: int     # measured in bytes
    width: int      # measured in bits

class Struct(NamedTuple):
    fields: List[Field]

all_peripherals: Dict[str, Peripheral] = {}
all_peripherals_list: List[Peripheral] = []
all_field_masks: Dict[str, Dict[str, int]] = {}
all_structs: Dict[str, Struct] = {}

COMMENT_RE = re.compile(r'/\*.*?\*/', re.DOTALL)

# e.g., `#define SDG0    (*(struct st_sdg     *)0xFCFF4800uL)`
PERIPHERAL_RE = re.compile(
    r'#define\s+([A-Z0-9_]+)\s+\(\*\(struct\s+([a-z0-9_+]+)\s*\*\)0x([A-F0-9]+)uL\)')

# e.g., `#define MTU2_TIORH_4_IOA     (0x0Fu)`
#                ^^^^^^^^^^ ^^^^^
FIELD_MASK_RE = re.compile(
    r'#define\s+([A-Z0-9]+_[a-zA-Z0-9]+)_([a-zA-Z0-9_]+)\s+\(0x([0-9A-F]+)uL?\)')

STRUCT_RE = re.compile(
    r'typedef struct ([0-9a-z_]+)\s*\{(.*?)\}', re.DOTALL)

FIELD_RE = re.compile(r'(volatile uint|union iodefine_reg)(8|16|32)[0-9_]*_t\s+([a-zA-Z0-9_]+)')
FIELD_PADDING_RE = re.compile(r'volatile uint8_t\s+dummy[0-9]*\[([0-9]+)\]')

DIGIT_RE = re.compile(r'[0-9]+')

for header_file in header_files:
    print(header_file)

    with open(header_file, 'r', encoding='Shift_JIS') as f:
        source = f.read()

    source = COMMENT_RE.sub('', source)

    matches = PERIPHERAL_RE.findall(source)
    for match in matches:
        name = match[0]
        if name in all_peripherals:
            raise RuntimeError("Peripheral '%s' defined twice" % name)
        all_peripherals[name] = Peripheral(
            name=name,
            st_name=match[1],
            base=match[2])
        all_peripherals_list.append(all_peripherals[name])

    matches = FIELD_MASK_RE.findall(source)
    for match in matches:
        peripheral_name = match[0]
        name = match[1]
        mask = int(match[2], 16)
        if mask == 0:
            print("Field '%s_%s' has null mask, ignoring" % (peripheral_name, name))
            continue
        if peripheral_name not in all_field_masks:
            all_field_masks[peripheral_name] = {}
        all_field_masks[peripheral_name][name] = mask

    matches = STRUCT_RE.findall(source)
    for match in matches:
        st_name = match[0]
        st_content = match[1]

        offset = 0
        fields = []

        field_fragments = st_content.split(';')
        for field_fragment in field_fragments:
            field_fragment = field_fragment.strip()

            if field_fragment == '':
                continue

            match = FIELD_PADDING_RE.fullmatch(field_fragment)
            if match:
                offset += int(match[1])
                continue

            match = FIELD_RE.fullmatch(field_fragment)
            if match:
                width = int(match[2])
                name = match[3]
                fields.append(Field(name=name, width=width, offset=offset))
                offset += width // 8
                continue

            raise RuntimeError("can't parse the field: '%s'" % field_fragment)

        all_structs[st_name] = Struct(fields=fields)

# For each struct, decide the primary peripheral
struct_pri_peripheral: Dict[str, str] = {}

for peripheral in all_peripherals_list:
    st_name = peripheral.st_name
    if st_name not in struct_pri_peripheral:
        struct_pri_peripheral[st_name] = peripheral.name

# Generate SVD
dom = xml.dom.getDOMImplementation('minidom')
svd_doc = dom.createDocument(xml.dom.EMPTY_NAMESPACE, 'device', None)

svd_root = svd_doc.documentElement
svd_root.setAttribute('schemaVersion', '1.1')
svd_root.setAttribute('xmlns:xs', 'http://www.w3.org/2001/XMLSchema-instance')
svd_root.setAttribute('xs:noNamespaceSchemaLocation', 'CMSIS-SVD.xsd')

def add_element(parent, name: str, value: str):
    e = svd_doc.createElement(name)
    e.appendChild(svd_doc.createTextNode(value))
    parent.appendChild(e)

def int_msb(i: int) -> int:
    return i.bit_length() - 1
def int_lsb(i: int) -> int:
    if i & 1 == 1:
        return 0
    else:
        return int_lsb(i // 2) + 1

add_element(svd_root, 'vendor', 'Renesas')
add_element(svd_root, 'name', 'RZ/A1H')
add_element(svd_root, 'addressUnitBits', '8')
add_element(svd_root, 'width', '32')
add_element(svd_root, 'size', '32')
add_element(svd_root, 'access', 'read-write')
add_element(svd_root, 'resetValue', '0x00000000')
add_element(svd_root, 'resetMask', '0xFFFFFFFF')

svd_peripherals = svd_doc.createElement('peripherals')
svd_root.appendChild(svd_peripherals)

for peripheral in all_peripherals_list:
    svd_peripheral = svd_doc.createElement('peripheral')
    svd_peripherals.appendChild(svd_peripheral)

    add_element(svd_peripheral, 'name', peripheral.name)
    add_element(svd_peripheral, 'description', peripheral.name)
    add_element(svd_peripheral, 'baseAddress', '0x' + peripheral.base)

    st_name = peripheral.st_name
    struct = all_structs[st_name]
    pri_peripheral = struct_pri_peripheral[st_name]

    # If it's not the primary peripheral for the structure, derive the
    # definitions from the primary peripheral
    if pri_peripheral != peripheral.name:
        svd_peripheral.setAttribute('derivedFrom', pri_peripheral)
        continue

    # Prefixes to remove from register names
    # e.g., `OSTMnCNT` → `CNT`
    stripped_prefixes = [
        DIGIT_RE.sub('n', peripheral.name) + '_',
        DIGIT_RE.sub('n', peripheral.name),
    ]

    if peripheral.name == 'ADC':
        stripped_prefixes = ['AD']
    elif peripheral.name == 'LIN0':
        stripped_prefixes = ['RLN3n']

    def strip_peripheral_name(name: str) -> str:
        for stripped_prefix in stripped_prefixes:
            if name.startswith(stripped_prefix):
                new_name = name[len(stripped_prefix):]
                print("Stripping name '%s' → '%s'" % (name, new_name))
                return new_name
        return name

    svd_registers = svd_doc.createElement('registers')
    svd_peripheral.appendChild(svd_registers)

    for field in struct.fields:
        svd_register = svd_doc.createElement('register')
        svd_registers.appendChild(svd_register)

        name = strip_peripheral_name(field.name)

        add_element(svd_register, 'name', name)
        add_element(svd_register, 'description', field.name)
        add_element(svd_register, 'addressOffset', str(field.offset))
        add_element(svd_register, 'width', str(field.width))
        add_element(svd_register, 'size', str(field.width))

        field_masks = all_field_masks.get(peripheral.name + '_' + field.name)
        if field_masks:
            svd_fields = svd_doc.createElement('fields')
            svd_register.appendChild(svd_fields)
            for subfield_name, subfield_mask in field_masks.items():
                svd_field = svd_doc.createElement('field')
                svd_fields.appendChild(svd_field)

                add_element(svd_field, 'name', strip_peripheral_name(subfield_name))
                add_element(svd_field, 'msb', str(int_msb(subfield_mask)))
                add_element(svd_field, 'lsb', str(int_lsb(subfield_mask)))

with open('rza1.svd', 'w') as f:
    svd_doc.writexml(f, addindent='  ', newl='\n')
