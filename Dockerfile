FROM gitpod/workspace-full-vnc
RUN sudo apt-get update \
    && sudo apt-get install -y libgtk-3-dev \
    && sudo apt install -y g++ pkg-config libx11-dev libasound2-dev libudev-dev \
    && sudo apt-get clean \
    && sudo rm -rf /var/cache/apt/* \
    && sudo rm -rf /var/lib/apt/lists/* \
    && sudo rm -rf /tmp/*
