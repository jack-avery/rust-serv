# inventory in ~/.inventory.yml

build:
	@ansible-playbook playbooks/build.yml

deploy:
	@ansible-playbook playbooks/deploy.yml