#!/bin/bash

# Test script to debug the reset process
echo "🔍 Debugging reset process..."

echo "📍 Current location: $(pwd)"
echo "📋 Worktree list:"
git worktree list

echo ""
echo "🎯 Navigating to practice-attempts..."
cd "/mnt/c/Users/André Colin/Documents/Rusty_exercises"

echo "📍 New location: $(pwd)"
echo "🌿 Current branch: $(git branch --show-current)"
echo "📁 Files in directory:"
ls -la

echo ""
echo "🔄 Testing reset command..."
echo "This would run: git reset --hard setup"
echo "Continue? (y/n)"
read -r response

if [ "$response" = "y" ]; then
    git reset --hard setup
    echo "✅ Reset completed"
    echo "📁 Files after reset:"
    ls -la
else
    echo "❌ Reset cancelled"
fi