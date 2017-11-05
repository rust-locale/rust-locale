from functools import partial, total_ordering
import os.path
import re
import sys

@total_ordering
class Item:
    __slots__ = ('_name', '_index', '_params')

    def __init__(self, name, index, nparam, *params):
        self._name = name
        self._index = index
        if nparam != len(params):
            raise TypeError('Item of type {} need {} arguments ({} given)'.format(name, nparam, len(params)))
        self._params = params

    def __eq__(self, right):
        return self._index == right._index and self._params == right._params

    def __hash__(self):
        return hash((self._index, self._params))

    def __lt__(self, right):
        return self._index < right._index or (self._index == right._index and self._params < right._params)

    def __str__(self):
        if self._params:
            return self._name + repr(self._params)
        else:
            return self._name

    __repr__ = __str__

START_RE = re.compile(r'^pub enum Item {')
END_RE = re.compile(r'^}')
LINE_RE = re.compile(r'\s*([A-Z][A-Za-z0-9]*)(\(.*\))?,')

def load_items(f):
    for l in f:
        if START_RE.match(l):
            break
    index = 0
    for l in f:
        if END_RE.match(l):
            break
        m = LINE_RE.match(l)
        if m:
            name = m.group(1)
            if m.group(2):
                nparam = len(m.group(2).split(','))
                globals()[name] = partial(Item, name, index, nparam)
            else:
                nparam = 0
                globals()[name] = Item(name, index, 0)
            #DEBUG
            print("item {}/{}".format(name, nparam))
            #GUBED
            index += 1
