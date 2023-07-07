FROM gitpod/workspace-full-vnc
RUN sudo apt-get update \
    && sudo apt-get install -y libgtk-3-dev \
    && sudo apt-get clean \
    && sudo rm -rf /var/cache/apt/* \
    && sudo rm -rf /var/lib/apt/lists/* \
    && sudo rm -rf /tmp/*

# Headless software rendering: https://github.com/gitpod-io/definitely-gp/blob/30f6a09fa5f0a7eaaa9cd0b7174b534a738bf21b/servo/.gitpod.dockerfile
# Addresses: https://github.com/gitpod-io/gitpod/issues/1876
# Enable required Xvfb extensions for Servo.
# Source: https://github.com/servo/servo/issues/7512#issuecomment-216665988
RUN sed -i "s/\(Xvfb .*\)&\s*$/\1+extension RANDR +extension RENDER +extension GLX \&/" /usr/bin/start-vnc-session.sh
# FIXME: Maybe also add "-pn" ?