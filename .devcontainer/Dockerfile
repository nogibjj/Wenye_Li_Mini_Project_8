ARG VARIANT="buster"
FROM mcr.microsoft.com/vscode/devcontainers/rust:0-${VARIANT}

# Install Python and related tools
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends python3-pip python3-venv python3-dev \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install Python formatter and linter
RUN pip3 install black pylint pytest pytest-cov