import json
from os.path import join

class SubtagRegistry:
    def __init__(self, cwd):
        self.language = {}
        self.extlang = {}
        self.script = {}
        self.region = {}
        self.variant = {}
        self.grandfathered = {}
        self.redundant = {}

        for msg in json.load(open(join(cwd, 'node_modules/language-subtag-registry/data/json/registry.json'), encoding='utf-8')):
            type_ = msg['Type']
            key = getattr(msg, 'Subtag', None) or getattr(msg, 'Tag', None)
            getattr(self, type_)[key] = msg
