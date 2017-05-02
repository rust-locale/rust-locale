import os.path
import re
import sys

from lxml.etree import ElementTree, parse

from . import items

def text_of(elem):
    return elem.text if elem is not None else None

def attr_of(elem, attr):
    return elem.get(attr) if elem is not None else None

def follow_alias(elem):
    if elem is not None:
        a = elem.find('alias')
        if a is not None:
            return follow_alias(elem.find(a.get('path')))
    return elem

def escape(c):
    if c == "'":
        return "\\'"
    elif c == '"':
        return '\\"'
    elif c == '\n':
        return '\\n'
    elif c == '\r':
        return '\\r'
    elif c == '\t':
        return '\\t'
    elif c == '\\':
        return '\\\\'
    elif c == '\0':
        return '\\0'
    elif ord(c) < 32:
        return '\\x{:02x}'.format(ord(c))
    else:
        return c

num_pattern_re = re.compile(r"(.*?)([#0@,.]+)(.*?)(;(.*?)([#0@,.]+)(.*?))?")

class DecimalPattern:
    def __init__(self, pat):
        m = num_pattern_re.match(pat)
        if m:
            self.pos_pre = m.group(1)
            core = m.group(2)
            self.pos_post = m.group(3)
            if m.group(4):
                self.neg_pre = m.group(5)
                # neg core is ignored according to the rules!
                self.neg_post = m.group(7)
            else:
                self.neg_pre = None
                self.neg_post = None
            if '.' in core:
                pos, neg = core.split('.', 2)
            else:
                pos, neg = core, ''
            groups = pos.split(',')[1:]
            fract_groups = neg.split(',')[:-1]
            self.groups = ';'.join((str(len(g)) for g in groups))
            self.fract_groups = ';'.join((str(len(g)) for g in fract_groups))
            self.min_int = str(sum(1 for c in pos if c == '0'))
        else:
            raise ValueError("Unrecognized pattern: ‘{}’".format(pat))

class Enum:
    class Const(str):
        def __repr__(self):
            return str(self)

    def __init__(self, prefix, *consts):
        for c in consts:
            setattr(self, c, self.Const(prefix + c))

width = Enum(
        'Width::',
        'FormatAbbr',
        'FormatWide',
        'FormatNarrow',
        'FormatShort',
        'StandAloneAbbr',
        'StandAloneWide',
        'StandAloneNarrow',
        'StandAloneShort',
        )

calendar = Enum(
        'Calendar::',
        'Gregorian'
        )

