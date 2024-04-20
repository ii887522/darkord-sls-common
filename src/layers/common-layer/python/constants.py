import os

# Stages
STAGING = "stage"
PRODUCTION = "prod"

# Environment variables
LOG_LEVEL = os.environ.get("LOG_LEVEL", "INFO")
REGION = os.environ.get("REGION", "us-east-1")
STAGE = os.environ.get("STAGE", STAGING)
STAGE_PREFIX = os.environ.get("STAGE_PREFIX", "stage_")
STAGE_DASH_PREFIX = os.environ.get("STAGE_DASH_PREFIX", "stage-")

# Timestamps
# Assume that these timestamps are not reachable
MAX_TIMESTAMP_IN_SECONDS = 10_000_000_000
MAX_TIMESTAMP_IN_MILLISECONDS = 10_000_000_000_000
