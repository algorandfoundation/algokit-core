# pyright: reportUnusedCallResult=false
from pathlib import Path
import subprocess
import select
import sys

if len(sys.argv) != 2:
    print("Crate must be specified as the argument")
    sys.exit(1)

crate = sys.argv[1].replace("_ffi", "")

def to_pascal_case(string: str) -> str:
    return string.title().replace(" ", "").replace("_", "")

def run(command: str, *, cwd: str | None = None):
    """
    Run a subprocess and print stdout and stderr in real-time

    Args:
        command (list): Command and arguments as a list (e.g., ["ls", "-la"])
    """
    print(f"Running '{command}'")
    process = subprocess.Popen(
        command.split(" "), stdout=subprocess.PIPE, stderr=subprocess.PIPE, cwd=cwd or Path(__file__).parents[2]
    )

    # Use select to read from stdout and stderr without blocking
    while True:
        assert process.stdout is not None
        assert process.stderr is not None
        reads = [process.stdout.fileno(), process.stderr.fileno()]
        ret: tuple[list[int], list[int], list[int]] = select.select(reads, [], [])

        for fd in ret[0]:
            if fd == process.stdout.fileno():
                line = process.stdout.readline()
                if line:
                    sys.stdout.buffer.write(line)
                    sys.stdout.buffer.flush()
            if fd == process.stderr.fileno():
                line = process.stderr.readline()
                if line:
                    sys.stderr.buffer.write(line)
                    sys.stderr.buffer.flush()

        if process.poll() is not None:
            break

    if process.wait() != 0:
        sys.exit(process.returncode)