class Locale:
    local_file_re = re.compile(r"([a-z]{2,3})(_[A-Z][a-z]{3})?(_(?:[A-Z]{2}|[0-9]{3}))?(_[A-Za-z0-9]{5,8})?\.xml")

    @classmethod
    def load_supplemental(class_, path):
        class_.numberingSystems = parse(os.path.join(path, 'numberingSystems.xml'))

    def __init__(self, path, fn):
        m = self.local_file_re.match(fn)
        if m:
            self._lang = m.group(1)
            self._script = m.group(2)[1:] if m.group(2) else None
            self._region = m.group(3)[1:] if m.group(3) else None
            self._variant = m.group(4)[1:].lower() if m.group(4) else None
        elif fn == "root.xml":
            self._lang = ''
            self._script = None
            self._region = None
            self._variant = None
        else:
            raise ValueError("File name {} can't be parsed as locale.".format(fn))

        self._cldr_id = fn[:-4]
        self._xml = parse(os.path.join(path, fn))
        self._children = dict()
        self._parent = None
        self._items = None
        self._index = None
        self._data = None

    def tag_list(self):
        return [tag for tag in (self._lang, self._script, self._region, self._variant) if tag]

    def id(self, sep='-'):
        return sep.join(self.tag_list())

    def symbol(self):
        return self._cldr_id.upper()

    def parent_id(self):
        return '-'.join(self.tag_list()[0:-1]) if self._lang else None

    def set_parent(self, parent):
        par_tl = parent.tag_list()
        self_tl = self.tag_list()
        assert par_tl == self_tl[0:-1]
        assert self._parent is None
        assert self_tl[-1] not in parent._children
        parent._children[self_tl[-1]] = self
        self._parent = parent

    def parents(self):
        return (self._parent.symbol(),) if self._parent is not None else ()

    def children(self):
        return ((k, v.symbol()) for (k, v) in sorted(self._children.items(), key=lambda x: x[0]))

    def _find(self, path):
        return self._xml.find(path)

    def _get(self, path):
        return text_of(self._find(path))

    def _numSysId(self):
        numSysId = self._get('numbers/defaultNumberingSystem')
        if numSysId:
            return numSysId
        if self._parent:
            return self._parent._numSysId()
        raise ValueError("No number system for {}".format(self.id()))

    def _common_exponent(self, numSymbols, numSystem):
        symbol = text_of(numSymbols.find('superscriptingExponent'))
        digits = numSystem.get('digits')
        if symbol is None:
            return None
        if digits is None:
            print("warning: superscripting exponent for {} is {}, but digits is None".format(self.id(), symbol))
            return None
        return '{}{}{}{}'.format(
                symbol,
                digits[1],
                digits[0],
                '' if digits[0] == '0' else '^')

    def _do_month(self, c, w, target, fallback):
        mw = follow_alias(
                self._find('dates/calendars/calendar[@type="gregorian"]/'
                    + 'months/monthContext[@type="' + c + '"]/'
                    + 'monthWidth[@type="' + w + '"]'))
        if mw is not None:
            for n in range(0, 12):
                path = 'month[@type="{}"]'.format(n + 1)
                self._items[items.Month(target, calendar.Gregorian, n)] = text_of(mw.find(path))
        elif fallback:
            for n in range(0, 12):
                self._items[items.Month(target, calendar.Gregorian, n)] = \
                        self._items.get(items.Month(fallback, calendar.Gregorian, n), None)

    def _do_day(self, c, w, target, fallback):
        dw = follow_alias(
                self._find('dates/calendars/calendar[@type="gregorian"]/'
                    + 'days/dayContext[@type="' + c + '"]/'
                    + 'dayWidth[@type="' + w + '"]'))
        if dw is not None:
            for n, d in enumerate(('sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'), 0):
                path = 'day[@type="{}"]'.format(d)
                self._items[items.Day(target, calendar.Gregorian, n)] = text_of(dw.find(path))
        elif fallback:
            for n in range(0, 7):
                self._items[items.Day(target, calendar.Gregorian, n)] = \
                        self._items.get(items.Day(fallback, calendar.Gregorian, n), None)

    def _init_items(self):
        if self._parent:
            self._items = dict(self._parent.items())
        else:
            self._items = dict()

        # Numeric
        numSysId = self._numSysId()
        numSystem = self.numberingSystems.find(
            'numberingSystems/numberingSystem[@id="{}"]'.format(numSysId))
        numSymbols = self._find('numbers/symbols[@numberSystem="{}"]'.format(numSysId))
        self._items[items.DecimalDigits] = numSystem.get('digits')
        if numSymbols is not None:
            self._items[items.DecimalSeparator] = text_of(numSymbols.find('decimal'))
            self._items[items.GroupSeparator] = text_of(numSymbols.find('group'))
            self._items[items.PlusSign] = text_of(numSymbols.find('plusSign'))
            self._items[items.MinusSign] = text_of(numSymbols.find('minusSign'))
            self._items[items.PercentSign] = text_of(numSymbols.find('percentSign'))
            self._items[items.PerMilleSign] = text_of(numSymbols.find('perMille'))
            self._items[items.EngineeringExponent] = text_of(numSymbols.find('exponential'))
            self._items[items.CommonExponent] = self._common_exponent(numSymbols, numSystem)
            self._items[items.InfinitySymbol] = text_of(numSymbols.find('infinity'))
            self._items[items.NotANumberSymbol] = text_of(numSymbols.find('nan'))
        decimalFmtLen = self._find(
                'numbers/decimalFormats[@numberSystem="{}"]/decimalFormatLength'.format(numSysId))
        if decimalFmtLen is not None and decimalFmtLen.get('type') is None:
            numPattern = DecimalPattern(text_of(decimalFmtLen.find('decimalFormat/pattern')))
            assert numPattern.pos_pre == '' and numPattern.pos_post == '' and numPattern.neg_pre is None
            self._items[items.Grouping] = numPattern.groups
            self._items[items.FractionalGrouping] = numPattern.fract_groups
            self._items[items.MinGroupingDigits] = self._get('numbers/minimumGroupingDigits')
            self._items[items.MinIntegralDigits] = numPattern.min_int

        # Date&Time
        self._do_month('format', 'abbreviated', width.FormatAbbr, None)
        self._do_month('format', 'wide', width.FormatWide, width.FormatAbbr)
        self._do_month('format', 'narrow', width.FormatNarrow, width.FormatAbbr)
        self._do_month('format', 'short', width.FormatShort, width.FormatAbbr)
        self._do_month('stand-alone', 'abbreviated', width.StandAloneAbbr, width.FormatAbbr)
        self._do_month('stand-alone', 'wide', width.StandAloneWide, width.FormatWide)
        self._do_month('stand-alone', 'narrow', width.StandAloneNarrow, width.FormatNarrow)
        self._do_month('stand-alone', 'short', width.StandAloneShort, width.FormatShort)
        self._do_day('format', 'abbreviated', width.FormatAbbr, None)
        self._do_day('format', 'wide', width.FormatWide, width.FormatAbbr)
        self._do_day('format', 'narrow', width.FormatNarrow, width.FormatAbbr)
        self._do_day('format', 'short', width.FormatShort, width.FormatAbbr)
        self._do_day('stand-alone', 'abbreviated', width.StandAloneAbbr, width.FormatAbbr)
        self._do_day('stand-alone', 'wide', width.StandAloneWide, width.FormatWide)
        self._do_day('stand-alone', 'narrow', width.StandAloneNarrow, width.FormatNarrow)
        self._do_day('stand-alone', 'short', width.StandAloneShort, width.FormatShort)

        # TODO: Date&Time Patterns
        # TODO: ! find whether they are just in generic or elsewhere

        # TODO: Messages (Plurals; may be generated differently)
        # TODO: Monetary
        # TODO: Collate
        # TODO: Units (?)

    def items(self):
        if self._items is None:
            self._init_items()
        return self._items

    def __getitem__(self, item):
        if self._items is None:
            self._init_items()
        return self._items.__getitem__(item)

    def __setitem__(self, item, value):
        if self._items is None:
            self._init_items()
        return self._items.__setitem__(item, value)

    def __delitem__(self, item):
        if self._items is None:
            self._init_items()
        return self._items.__delitem__(item)

    def _init_data(self):
        if self._parent:
            parent_get = self._parent.__getitem__
        else:
            parent_get = lambda k: None

        self._index = []
        data = ''
        i = 0
        for k in sorted(self.items().keys()):
            v = self._items[k]
            if v is not None and v != parent_get(k):
                i += len(v.encode('utf-8'))
                self._index += (k, i),
                data += v
        self._data = ''.join(map(escape, data))

    def index(self):
        if self._data is None:
            self._init_data()
        return self._index

    def data(self):
        if self._data is None:
            self._init_data()
        return self._data

    def __str__(self):
        return self.id('-')

    def __repr__(self):
        return 'Locale({1}, lang=‘{0._lang}’, script=‘{0._script}’, region=‘{0._region}’, variant=‘{0._variant}’)'.format(self, str(self))
