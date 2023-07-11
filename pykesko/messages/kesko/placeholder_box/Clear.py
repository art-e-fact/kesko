# automatically generated by the FlatBuffers compiler, do not modify

# namespace: placeholder_box

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class Clear(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = Clear()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsClear(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def ClearBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x50\x42\x43\x4C", size_prefixed=size_prefixed)

    # Clear
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

def ClearStart(builder):
    builder.StartObject(0)

def Start(builder):
    ClearStart(builder)

def ClearEnd(builder):
    return builder.EndObject()

def End(builder):
    return ClearEnd(builder)