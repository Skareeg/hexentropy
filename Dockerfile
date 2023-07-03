FROM gitpod/workspace-full-vnc
RUN sudo apt-get update && \
	sudo apt-get install -y libgtk-3-dev libx11-dev libxkbfile-dev libsecret-1-dev libgconf2-4 libnss3 && \
	sudo rm -rf /var/lib/apt/lists/*