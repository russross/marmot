Marmot Faculty Timetabling Chat
===============================

This directory contains a self-contained faculty chat application. A FastAPI server
coaches faculty through timetabling preferences with an OpenRouter model, validates model
tool calls against an installed semester snapshot, and writes the latest complete Python
snippet for each faculty member. The web client uses assistant-ui and is exported as static
files for the Python server to host.

Development setup
-----------------

1.  Put `OPENROUTER_API_KEY` in `~/.keys`, or copy `.env.example` to `.env` and set it
    there. The default model is `deepseek/deepseek-v4-flash-0731`. Every request asks
    OpenRouter to choose the highest-throughput provider among `int8`, `fp8`, `fp16`,
    and `bf16` endpoints.
2.  Install Python dependencies and run checks:

    ```console
    uv sync
    uv run ruff check .
    uv run ty check
    uv run pytest
    ```

3.  Build the client:

    ```console
    cd frontend
    npm install
    npm run lint
    npm run build
    ```

4.  Start the combined app from this directory:

    ```console
    uv run uvicorn timetable_chat.main:app --host 0.0.0.0 --port 8000
    ```

Runtime files are created below `runtime/`. Session events are append-only JSONL files in
`runtime/sessions/`; the current faculty snippets are in `runtime/preferences/`.

Installing another semester
----------------------------

The installer is deliberately separate from the deployed server. It reads a Marmot data
tree once and produces the JSON snapshot shipped with the app. Directory names and the
explicit `--term` arguments are authoritative; database term metadata is not.

```console
python3 scripts/install_semester.py \
    --database ../fall2026/data/timetable.db \
    --current-faculty ../fall2026/data/computingfaculty.py \
    --previous-faculty ../spring2026/data/computingfaculty.py \
    --older-faculty ../fall2025/data/computing.py \
    --term 'Fall 2026' \
    --previous-term 'Spring 2026' \
    --older-term 'Fall 2025' \
    --output data/fall-2026.json
```

The installer records both historical terms in newest-first order. Fall 2025 uses the
legacy preference API, so this test installation converts its small preference vocabulary
to current names. Production history beginning with Spring 2026 requires no conversion.
After installation, the wider Marmot repository is not used at runtime.
