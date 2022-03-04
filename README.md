# Awssecretmanamerjson-to-linux-envs

This is a simple Rust script that generates a script file from the Aws Secret Manager json file for exporting variables to linux

---

**Build and Push docker image with scipt make_envs**

*Preparation: before running the docker_publish command, you need to specify your docker repository in the DOCKER_REGISTRY_REPO parameter* 
```bash
   make docker_publish
   make clean
```

**Launch example:**
```bash
    make_envs  ~/Templates/aws_secrets.json  ~/Templates/env_secrets_export.sh
```

