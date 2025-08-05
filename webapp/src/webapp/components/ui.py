import base64

import streamlit as st

from webapp.config.constants import LinkUrls
from webapp.resources.icons.icons import IconSvg


def favicon(icon_svg: IconSvg) -> str:
    """Load page favicon from svg."""
    icon: str = icon_svg.value
    encoded_icon: str = base64.b64encode(icon.encode("utf-8")).decode("utf-8")

    favicon_uri: str = f"data:image/svg+xml;base64,{encoded_icon}"
    return favicon_uri

def icon_link(icon_svg: IconSvg, link_url: LinkUrls) -> None:
    """Generate a github image link that expands on hover."""
    icon: str = icon_svg.value
    url: str = link_url.value

    st.markdown(
        # target="_blank" opens link in new tab
        # rel="noopener noreferrer" prevents tabnabbing
        f"""
        <a href="{url}"
            target="_blank"
            rel="noopener noreferrer"
            class="hover-enlarge">
            {icon}
        </a>
        """,
        unsafe_allow_html=True,
    )
