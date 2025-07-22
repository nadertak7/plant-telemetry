import os

import streamlit as st

from webapp.config.constants import CSS_FILE_PATH


def load_styles() -> None:
    if os.path.exists(CSS_FILE_PATH):
        with open(CSS_FILE_PATH) as styles_file:
            st.markdown(
                f"<style>{styles_file.read()}</style>",
                unsafe_allow_html=True
            )
    else:
        st.toast(f"CSS file not found at: {CSS_FILE_PATH}. Custom styles may not apply.")
