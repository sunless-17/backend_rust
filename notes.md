# quest
- better auth
- send queries < create suql
- crud
# docker
- setup surrealkv (remember to add it as a feature to surrealdb dependency)
```bash
docker run \
    --name db \
    --volume $(pwd)/<dir_name>:/<dir_name> \
    --publish 8000:8000 \
    # give docker permissions to edit and create stuffs in the folder 
    --user $(id -u) \
    surrealdb/surrealdb:latest start \
    --user <user_name> \
    --pass <user_password> \
    surrealkv://<dir_name>
```
