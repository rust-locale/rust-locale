import collections
from datetime import datetime, timezone
import json
import os.path

numberingSystems = dict()
metaZones = list()
timeZoneMap = list()

ZoneInfo = collections.namedtuple('ZoneInfo', ('timeZone', 'from_', 'to_', 'metaZone'))

def unixMinutes(text, default):
    if text is None:
        return default
    else:
        d = datetime.strptime(text, '%Y-%m-%d %H:%M').replace(tzinfo=timezone.utc)
        return str(int(d.timestamp() / 60))

ZoneInfo.get_from = lambda self: unixMinutes(self.from_, 'i32::MIN')
ZoneInfo.get_to = lambda self: unixMinutes(self.to_, 'i32::MAX')

def load(path):
    nsfile = json.load(open(os.path.join(path, 'numberingSystems.json'), encoding='utf-8'))
    numberingSystems.update(nsfile['supplemental']['numberingSystems'])

    mzfile = json.load(open(os.path.join(path, 'metaZones.json'), encoding='utf-8'))

    mzs = set()
    for z in mzfile['supplemental']['metaZones']['metazones']:
        mzs.add(z['mapZone']['_other'])
    metaZones[:] = sorted(mzs)

    def infos(mzi, prefix=''):
        for name, info in mzi.items():
            name = name.replace('-', '')
            if isinstance(info, list):
                yield prefix + name, info
            elif isinstance(info, dict):
                yield from infos(info, prefix + name + '__')
            else:
                raise TypeError('info is expected to be list or dict, not' + type(info))

    prev = ZoneInfo(timeZone=None, from_=None, to_=None, metaZone=None)
    for tz, info in sorted(infos(mzfile['supplemental']['metaZones']['metazoneInfo']['timezone'])):
        for i in info:
            uses = i['usesMetazone']
            curr = ZoneInfo(timeZone=tz, from_=uses.get('_from', None), to_=uses.get('_to', None), metaZone=uses['_mzone'])
            if curr.timeZone == prev.timeZone and curr.from_ != prev.to_: # There is a gap
                timeZoneMap.append(ZoneInfo(timeZone=tz, from_=prev.to_, to_=curr.from_, metaZone='Unknown'))
            elif curr.timeZone != prev.timeZone and prev.to_ != None: # Prev does not extend to future
                timeZoneMap.append(ZoneInfo(timeZone=prev.timeZone, from_=prev.to_, to_=None, metaZone='Unknown'))
            timeZoneMap.append(curr)
            prev = curr
    if prev.timeZone != None and prev.to_ != None: # Last does not extend to future
        timeZoneMap.append(ZoneInfo(timeZone=prev.timeZone, from_=prev.to_, to_=None, metaZone='Unknown'))
