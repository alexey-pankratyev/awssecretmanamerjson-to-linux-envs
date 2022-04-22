# awssecretmanamerjson-to-linux-envs

This is a simple Rust script that generates a script file from the Aws Secret Manager json file for exporting variables to linux

---

**Launch example:**
```bash
   make_envs ( settings_local or sh_envs ) ~/Templates/secret_sm_pass.json  ~/Templates/settings_local.py
   usage:
   make_envs {settings_local|sh_envs} <string>
             {path to file name with secrets} <string>
             {path to file name which will be generated } <string>

```