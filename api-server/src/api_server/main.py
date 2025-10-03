from typing import Dict

from fastapi import FastAPI

app = FastAPI()

@app.get("/")
def root() -> Dict[str, str]:
    """Root route."""
    return {"Hello": "World"}
