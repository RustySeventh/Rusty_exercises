#!/bin/bash

# Reset Attempts Script
# This script resets the attempts branch to match setup branch exactly
# Run this when you want to start fresh from setup templates

echo "🔄 Resetting attempts branch to match setup..."

# Navigate to the attempts worktree
cd /mnt/c/Users/André\ Colin/Documents/Rusty_exercise

# Check if we're in the right place
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository"
    echo "Make sure you're running this from the correct location"
    exit 1
fi

# Verify we're on attempts branch
current_branch=$(git branch --show-current)
if [ "$current_branch" != "attempts" ]; then
    echo "❌ Error: Not on attempts branch (currently on: $current_branch)"
    exit 1
fi

# Reset attempts branch to exactly match setup branch
echo "🧹 Clearing all changes in attempts branch..."
git reset --hard setup

# Clean any untracked files
echo "🗑️  Removing untracked files..."
git clean -fd

# Force push to update remote attempts branch
echo "⬆️  Pushing reset to remote..."
git push origin attempts --force

echo "✅ Reset complete!"
echo "📁 Your attempts branch now matches setup branch exactly"
echo "🚀 Ready for fresh practice!"

# Optional: Show what files are now available
echo ""
echo "📋 Files available for practice:"
ls -la