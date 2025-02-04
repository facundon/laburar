#!/bin/bash

set -e  # Exit on error

BRANCH_MAIN="main"
BRANCH_RELEASE="release"

# Ensure we're on the main branch
git checkout $BRANCH_MAIN
git pull origin $BRANCH_MAIN

# Bump minor version in src-tauri/tauri.conf.json
jq '.version |= (split(".") | .[1] |= (tonumber + 1 | tostring) | join("."))' src-tauri/tauri.conf.json > tmp-conf.json && mv tmp-conf.json src-tauri/tauri.conf.json

# Commit the version bump
git add src-tauri/tauri.conf.json
git commit -m "Bump minor version in src-tauri/tauri.conf.json"

# Merge main into release (fast-forward if possible)
git checkout $BRANCH_RELEASE
git pull origin $BRANCH_RELEASE
git reset --hard $BRANCH_MAIN  # Make release identical to main

# Push release branch
git push origin $BRANCH_RELEASE --force

echo "Deployment completed successfully!"
