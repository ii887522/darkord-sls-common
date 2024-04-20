import os

# Stages
STAGING = "stage"
PRODUCTION = "prod"

# Environment variables
LOG_LEVEL = os.environ["LOG_LEVEL"]
REGION = os.environ["REGION"]
STAGE = os.environ["STAGE"]
STAGE_PREFIX = os.environ["STAGE_PREFIX"]
STAGE_DASH_PREFIX = os.environ["STAGE_DASH_PREFIX"]

# Timestamps
# Assume that these timestamps are not reachable
MAX_TIMESTAMP_IN_SECONDS = 10_000_000_000
MAX_TIMESTAMP_IN_MILLISECONDS = 10_000_000_000_000
