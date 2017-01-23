import email

class SubtagRegistry:
    def __init__(self, data=None):
        if not data:
            import urllib.request
            data = urllib.request.urlopen(
                    'https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry')
            if data.status != 200:
                raise RuntimeError("Can't download language subtag registry, status={}".format(repr(data.status)))

        self.language = {}
        self.extlang = {}
        self.script = {}
        self.region = {}
        self.variant = {}
        self.grandfathered = {}
        self.redundant = {}

        with data:
            for chunk in data.read().split(b'%%'):
                #print('Chunk:', chunk.decode('utf-8'))
                msg = email.message_from_string(chunk.decode('utf-8').strip())
                #print('Msg:', msg.items())
                if not 'Type' in msg:
                    continue # Skip the first entry
                #print("Accepted!")
                type_ = msg['Type']
                key = msg['Subtag'] or msg['Tag']
                getattr(self, type_)[key] = msg
                #print('Msg:', msg.items())
