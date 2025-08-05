import base64

from webapp.public.icons.icons import IconSvg


def favicon(icon_svg: IconSvg) -> str:
    """Load page favicon from svg."""
    icon: str = icon_svg.value
    encoded_icon: str = base64.b64encode(icon.encode("utf-8")).decode("utf-8")

    favicon_uri: str = f"data:image/svg+xml;base64,{encoded_icon}"
    return favicon_uri
