# Script Overview

## sync-turbin3-fork.sh

This script syncs the personal repo with the turbin3 fork.

In essence it runs these commands:

```bash
git remote add $FORK_REMOTE_NAME $FORK_REPO_URL
git push $FORK_REMOTE_NAME main
```

### Options

Run it with `--force` to run the git push command forcefully.

### Setup

- `FORK_REPO_URL`: Put the SSH url of the fork here
- `SOURCE_BRANCH`: Name of the branch you want to push to the fork (usually `main`)
- `FORK_REMOTE_NAME`: Put the name for the git remote that the fork will be added (this can be any arbitrary name, e.g. `turbin3-fork`)

You might have to run this command to make the file executable:

```bash
chmod +x your_script.sh
```
