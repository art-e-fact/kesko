from typing import Union, Protocol
import base64

import numpy as np

from ..color import Rgba, Color
from ..pykesko import Model


class Command(Protocol):
    def to_json(self) -> Union[dict, str]:
        ...


class CheckAlive:
    def to_json(self):
        return "IsAlive"


class Spawn:
    def __init__(
        self,
        model: Model,
        position: list[float],
        color: Union[Rgba, Color],
        scale: list[float] = [1.0, 1.0, 1.0],
        rotation: list[float] = [0.0, 0.0, 0.0],
    ):
        self.model = model
        self.position = position
        self.color = color
        self.scale = scale
        self.rotation = rotation

    def to_json(self):
        return {
            "SpawnModel": {
                "model": self.model.name,
                "position": self.position,
                "color": self.color.to_json(),
                "scale": self.scale,
                "rotation": self.rotation,
            }
        }


class SpawnAsset:
    def __init__(self, asset_path: str, position: list[float]):
        self.asset_path = asset_path
        self.position = position

    def to_json(self):
        return {
            "SpawnAsset": {
                "asset_path": self.asset_path,
                "position": self.position,
            }
        }


class SpawnUrdf:
    def __init__(
        self, urdf_path: str, package_map: dict[str, str], position: list[float]
    ):
        self.urdf_path = urdf_path
        self.package_map = package_map
        self.position = position

    def to_json(self):
        return {
            "SpawnUrdf": {
                "urdf_path": self.urdf_path,
                "package_map": self.package_map,
                "position": self.position,
            }
        }


class Despawn:
    def __init__(self, id: int):
        self.id = id

    def to_json(self):
        return {"Despawn": {"id": self.id}}


class DespawnAll:
    def to_json(self):
        return "DespawnAll"


class Shutdown:
    def to_json(self):
        return "Close"


class GetState:
    def to_json(self):
        return "GetState"


class ApplyControl:
    def __init__(
        self,
        body_id: int,
        values: Union[dict[np.uint64, float], np.ndarray],
    ):
        self.body_id = body_id
        self.values = values

    def to_json(self):
        return {"ApplyMotorCommand": {"id": self.body_id, "command": self.values}}


class PausePhysics:
    def to_json(self):
        return "PausePhysics"


class RunPhysics:
    def to_json(self):
        return "RunPhysics"

class PublishFlatBuffers:
    def __init__(self, data: bytearray):
        self.data = data

    def to_json(self):
        data = base64.b64encode(self.data).decode('utf-8')
        return {"PublishFlatBuffers": {"data": data}}
