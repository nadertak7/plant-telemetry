import streamlit as st

from webapp.config.constants import LinkUrls
from webapp.resources.icons.icons import IconSvg
from webapp.utils.icons import generate_favicon, generate_icon_link
from webapp.utils.load_styles import load_styles


def load_page_header() -> None:
    # Initialisation
    st.set_page_config(
        page_title="Plant Telemetry",
        page_icon=generate_favicon(icon_svg=IconSvg.BIRD_ICON),
        layout="wide"
    )
    load_styles()

    # Page contents
    with st.container():
        left_column, middle_column, right_column = st.columns([0.1, 0.7, 0.2])

        with left_column:
            generate_icon_link(
                icon_svg=IconSvg.GITHUB_LOGO_ICON,
                link_url=LinkUrls.GITHUB_REPO_URL
            )
            generate_icon_link(
                icon_svg=IconSvg.LINKEDIN_LOGO_ICON,
                link_url=LinkUrls.LINKEDIN_URL
            )

        with middle_column:
            st.header("Plant Telemetry", divider="green")

    st.write("\n")
    st.write("\n")
