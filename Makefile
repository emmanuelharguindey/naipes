# naipes — atajos de desarrollo
#
# Uso: `make <target>` desde la raíz del repo.
# Asume que tienes ~/naipes-tools/ con build, twine, pytest instalados
# (ver BUILD.md sección 1). Si no, sustituye TOOLS abajo.

TOOLS := ~/naipes-tools/bin
PY_PORT := ports/python

.PHONY: help test build clean verify install-dev publish-test publish

help:
	@echo "naipes — targets disponibles:"
	@echo "  make test          Run the Python port test suite"
	@echo "  make build         Build wheel + sdist into $(PY_PORT)/dist/"
	@echo "  make clean         Remove build artefacts and caches"
	@echo "  make verify        Build, install in temp venv, run a quick game"
	@echo "  make install-dev   Install the Python port in editable mode (uses $(TOOLS))"
	@echo "  make publish-test  Upload to TestPyPI (sandbox)"
	@echo "  make publish       Upload to real PyPI — irrevocable!"
	@echo ""
	@echo "Read BUILD.md for details."

install-dev:
	$(TOOLS)/pip install -e $(PY_PORT) --quiet

test: install-dev
	cd $(PY_PORT) && $(TOOLS)/pytest tests/

build: clean
	cd $(PY_PORT) && $(TOOLS)/python -m build

clean:
	rm -rf $(PY_PORT)/dist $(PY_PORT)/build $(PY_PORT)/*.egg-info
	find . -type d -name __pycache__ -exec rm -rf {} + 2>/dev/null || true
	find . -type d -name .pytest_cache -exec rm -rf {} + 2>/dev/null || true

verify: build
	@echo ""
	@echo "==> Installing built wheel in throwaway venv..."
	python3 -m venv /tmp/naipes-verify
	/tmp/naipes-verify/bin/pip install $(PY_PORT)/dist/*.whl --quiet
	@echo ""
	@echo "==> naipes --version:"
	/tmp/naipes-verify/bin/naipes --version
	@echo ""
	@echo "==> Quick scripted game (seed 42, expect RESULT human=40 ai=80 outcome=loss):"
	@printf "1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n1\n" | \
		/tmp/naipes-verify/bin/naipes play brisca --seed 42 --quiet | tail -1
	@rm -rf /tmp/naipes-verify

publish-test: build
	@echo "==> Uploading to TestPyPI..."
	$(TOOLS)/twine upload --repository testpypi $(PY_PORT)/dist/*

publish: build
	@echo "==> Uploading to PyPI (irrevocable)..."
	@read -p "Are you sure you want to publish to real PyPI? [y/N] " confirm && [ "$$confirm" = "y" ]
	$(TOOLS)/twine upload $(PY_PORT)/dist/*
