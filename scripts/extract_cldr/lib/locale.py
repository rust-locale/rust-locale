import json
import os.path
import re
import sys

from . import items
from . import supplemental

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

calendar = Enum(
        'Calendar::',
        'Gregorian',
        )

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

length = Enum(
        'Length::',
        'Short',
        'Medium',
        'Long',
        'Full',
        )

dayPeriodType = Enum(
        'DayPeriodType::',
        'AM',
        'PM',
        'Midnight',
        'Noon',
        )

class Locale:
    local_file_re = re.compile(r"([a-z]{2,3})(-[A-Z][a-z]{3})?(-(?:[A-Z]{2}|[0-9]{3}))?(-[A-Za-z0-9]{5,8})?$")

    def __init__(self, pathfn, lcid):
        m = self.local_file_re.match(lcid)
        if m:
            self._lang = m.group(1)
            self._script = m.group(2)[1:] if m.group(2) else None
            self._region = m.group(3)[1:] if m.group(3) else None
            self._variant = m.group(4)[1:].lower() if m.group(4) else None
        elif lcid == 'root':
            self._lang = ''
            self._script = None
            self._region = None
            self._variant = None
        else:
            raise ValueError("File name {} can't be parsed as locale.".format(lcid))
        self._symbol = lcid.upper().replace('-', '_')

        def loader(*args):
            return json.load(open(pathfn(*args), encoding='utf-8'))

        self._numbers = loader('numbers', lcid, 'numbers.json')['main'][lcid]['numbers']
        self._ca_gregorian = loader('dates', lcid, 'ca-gregorian.json')['main'][lcid]['dates']['calendars']['gregorian']
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
        return self._symbol

    def parent_id(self):
        return '-'.join(self.tag_list()[0:-1]) if self._lang else None

    def set_parent(self, parent):
        par_tl = parent.tag_list()
        self_tl = self.tag_list()
        # TODO: Make sure skipped intermediate tags are properly handled
        # assert par_tl == self_tl[0:-1]
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

    def _numSysId(self):
        try:
            return self._numbers['defaultNumberingSystem']
        except KeyError:
            raise ValueError("No number system for {}".format(self.id()))

    def _common_exponent(self, numSymbols, numSystem):
        symbol = numSymbols['superscriptingExponent']
        digits = numSystem['_digits']
        return '{}{}{}{}'.format(
                symbol,
                digits[1],
                digits[0],
                '' if digits[0] == '0' else '^')

    def _date_items(self, data, item, srng, trng=None, ignore_missing=False):
        rng = list(zip(trng, srng)) if trng else list(enumerate(srng, 0))
        for (c, w, target) in (
                ('format', 'abbreviated', width.FormatAbbr),
                ('format', 'narrow', width.FormatNarrow),
                ('format', 'short', width.FormatShort),
                ('format', 'wide', width.FormatWide),
                ('stand-alone', 'abbreviated', width.StandAloneAbbr),
                ('stand-alone', 'narrow', width.StandAloneNarrow),
                ('stand-alone', 'short', width.StandAloneShort),
                ('stand-alone', 'wide', width.StandAloneWide),
            ):
            if w in data[c]:
                leaf = data[c][w]
                for n, d in rng:
                    self._items[item(target, calendar.Gregorian, n)] = \
                        leaf.get(str(d), None) if ignore_missing else leaf[str(d)]

    def _date_formats(self, data, item):
        self._items[item(length.Full, calendar.Gregorian)] = data['full']
        self._items[item(length.Long, calendar.Gregorian)] = data['long']
        self._items[item(length.Medium, calendar.Gregorian)] = data['medium']
        self._items[item(length.Short, calendar.Gregorian)] = data['short']

    def _init_items(self):
        if self._parent:
            self._items = dict(self._parent.items())
        else:
            self._items = dict()

        # Numeric
        numSysId = self._numSysId()
        numSystem = supplemental.numberingSystems[numSysId]
        numSymbols = self._numbers['symbols-numberSystem-' + numSysId]
        self._items[items.DecimalDigits] = numSystem['_digits']

        self._items[items.DecimalSeparator] = numSymbols['decimal']
        self._items[items.GroupSeparator] = numSymbols['group']
        self._items[items.PlusSign] = numSymbols['plusSign']
        self._items[items.MinusSign] = numSymbols['minusSign']
        self._items[items.PercentSign] = numSymbols['percentSign']
        self._items[items.PerMilleSign] = numSymbols['perMille']
        self._items[items.EngineeringExponent] = numSymbols['exponential']
        self._items[items.CommonExponent] = self._common_exponent(numSymbols, numSystem)
        self._items[items.InfinitySymbol] = numSymbols['infinity']
        self._items[items.NotANumberSymbol] = numSymbols['nan']

        decimalFmtStd = self._numbers['decimalFormats-numberSystem-' + numSysId]['standard']
        numPattern = DecimalPattern(decimalFmtStd)
        assert numPattern.pos_pre == '' and numPattern.pos_post == '' and numPattern.neg_pre is None
        self._items[items.Grouping] = numPattern.groups
        self._items[items.FractionalGrouping] = numPattern.fract_groups
        self._items[items.MinGroupingDigits] = self._numbers['minimumGroupingDigits']
        self._items[items.MinIntegralDigits] = numPattern.min_int

        # Date&Time
        self._date_items(
            self._ca_gregorian['months'], items.Month, range(1, 13))
        self._date_items(self._ca_gregorian['days'], items.Day,
                         ('sun', 'mon', 'tue', 'wed', 'thu', 'fri', 'sat'))
        self._date_items(
            self._ca_gregorian['quarters'], items.Quarter, range(1, 5))
        self._date_items(self._ca_gregorian['dayPeriods'], items.DayPeriod,
                         ('am', 'pm', 'midnight', 'noon'),
                         (dayPeriodType.AM, dayPeriodType.PM,
                          dayPeriodType.Midnight, dayPeriodType.Noon),
                         ignore_missing=True)

        for e in (0, 1):
            self._items[items.EraWide(calendar.Gregorian, e)] = \
                self._ca_gregorian['eras']['eraNames'][str(e)]
            self._items[items.EraAbbr(calendar.Gregorian, e)] = \
                self._ca_gregorian['eras']['eraAbbr'][str(e)]
            self._items[items.EraNarrow(calendar.Gregorian, e)] = \
                self._ca_gregorian['eras']['eraNarrow'][str(e)]

        self._date_formats(self._ca_gregorian['dateFormats'], items.DateFormat)
        self._date_formats(self._ca_gregorian['timeFormats'], items.TimeFormat)
        self._date_formats(
            self._ca_gregorian['dateTimeFormats'], items.DateTimeFormat)

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
