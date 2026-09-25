from tapo.to_dict_ext import ToDictExt

class Snapshot(ToDictExt):
    """A still snapshot captured from a camera."""

    data: bytes
    """The raw bytes of the snapshot."""

    content_type: str
    """MIME content type of the snapshot, e.g. `"image/jpeg"`."""
