build:
	docker build -t jmaguire5588/rust ./

run:
	docker run -d -v /Users/jmaguire/projects/TheRustBook:/TheRustBook --name rustbook -it jmaguire5588/rust /bin/bash

exec:
	docker exec -it rustbook /bin/bash
