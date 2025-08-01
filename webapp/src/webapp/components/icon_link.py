import streamlit as st

from webapp.config.constants import LinkUrls
from webapp.public.icons.icons import IconSvg


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
