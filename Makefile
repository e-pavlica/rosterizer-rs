DOCKER_CMD := docker
BUILD_OPTS :=
RUN_ARGS :=
RUN_OPTS :=

build:
	$(DOCKER_CMD) compose run --rm builder cargo build $(BUILD_OPTS)

run:
	$(DOCKER_CMD) compose run --rm builder cargo run $(RUN_OPTS) -- $(RUN_ARGS)

clean:
	rm -rf target
