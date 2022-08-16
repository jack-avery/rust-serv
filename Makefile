# inventory in ~/.inventory.yml

all: build deploy

build:
	@ansible-playbook playbooks/build.yml

deploy:
	@ansible-playbook playbooks/deploy.yml