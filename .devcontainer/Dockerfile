FROM docker.io/library/ubuntu:24.04

# Set environment variables
ENV SHELL="/bin/bash"
ENV BASH_FLAGS="-x"
ENV BIN=/usr/local/bin

# Install essential system packages and development dependencies
ARG APT_PACKAGES="\
jq \
git \
sudo \
curl \
cmake \
build-essential \
neovim \
tree \
avr-libc \
gcc-avr \
avrdude \
ca-certificates \
libssl-dev \
openssl \
pkg-config \
libudev-dev \
"
RUN set ${BASH_FLAGS} \
    && apt-get update \
    && apt-get install -y ${APT_PACKAGES} \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && echo

# Set up groups and permissions for the default ubuntu user
RUN set ${BASH_FLAGS} \
    && groupadd --force --system sudo \
    && groupadd --force --gid 127 --system docker \
    && usermod -aG sudo,docker ubuntu \
    && echo "%sudo ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/sudo \
    && chmod 0440 /etc/sudoers.d/sudo \
    && echo

# Ensure workspace directory exists with correct permissions
RUN mkdir -p /workspaces && chown ubuntu:ubuntu /workspaces

# Switch to ubuntu user for Rust installation
USER ubuntu
WORKDIR /home/ubuntu

# Add cargo binaries to PATH
ENV PATH="/home/ubuntu/.cargo/bin:${PATH}"

# Install Rust using rustup and required cargo tools
RUN set ${BASH_FLAGS} \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- \
        --default-toolchain nightly \
        --profile default \
        --component rust-src \
        -y \
    && cargo install cargo-generate \
    && cargo install ravedude \
    && echo

# Install GitHub CLI
USER root
RUN set ${BASH_FLAGS} \
    && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null \
    && apt-get update \
    && apt-get install -y gh \
    && rm -rf /var/lib/apt/lists/* \
    && echo

# Switch back to ubuntu user
USER ubuntu
WORKDIR /workspaces

EXPOSE 5150

CMD ["sleep", "infinity"]
