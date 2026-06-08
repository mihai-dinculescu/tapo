"""Common utilities for examples."""

import os
import sys


def require_env_vars(*names: str) -> list[str]:
    """Read the given required environment variables, returning their values in order.

    If any are missing, exit with a message that lists all of them so the user can
    set everything at once instead of discovering them one at a time.
    """
    missing = [name for name in names if os.getenv(name) is None]
    if missing:
        sys.exit(f"missing required environment variable(s): {', '.join(missing)}")

    return [os.environ[name] for name in names]
