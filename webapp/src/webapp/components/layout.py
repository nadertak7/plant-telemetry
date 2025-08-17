import streamlit as st

from webapp.components.ui import favicon, icon_link
from webapp.config.constants import LinkUrls
from webapp.resources.images.icons import IconSvg
from webapp.utils.style_management import load_styles


def page_header() -> None:
    """Create the page header with title, external links and divider."""
    # Initialisation
    st.set_page_config(
        page_title="Plant Telemetry",
        page_icon=favicon(icon_svg=IconSvg.BIRD_ICON),
        layout="wide"
    )
    load_styles()

    # Page contents
    with st.container():
        left_column, middle_column, right_column = st.columns([0.05, 0.05, 0.9])

        with left_column:
            icon_link(
                icon_svg=IconSvg.GITHUB_LOGO_ICON,
                link_url=LinkUrls.GITHUB_REPO_URL
            )
        with middle_column:
            icon_link(
                icon_svg=IconSvg.LINKEDIN_LOGO_ICON,
                link_url=LinkUrls.LINKEDIN_URL
            )

    with st.container():
        left_column, right_column = st.columns([0.6, 0.4])
        with left_column:
            st.header("Plant Telemetry", divider="green")

    st.write("\n")
    st.write("\n")
