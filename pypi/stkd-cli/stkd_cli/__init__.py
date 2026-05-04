"""Stack CLI - Python wrapper for the gt binary."""

import json
import os
import subprocess
import sys
import shutil


def _find_binary():
    """Find the gt binary in PATH or known locations."""
    binary = shutil.which("gt")
    if binary:
        return binary

    # Check if it was installed by this package
    scripts_dir = os.path.dirname(sys.executable)
    for name in ["gt", "gt.exe"]:
        path = os.path.join(scripts_dir, name)
        if os.path.exists(path):
            return path

    raise RuntimeError(
        "gt binary not found. Install with: pip install stkd-cli, or cargo install stkd-cli"
    )


def run(args=None, capture_output=True, text=True, **kwargs):
    """Run the gt CLI with the given arguments.

    Args:
        args: List of arguments (e.g., ["log", "--json"])
        capture_output: Whether to capture stdout/stderr
        text: Whether to return strings instead of bytes
        **kwargs: Additional arguments for subprocess.run

    Returns:
        CompletedProcess instance
    """
    binary = _find_binary()
    cmd = [binary] + (args or [])
    return subprocess.run(cmd, capture_output=capture_output, text=text, **kwargs)


def run_json(args=None, **kwargs):
    """Run gt with --json and parse the output as JSON.

    Args:
        args: List of arguments
        **kwargs: Additional arguments for subprocess.run

    Returns:
        Parsed JSON object
    """
    args = (args or []) + ["--json"]
    result = run(args, capture_output=True, text=True, **kwargs)
    result.check_returncode()
    return json.loads(result.stdout)


def main():
    """Entry point for the gt console script."""
    binary = _find_binary()
    os.execvp(binary, [binary] + sys.argv[1:])
