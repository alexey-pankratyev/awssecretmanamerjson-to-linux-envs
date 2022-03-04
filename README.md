# awssecretmanamerjson-to-linux-envs

This is a simple Rust script that generates a script file from the Aws Secret Manager json file for exporting variables to linux

---

**Launch example:**
```bash
    make_envs  ~/Templates/aws_secrets.json  ~/Templates/env_secrets_export.sh
```