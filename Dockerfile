FROM gitpod/workspace-full-vnc
RUN sudo apt-get update && \
	sudo apt-get install -y libx11-dev libxkbfile-dev libsecret-1-dev libgconf2-dev libnss3 libgtk-3-dev libasound2-dev twm && \
	sudo rm -rf /var/lib/apt/lists/*