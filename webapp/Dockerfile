FROM ghcr.io/astral-sh/uv:0.8.4-python3.12-bookworm-slim

WORKDIR /app

ENV PYTHONPATH="/app:/app/src:${PYTHONPATH}" \
    UV_LINK_MODE=copy \
    UV_COMPILE_BYTECODE=1

# Dependency installation layer
  # Isolate dependency installation. Only reinstalled
  # if these files are changed
COPY uv.lock pyproject.toml ./
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-install-project --no-dev

# Project installation layer
COPY .streamlit/ .streamlit/
COPY src/ src/
RUN --mount=type=cache,target=/root/.cache/uv \
    uv sync --frozen --no-dev

# Activate project venv
# See: https://docs.astral.sh/uv/guides/integration/docker/#using-the-environment
ENV PATH="/app/.venv/bin:$PATH"

CMD ["streamlit", "run", "src/webapp/Homepage.py", "--server.port=8501"]

