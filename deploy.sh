#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Fetch the latest changes from the remote repository
git fetch origin

# Checkout the main branch and pull the latest changes
git checkout main
git pull origin main

# Checkout the release branch and pull the latest changes
git checkout release
git pull origin release

# Merge the main branch into the release branch
git merge main

# Push the release branch to the remote repository
git push origin release

# Merge the release branch back into the main branch
git checkout main
git merge release

# Push the main branch to the remote repository
git push origin main

echo "Deployment completed successfully."
