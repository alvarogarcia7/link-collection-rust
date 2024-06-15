IMAGE_NAME:=mygostub
IMAGE_TAG:=latest

build:
	docker build --tag $(IMAGE_NAME):$(IMAGE_TAG) .
.PHONY: build

push:
	docker push $(IMAGE_NAME):$(IMAGE_TAG)
.PHONY: push

bash:
	docker run -it --rm $(IMAGE_NAME):$(IMAGE_TAG) bash
.PHONY: bash

rm:
	docker rmi $(IMAGE_NAME):$(IMAGE_TAG)
.PHONY: rm
