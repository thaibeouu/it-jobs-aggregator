server-dev:
	fd -g "*.rs" | entr -r cargo run

client-dev:
	cd client && npm run dev

dev:
	make server-dev &
	make client-dev

publish:
	docker build . | grep "Successfully built" | awk '{ print $NF }' | xargs -I@ docker image tag @ thaibeouu/it-job-aggs:latest && docker push thaibeouu/it-job-aggs:latest
