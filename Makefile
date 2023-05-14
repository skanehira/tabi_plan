.PHONY: iamge-build
image-build:
	okteto build --platform=linux/amd64 -t okteto.dev/tabiplan:latest -f infra/Dockerfile
