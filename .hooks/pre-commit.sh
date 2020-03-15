#!/usr/bin/env sh
# pre-commit.sh

# Colors
RED='\033[1;31m'
GREEN='\033[1;32m'
YELLOW='\033[1;33m'
NC='\033[0m'
BOLD='\033[1m'

# Git Metadata
ROOT_DIR="$(git rev-parse --show-toplevel)"
BUILD_DIR="${ROOT_DIR}/target"
BRANCH_NAME=$(git branch | grep '*' | sed 's/* //')
STASH_NAME="pre-commit-$(date +%s) on ${BRANCH_NAME}"

echo "* ${BOLD}Checking for unstashed changes:${NC}"
stash=0
# Check to make sure commit isn't empty
if git diff-index --cached --quiet HEAD --; then
    # It was empty, exit with status 0 to let git handle it
    exit 0
else
    # Stash changes that aren't added to the staging index so we test
    # only the changes to be committed
    old_stash=$(git rev-parse -q --verify refs/stash)
    git stash push -q --keep-index -m "$STASH_NAME"
    new_stash=$(git rev-parse -q --verify refs/stash)

    echo "  - Stashed changes as: ${BOLD}${STASH_NAME}${NC}"
    if [ "$old_stash" = "$new_stash" ]; then
        echo "  - no changes, ${YELLOW}skipping tests${NC}"
        exit 0
    else
        stash=1
    fi
fi

echo "* ${BOLD}Testing:${NC}"
git diff --cached --stat
echo ""

# use && to combine test commands so if any one fails it's accurately represented
# in the exit code
cargo check --all-targets --examples --benches &&
cargo test &&
cargo +nightly fmt --all -- --check

# Capture exit code from tests
status=$?

# Inform user of build failure
echo "* ${BOLD}Build status:${NC}"
if [ "$status" -ne "0" ]
then
    echo "  - ${RED}failed:${NC} if you still want to commit use ${BOLD}'--no-verify'${NC}"
else
    echo "  - ${GREEN}passed${NC}"
fi

# Revert stash if changes were stashed to restore working directory files
if [ "$stash" -eq 1 ]
then
    echo "* ${BOLD}Resotring working tree${NC}"
    if git reset --hard -q &&
       git stash apply --index -q &&
       git stash drop -q
    then
        echo "  - ${GREEN}restored${NC} ${STASH_NAME}"
    else
        echo "  - ${RED}unable to revert stash command${NC}"
    fi
fi

# Exit with exit code from tests, so if they fail, prevent commit
exit $status
