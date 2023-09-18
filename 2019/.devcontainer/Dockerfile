FROM --platform=arm64 ocaml/opam:debian-ocaml-5.0 as devcontainer

ENV DEBIAN_FRONTEND=noninteractive

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

USER root

RUN apt-get update \
    && apt-get -y install --no-install-recommends m4 \
    && apt-get -y install --no-install-recommends sudo \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

RUN if [ "$USER_GID" != "1000" ] || [ "$USER_UID" != "1000" ]; then \
    groupmod --gid $USER_GID $USERNAME \
    && usermod --uid $USER_UID --gid $USER_GID $USERNAME \
    && chown -R $USER_UID:$USER_GID /home/$USERNAME; \
    fi

USER opam

RUN opam update && \
    opam install dune merlin ocaml-lsp-server odoc ocamlformat utop dune-release

ENV DEBIAN_FRONTEND=dialog

ENV PATH=/home/opam/.opam/5.0/bin:$PATH