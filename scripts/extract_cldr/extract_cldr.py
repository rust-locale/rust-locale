#!/usr/bin/python3

import argparse
import os
import os.path
import sys

from genshi.template import NewTextTemplate

cwd = os.path.dirname(__file__)

items_module_path = os.path.join(cwd, '../../src/data/mod.rs')

data_path = os.path.join(cwd, '../../src/data/cldr/data.rs')

supplemental_path = os.path.join(cwd, '../../src/supplemental.rs')

def generate(path, *args, **kwargs):
    with open(path + '.genshi', mode='r', encoding='utf-8') as template:
        tmpl = NewTextTemplate(template)
    out = tmpl.generate(*args, **kwargs)
    with open(path, mode='w', encoding='utf-8') as output:
        output.write(out.render())

def cldrp(section, *components):
    return os.path.join(cwd, 'node_modules/cldr-{}-modern/main'.format(section), *components)

# Load tag list
from lib import items
with open(items_module_path, mode='r', encoding='utf-8') as items_module:
    items.load_items(items_module)

# Load the other modules so they see the items module with items already created
from lib.locale import Locale
from lib.subtags import SubtagRegistry
from lib import supplemental

# Load subtag registry
subtag_registry = SubtagRegistry(cwd)

# Load supplemental:
common_supplemental = os.path.join(cwd, 'node_modules/cldr-core/supplemental')
supplemental.load(common_supplemental)

# Load all languages:
locale_map = dict(
        (str(l), l) for l in
            (Locale(cldrp, lang) for lang in os.listdir(cldrp('numbers'))))

print(list(sorted(locale_map.keys())))

for l in locale_map.values():
    pi = l.parent_id()
    while pi is not None:
        if pi in locale_map:
            p = locale_map[pi]
            l.set_parent(p)
            break
        else:
            pi = pi[:pi.rindex('-')]

# Generate supplemental data modules
generate(supplemental_path, metaZones=supplemental.metaZones, timeZoneMap=supplemental.timeZoneMap)
# Generate the main data module
generate(data_path, locales=(v for (k, v) in sorted(locale_map.items(), key=lambda x: x[0])))

# TODO:
# - Add suppressed scripts to the search tree.
# - Add omitted script variants to the search tree.
# - Add the obsolete forms to the search tree.
# - Generate the code.
