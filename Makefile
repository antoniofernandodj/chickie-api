export-docs:
	cp API.md ../../TS/chickie-ui/api_docs/API.md
	cp CLAUDE.md ../../TS/chickie-ui/api_docs/CLAUDE.md
	cp README.md ../../TS/chickie-ui/api_docs/README.md

export-proto:
	cp /home/antonio/Development/rust/chickie/proto/chickie.proto /home/antonio/Development/TS/chickie-ui/src/app/proto/chickie.proto
	cd /home/antonio/Development/TS/chickie-ui && make generate-proto

sync:
	git checkout main
	git merge $(b)
	for branch in main-api main-scheduler main-worker; do \
		git checkout $$branch; \
		git merge main; \
		git push; \
	done
	git checkout $(b)

test:
	export DATABASE_URL="postgres://myuser:mypassword@localhost:5432/mydatabase" && clurl tests/00-tests.clurl