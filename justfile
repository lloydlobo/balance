#!/usr/bin/env -S just --justfile
# ^ A shebang isn't required, but allows a justfile to be executed
#   like a script, with `./justfile test`, for example.
# Source https://github.com/casey/just/blob/master/justfile

# choose a recipe interactively
default:
  just --list

#
# VARIABLES
#

docker_image := "balance"
docker_container_name := 'balance1'
username := 'lloydlobo'



##############################################
##                 DOCKER                   ##
##############################################

#
# BUILD
#

docker-build:
  docker build -t {{docker_image}} .

#
# DEV
#

# run cmd container in detatched mode at 8080
docker-run:
  docker run -dp 8080:3030 --rm --name {{docker_container_name}} {{docker_image}}

# Run the binary interactively in the terminal `$ ./balance`
docker-entrypoint:
  docker run -it --rm --name {{docker_container_name}} --entrypoint bin/bash {{docker_image}}

# fetches container logs & follows log output
docker-logs:
  docker logs -f {{docker_container_name}}

# stop container `sample1`
docker-stop:
  docker stop {{docker_container_name}}


#
# RELEASE
#

# tags the image with a docker tag
docker-tag-latest:
  docker tag {{docker_image}} {{username}}/{{docker_image}}

# pushes to hub with default tag `latest` if not specified
docker-push:
  docker push {{username}}/{{docker_image}}

