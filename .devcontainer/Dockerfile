# TODO: adopt global version control for rust version
FROM docker.io/library/rust:1.81.0-bullseye
ENV SHELL="/bin/bash"
ENV BASH_FLAGS="-e"
ENV BIN=/usr/local/bin

# Create a non-root user with same UID/GID as host user
ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Install basic tools
ARG APT_LIST="\
jq \
git \
sudo \
curl \
cmake \
build-essential \
neovim \
tree \
"
RUN set ${BASH_FLAGS} \
    && apt-get update \
    && apt-get install -y ${APT_LIST} \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/* \
    && echo

# Create dev user `vscode`
RUN set ${BASH_FLAGS} \
  && mkdir -p /etc/sudoers.d \
  && groupadd --force --system sudo \
  && groupadd --force --gid 127 --system docker \
  && echo "%sudo ALL=(ALL:ALL) NOPASSWD: ALL" > /etc/sudoers.d/sudo \
  && groupadd --force --gid ${USER_GID} ${USERNAME} \
  && useradd \
    --create-home \
    --uid ${USER_UID} \
    --gid ${USER_GID} \
    --shell $(which bash) \
    --groups sudo,docker \
    ${USERNAME} \
  && echo

# Install GitHub `gh` cli from github.com/cli/cli
RUN set ${BASH_FLAGS} \
  && export NAME=gh \
  && export TEST="${NAME} version" \
  && export REPOSITORY="cli/cli" \
  && export VERSION_TAG="$(curl -Lks https://api.github.com/repos/${REPOSITORY}/releases/latest | jq --raw-output .tag_name)" \
  && export VERSION="${VERSION_TAG#v}" \
  && export ARCH=$(uname -m | awk '{ if ($1 == "x86_64") print "amd64"; else if ($1 == "aarch64" || $1 == "arm64") print "arm64"; else print "unknown" }') \
  && export PKG="${NAME}_${VERSION}_linux_${ARCH}.tar.gz" \
  && export URL="https://github.com/${REPOSITORY}/releases/download/${VERSION_TAG}/${PKG}" \
  && echo "---------------------------------------------------------"\
  && echo "INFO[${NAME}] Installed:" \
  && echo "INFO[${NAME}]   Command:        ${NAME}" \
  && echo "INFO[${NAME}]   Package:        ${PKG}" \
  && echo "INFO[${NAME}]   Latest Release: ${VERSION_TAG}" \
  && echo "INFO[${NAME}]   Architecture:   ${ARCH}" \
  && echo "INFO[${NAME}]   Source:         ${URL}" \
  && echo "---------------------------------------------------------"\
  && curl -Lks ${URL} | tar xzvf - --directory /tmp \
  && sudo install -m 755 /tmp/${NAME}_${VERSION}_linux_${ARCH}/bin/${NAME} ${BIN}/${NAME} \
  && rm -rf /tmp/${NAME}_${VERSION}_linux_${ARCH} \
  && rm -rf /var/lib/apt/lists/* \
  && apt-get clean \
  && ${TEST} \
  && echo

# Install Gitpod CLI from gitpod.io
RUN set ${BASH_FLAGS} \
  && export NAME=gp \
  && export TEST="${NAME} version" \
  && export VERSION="$(curl -s https://gitpod.io/static/bin/manifest.json | jq --raw-output .version)" \
  && export ARCH=$(uname -m | awk '{ if ($1 == "x86_64") print "amd64"; else if ($1 == "aarch64" || $1 == "arm64") print "arm64"; else print "unknown" }') \
  && export PKG="gitpod-cli-linux-${ARCH}" \
  && export URL="https://gitpod.io/static/bin/${PKG}" \
  && echo "---------------------------------------------------------" \
  && echo "INFO[${NAME}] Installed:" \
  && echo "INFO[${NAME}]   Command:        ${NAME}" \
  && echo "INFO[${NAME}]   Binary:         ${PKG}" \
  && echo "INFO[${NAME}]   Version:        ${VERSION}" \
  && echo "INFO[${NAME}]   Architecture:   ${ARCH}" \
  && echo "INFO[${NAME}]   Source:         ${URL}" \
  && echo "---------------------------------------------------------" \
  && curl -fSL ${URL} -o /tmp/${PKG} \
  && chmod +x /tmp/${PKG} \
  && sudo mv /tmp/${PKG} /usr/local/bin/${NAME} \
  && ${TEST} \
  && echo

# Install GitLab CLI (glab) from gitlab.com/gitlab-org/cli
RUN set ${BASH_FLAGS} \
  && export NAME=glab \
  && export TEST="${NAME} version" \
  && export VERSION_TAG="$(curl -s "https://gitlab.com/api/v4/projects/gitlab-org%2Fcli/releases" | jq -r ".[0].tag_name")" \
  && export VERSION="${VERSION_TAG#v}" \
  && export ARCH=$(uname -m | awk '{ if ($1 == "x86_64") print "amd64"; else if ($1 == "aarch64" || $1 == "arm64") print "arm64"; else print "unknown" }') \
  && export PKG="${NAME}_${VERSION}_linux_${ARCH}.tar.gz" \
  && export URL="https://gitlab.com/gitlab-org/cli/-/releases/${VERSION_TAG}/downloads/${PKG}" \
  && echo "---------------------------------------------------------" \
  && echo "INFO[${NAME}] Installed:" \
  && echo "INFO[${NAME}]   Command:        ${NAME}" \
  && echo "INFO[${NAME}]   Package:        ${PKG}" \
  && echo "INFO[${NAME}]   Latest Release: ${VERSION_TAG}" \
  && echo "INFO[${NAME}]   Architecture:   ${ARCH}" \
  && echo "INFO[${NAME}]   Source:         ${URL}" \
  && echo "---------------------------------------------------------" \
  && curl -fSL ${URL} | tar xz -C /tmp \
  && sudo mv /tmp/bin/glab /usr/local/bin/glab \
  && ${TEST} \
  && echo

# Set working directory
WORKDIR /workspaces/veilid

# Set the default user
USER $USERNAME

# Expose port
EXPOSE 5150

# Set default shell to bash
CMD ["sleep", "infinity"]
