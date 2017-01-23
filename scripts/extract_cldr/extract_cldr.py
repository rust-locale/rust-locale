#!/usr/bin/python3

import argparse
import os
import os.path
import sys

from genshi.template import NewTextTemplate

from lib.locale import Locale
from lib.subtags import SubtagRegistry

default_template = os.path.join(os.path.dirname(__file__), '../../src/data/cldr/data.rs.genshi')

ap = argparse.ArgumentParser(
        description="Generate mod/data/cldr/data.rs from CLDR repository and other resources.",
        )
ap.add_argument("--iana-registry", type=argparse.FileType(mode='rb'),
        help="Path to downloaded copy of https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry, if available.")
ap.add_argument("--template", type=argparse.FileType(mode='r', encoding='utf-8'),
        default=default_template, help="Path to the Rust source template")
ap.add_argument("--output", type=argparse.FileType(mode='w', encoding='utf-8'),
        help="Path to the output Rust source")
ap.add_argument("cldr", help="Path to checkout of http://unicode.org/repos/cldr/, preferably tags/latest")

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


def cldrp(*components):
    return os.path.join(args.cldr, *components)

if not os.path.exists(cldrp('common/main/root.xml')):
    sys.stderr.write(
            "error: {cldr} does not contain checkout of CLDR repository\n".format(*args))
    sys.exit(1)

# Load subtag registry
subtag_registry = SubtagRegistry(args.iana_registry)

# Load all languages:
common_main = cldrp('common/main')
locale_map = dict(
        (str(l), l) for l in
            (Locale(common_main, fn) for fn in os.listdir(common_main) if fn.endswith('.xml')))

# Load supplemental:
common_supplemental = cldrp('common/supplemental')
Locale.load_supplemental(common_supplemental)


for l in locale_map.values():
    pi = l.parent_id()
    if pi is not None:
        p = locale_map[pi]
        l.set_parent(p)

tmpl = NewTextTemplate(args.template)
out = tmpl.generate(locales=(v for (k, v) in sorted(locale_map.items(), key=lambda x: x[0])))
args.output.write(out.render())

# TODO:
# - Add suppressed scripts to the search tree.
# - Add omitted script variants to the search tree.
# - Add the obsolete forms to the search tree.
# - Generate the code.
