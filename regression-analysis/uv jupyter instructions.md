# uv Jupyter Instructions

1. Create the kernel
   1. `uv add --dev ipykernel` in the `uv` environment.
   2. Create the kernel `uv run ipython kernel install --user --env VIRTUAL_ENV $(pwd)/.venv --name=regression-analysis`

Check this [documentation](https://docs.astral.sh/uv/guides/integration/jupyter/#using-jupyter-within-a-project)
