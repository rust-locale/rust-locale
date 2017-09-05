#!/usr/bin/python3

import argparse
import os
import os.path
import sys

from genshi.template import NewTextTemplate

cwd = os.path.dirname(__file__)
default_template = os.path.join(cwd, '../../src/data/cldr/data.rs.genshi')
default_items_module = os.path.join(cwd, '../../src/data/mod.rs')

ap = argparse.ArgumentParser(
        description="Generate mod/data/cldr/data.rs from CLDR repository and other resources.",
        )
ap.add_argument("--items-module", type=argparse.FileType(mode='r', encoding='utf-8'),
        default=default_items_module, help="Path to the Rust module defining Items enum")
ap.add_argument("--template", type=argparse.FileType(mode='r', encoding='utf-8'),
        default=default_template, help="Path to the Rust source template")
ap.add_argument("--output", type=argparse.FileType(mode='w', encoding='utf-8'),
        help="Path to the output Rust source")

args = ap.parse_args()

if not args.output:
    if args.template.name == '<stdin>':
        args.output = sys.stdout
    elif args.template.name.endswith('.genshi'):
        args.output = open(args.template.name[0:-7], mode='w', encoding='utf-8')
        print("output:", args.output.name)
    else:
        sys.stderr.write(
                "error: {name} does not end in .genshi and output not specified\n".format(name=args.template.name))
        sys.exit(1)


def cldrp(section, *components):
    return os.path.join(cwd, 'node_modules/cldr-{}-modern/main'.format(section), *components)

# Load tag list
from lib import items
items.load_items(args.items_module)

# Load the other modules so they see the items module with items already created
from lib.locale import Locale
from lib.subtags import SubtagRegistry

# Load subtag registry
subtag_registry = SubtagRegistry(cwd)

# Load supplemental:
common_supplemental = os.path.join(cwd, 'node_modules/cldr-core/supplemental')
Locale.load_supplemental(common_supplemental)

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

tmpl = NewTextTemplate(args.template)
out = tmpl.generate(locales=(v for (k, v) in sorted(locale_map.items(), key=lambda x: x[0])))
args.output.write(out.render())

# TODO:
# - Add suppressed scripts to the search tree.
# - Add omitted script variants to the search tree.
# - Add the obsolete forms to the search tree.
# - Generate the code.
