#!/usr/bin/env python3
import re, sys
from typing import List, Dict

lib_path = sys.argv[1]

# Mapping from a module name to a Cargo feature
mod_to_cfg: Dict[str, str] = {
    'adc': 'adc',
    'bsc': 'bsc',
    'ceu': 'ceu',
    'cpg': 'cpg',
    'disc0': 'disc',
    'dmac': 'dmac',
    'dvdec1': 'dvdec',
    'ether': 'ether',
    'flctl': 'flctl',
    'gpio': 'gpio',
    'ieb': 'ieb',
    'inb': 'inb',
    'intc': 'intc',
    'irda': 'irda',
    'jcu': 'jcu',
    'l2c': 'l2c',
    'lin0': 'lin',
    'lvds': 'lvds',
    'mlb': 'mlb',
    'mmc': 'mmc',
    'mtu2': 'mtu2',
    'ostm0': 'ostm',
    'pfv0': 'pfv',
    'pwm': 'pwm',
    'riic0': 'riic',
    'romdec': 'romdec',
    'rscan0': 'rscan',
    'rspi0': 'rspi',
    'rtc': 'rtc',
    'scif0': 'scif',
    'scim0': 'scim',
    'scux': 'scux',
    'sdg0': 'sdg',
    'spdif': 'spdif',
    'spibsc0': 'spibsc',
    'ssif0': 'ssif',
    'usb200': 'usb20',
    'vdc50': 'vdc5',
    'wdt': 'wdt',
}

cfgs = set(mod_to_cfg.values())

# Matches to an item or field definition pertaining to a peripheral (e.g.,
# `crate::OSTM0`, `crate::OSTM1`) to be cfg-gated
PERIPHERAL_RE = [
    re.compile(r"^ *pub struct ([a-zA-Z0-9_]+)", re.MULTILINE),
    re.compile(r"^ *unsafe impl Send for ([a-zA-Z0-9_]+)", re.MULTILINE),
    re.compile(r"^ *impl ([a-zA-Z0-9_]+) \{", re.MULTILINE),
    re.compile(r"^ *impl Deref for ([a-zA-Z0-9_]+)", re.MULTILINE),
    re.compile(r"^ *pub ([a-zA-Z0-9_]+): \1", re.MULTILINE),
    re.compile(r"([a-zA-Z0-9_]+): \1 \{"),
]

# Matches to an item or field definition pertaining to a peripheral struct
# module (e.g., `crate::ostm0`) to be cfg-gated
MOD_RE = re.compile(r"pub mod ([a-zA-Z0-9_]+);")

PERIPHERAL_NAME_RE = re.compile(r".*?([0-9]*)")

with open(lib_path) as f:
    source = f.read()

def peripheral_to_cfg(peripheral: str) -> str:
    """
    Guess the Cargo feature that the given peripheral is dependent on.
    """
    match = PERIPHERAL_NAME_RE.fullmatch(peripheral)
    num_digits = len(match[1])
    for i in range(num_digits + 1):
        feature_name = peripheral[:len(peripheral) - i].lower()
        if feature_name in cfgs:
            return feature_name

    raise RuntimeError("coudln't guess the Cargo feature name for peripheral '%s'"
        % peripheral)

def cfg_meta(cfg):
    return '#[cfg(%s)] #[cfg_attr(docsrs, doc(cfg(%s)))]' % (cfg, cfg)

def subst_peripheral(match: List[str]) -> str:
    name = match[1]
    if name == 'Peripherals':
        return match[0]

    feature = peripheral_to_cfg(name)
    return cfg_meta('feature = "%s"' % feature) + match[0]

def subst_mod(match: List[str]) -> str:
    name = match[1]
    feature = mod_to_cfg[name]
    return cfg_meta('feature = "%s"' % feature) + match[0]

# Add `#[cfg(feature = ...)]`
source = MOD_RE.sub(subst_mod, source)

for re_compiled in PERIPHERAL_RE:
    old_source = source
    source = re_compiled.sub(subst_peripheral, source)
    if old_source == source:
        raise RuntimeError("'%s' didn't match anything" % re_compiled)

# List the Cargo features in the crate-level documentation
cfg_list = list(cfgs)
cfg_list.sort()

text = """
# Cargo Features

This crate defines the following Cargo features: %s

All features are disabled by default.
""" % (', '.join("`%s`" % s for s in cfg_list))

text_escaped = text.replace('\n', r'\n')
inserted_fragment = """
#![doc = "%s"]
#![cfg_attr(docsrs, feature(doc_cfg))]
""" % text_escaped

i = source.find('\n')
source = source[0:i] + inserted_fragment + source[i:]

with open(lib_path, 'w') as f:
    f.write(source)
