Marmot Faculty Timetabling Chat
===============================

This directory contains a self-contained faculty chat application. A FastAPI server
coaches faculty through timetabling preferences with an OpenRouter model, validates model
tool calls against an installed semester snapshot and a live tentative-assignment
spreadsheet, and automatically maintains the latest complete working draft for each faculty
member. Each Python artifact embeds the typed submission that produced it. The web client
uses assistant-ui and is exported as static files for the Python server to host.

Development setup
-----------------

1.  Put `OPENROUTER_API_KEY` in `~/.keys`, or copy `.env.example` to `.env` and set it
    there. The default model is `stealth/ox-alpha`. Every request asks OpenRouter to choose
    the highest-throughput provider. Set `OPENROUTER_MODEL=deepseek/deepseek-v4-flash-0731`
    to switch back to DeepSeek without changing application code.
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
`runtime/sessions/`; the current faculty snippets are in `runtime/preferences/`. Assignment
tools download the Spring 2027 workbook on every call, so collaborative edits are visible
without rebuilding or restarting the app. Override its URL with `CURRENT_ASSIGNMENTS_URL`.
Automatic saves compare both the workbook revision and the prior saved-artifact revision;
validation errors or stale conversation branches leave the previous draft untouched.

To download the SharePoint workbook, start with the configured sharing URL and follow its
complete redirect chain in one stateful HTTP session. Copy each authentication token or
cookie issued by a response into the next redirected request. Requesting a redirected URL
without that authentication state can misleadingly return `403` or a Microsoft sign-in
page even though the sharing URL works.

Installing another semester
----------------------------

The installer is deliberately separate from the deployed server. It reads a Marmot data
tree once and produces the JSON snapshot shipped with the app. Directory names and the
explicit `--term` arguments are authoritative; database term metadata is not.

```console
python3 scripts/install_semester.py \
    --database ../data/timetable.db \
    --current-assignments 'https://dixiestate-my.sharepoint.com/:x:/g/personal/d00003177_utahtech_edu/IQBwn5uigXgnSbURqT5f7uSXATuBEraNsUaHm5xpQ8uTJPQ?rtime=KvRjnMcC30g&download=1' \
    --previous-faculty ../fall2026/data/computingfaculty.py \
    --older-faculty ../spring2026/data/computingfaculty.py \
    --term 'Spring 2027' \
    --previous-term 'Fall 2026' \
    --older-term 'Spring 2026' \
    --output data/spring-2027.json
```

The installer records both historical terms in newest-first order, including their section
setups for course-specific inference. It also installs catalog credit ranges, course
scheduling policy, and exceptional contact-minute requirements. It does not accept or read
a Spring 2027 faculty Python source. Current assignments come only from the workbook at
tool-execution time; the database contributes installed curriculum, room, and time
vocabulary. After installation, the wider Marmot repository is not used at runtime.
