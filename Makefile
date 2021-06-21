server-dev:
	fd -g "*.rs" | entr -r cargo run

client-dev:
	cd client && npm run dev

dev:
	make server-dev &
	make client-dev

publish:
	docker build . -t "thaibeouu/it-job-aggs:latest"
	docker push "thaibeouu/it-job-aggs:latest"
