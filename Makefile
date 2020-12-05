TAG_NAME=rusty-shorturl

build-prod:
	docker build -t ${TAG_NAME} .

build:
	docker build -t ${TAG_NAME}:dev -f dev.Dockerfile .

start-prod:
	docker run --init --rm -it -p 8000:8000 ${TAG_NAME}

start:
	docker run --init --rm -it -p 8000:8000 ${TAG_NAME}:dev

stop-prod:
	docker stop ${TAG_NAME}

stop:
	docker stop ${TAG_NAME}:dev

clean:
	docker system prune -f
