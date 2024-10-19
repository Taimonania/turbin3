#!/bin/bash

# Fail the script if any command fails
set -e

# URL of the fork you want to sync
FORK_REPO_URL="git@github.com-personal:solana-turbin3/Q4_Sol_Taimonania.git" 

# Navigate to the current directory where the script is run
REPO_PATH="$(pwd)"
FORK_REMOTE_NAME="turbin3-fork" # Name for the git remote fork (can be anything)

# Ensure we're in a git repository
if [ ! -d "$REPO_PATH/.git" ]; then
    echo "Error: This script must be run from the root of a Git repository."
    exit 1
fi

# Check that we're on the main branch
git checkout main

# Fetch latest changes from the original repository
git pull origin main

# Add the fork as a remote if not already added
git remote get-url "$FORK_REMOTE_NAME" &>/dev/null || git remote add "$FORK_REMOTE_NAME" "$FORK_REPO_URL"

# Push the changes to the fork's main branch
git push "$FORK_REMOTE_NAME" main

echo "Fork has been successfully synced with the original repository."
