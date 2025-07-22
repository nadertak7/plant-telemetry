import base64
import os

import streamlit as st

CSS_FILE_PATH = "src/webapp/styles/main.css"

def generate_github_link() -> None:
    """Generate a github image link that expands on hover."""
    if os.path.exists(CSS_FILE_PATH):
        with open(CSS_FILE_PATH) as f:
            st.markdown(f"<style>{f.read()}</style>", unsafe_allow_html=True)
    else:
        st.warning(f"CSS file not found at {CSS_FILE_PATH}. Custom styles will not apply.")

    st.markdown("""
        <a href="https://github.com/nadertak7/plant-telemetry"
            target="_blank"
            rel="noopener noreferrer"
            class="hover-enlarge">
            <img src="data:image/png;base64,{}" width="40">
        </a>
    """.format(
        base64.b64encode(open("src/webapp/resources/images/github.png", "rb").read()).decode()
    ),
    unsafe_allow_html=True
    )
