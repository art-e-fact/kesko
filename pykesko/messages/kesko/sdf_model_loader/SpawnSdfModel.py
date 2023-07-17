# automatically generated by the FlatBuffers compiler, do not modify

# namespace: sdf_model_loader

import flatbuffers
from flatbuffers.compat import import_numpy
np = import_numpy()

class SpawnSdfModel(object):
    __slots__ = ['_tab']

    @classmethod
    def GetRootAs(cls, buf, offset=0):
        n = flatbuffers.encode.Get(flatbuffers.packer.uoffset, buf, offset)
        x = SpawnSdfModel()
        x.Init(buf, n + offset)
        return x

    @classmethod
    def GetRootAsSpawnSdfModel(cls, buf, offset=0):
        """This method is deprecated. Please switch to GetRootAs."""
        return cls.GetRootAs(buf, offset)
    @classmethod
    def SpawnSdfModelBufferHasIdentifier(cls, buf, offset, size_prefixed=False):
        return flatbuffers.util.BufferHasIdentifier(buf, offset, b"\x53\x4C\x53\x50", size_prefixed=size_prefixed)

    # SpawnSdfModel
    def Init(self, buf, pos):
        self._tab = flatbuffers.table.Table(buf, pos)

    # SpawnSdfModel
    def SdfPath(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(4))
        if o != 0:
            return self._tab.String(o + self._tab.Pos)
        return None

    # SpawnSdfModel
    def Transform(self):
        o = flatbuffers.number_types.UOffsetTFlags.py_type(self._tab.Offset(6))
        if o != 0:
            x = o + self._tab.Pos
            from kesko.sdf_model_loader.Transform import Transform
            obj = Transform()
            obj.Init(self._tab.Bytes, x)
            return obj
        return None

def SpawnSdfModelStart(builder):
    builder.StartObject(2)

def Start(builder):
    SpawnSdfModelStart(builder)

def SpawnSdfModelAddSdfPath(builder, sdfPath):
    builder.PrependUOffsetTRelativeSlot(0, flatbuffers.number_types.UOffsetTFlags.py_type(sdfPath), 0)

def AddSdfPath(builder, sdfPath):
    SpawnSdfModelAddSdfPath(builder, sdfPath)

def SpawnSdfModelAddTransform(builder, transform):
    builder.PrependStructSlot(1, flatbuffers.number_types.UOffsetTFlags.py_type(transform), 0)

def AddTransform(builder, transform):
    SpawnSdfModelAddTransform(builder, transform)

def SpawnSdfModelEnd(builder):
    return builder.EndObject()

def End(builder):
    return SpawnSdfModelEnd(builder)