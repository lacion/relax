#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = []
# ///
"""Two shared checkouts observe direct and atomic source saves through inspect JSON."""
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time

runner = str(Path(sys.argv[1]).resolve())
fixture = Path(__file__).parent / "pkg"
with tempfile.TemporaryDirectory(prefix="live-shared-") as tmp:
    root = Path(tmp)
    env = dict(os.environ, RELAX_HOME=str(root / "home"), RELAX_SHARE="live-shared-example",
               RELAX_PANIC_PARK="0", TMPDIR=str(root))
    for name in ("a", "b"):
        pkg = root / name
        shutil.copytree(fixture, pkg)
        with (root / f"{name}.log").open("w") as log:
            process = subprocess.Popen([runner, "test", "--live", "live-shared-example", "--lib"],
                                       cwd=pkg, env=env, stdout=log, stderr=log, start_new_session=True)
            try:
                def query(*args):
                    result = subprocess.run([runner, "inspect", *map(str, args), "--json"],
                                            env=env, capture_output=True, text=True, timeout=90, check=True)
                    return [json.loads(line) for line in result.stdout.splitlines()]
                pid = None
                deadline = time.monotonic() + 300
                while time.monotonic() < deadline:
                    runs = [r for r in query("list") if r.get("app") == "live-shared-example"]
                    if runs:
                        assert len(runs) == 1, runs
                        pid = runs[0]["pid"]
                        break
                    assert process.poll() is None, "live session exited before discovery"
                    time.sleep(0.05)
                assert pid is not None, "no live inspect endpoint"

                def verdict(since):
                    rows = query(pid, "watch", "on=testverdict", f"since={since}", "timeout_s=30")
                    event = next((r for r in rows if r.get("event") == "testverdict"), None)
                    assert event is not None, f"shared workspace ignored the source save: {rows}"
                    return event

                initial = verdict(0)
                assert (initial["passed"], initial["failed"]) == (2, 0), initial
                source = pkg / "src/lib.rs"
                base = source.read_text()
                changed = base.replace("{ 1 }", "{ 2 }")
                if name == "a":
                    source.write_text(changed)
                else:
                    replacement = source.with_suffix(".new")
                    replacement.write_text(changed)
                    replacement.replace(source)
                failed = verdict(initial["seq"])
                assert failed["group"] > initial["group"] and not failed["restarted"], failed
                assert failed["failures"] == ["value_is_one"], failed
                source.write_text(base)
                recovered = verdict(failed["seq"])
                assert (recovered["passed"], recovered["failed"]) == (2, 0), recovered
                assert not (pkg / ".makepad-run-edits").exists(), "a shared checkout needs no ledger"
                print(f"live_shared_workspace: PASS ({name}, save, failure, recovery)", flush=True)
            except BaseException:
                print((root / f"{name}.log").read_text(), file=sys.stderr)
                raise
            finally:
                try:
                    os.killpg(process.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                try:
                    process.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    process.wait()
