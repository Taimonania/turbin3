#!/bin/bash

# Fail the script if any command fails
set -e

### Configuration START ###

# URL of the fork you want to sync
FORK_REPO_URL="git@github.com-personal:solana-turbin3/Q4_Sol_Taimonania.git" 
# Name of the source branch to push from
SOURCE_BRANCH="main"
# Name for the git remote fork (can be anything)
FORK_REMOTE_NAME="turbin3-fork" 

### Configuration END ###

# Navigate to the current directory where the script is run
REPO_PATH="$(pwd)"

# Ensure we're in a git repository
if [ ! -d "$REPO_PATH/.git" ]; then
    echo "Error: This script must be run from the root of a Git repository."
    exit 1
fi

# Check for the '--force' argument
FORCE_PUSH="false"
if [[ "$1" == "--force" ]]; then
    FORCE_PUSH="true"
fi

# Check that we're on the main branch
git checkout $SOURCE_BRANCH

# Fetch latest changes from the original repository
git pull origin $SOURCE_BRANCH

# Add the fork as a remote if not already added
git remote get-url "$FORK_REMOTE_NAME" &>/dev/null || git remote add "$FORK_REMOTE_NAME" "$FORK_REPO_URL"

# Push the changes to the fork's main branch, optionally with --force
if [[ "$FORCE_PUSH" == "true" ]]; then
    git push "$FORK_REMOTE_NAME" $SOURCE_BRANCH --force
else
    git push "$FORK_REMOTE_NAME" $SOURCE_BRANCH
fi

echo "Fork has been successfully synced with the original repository."
