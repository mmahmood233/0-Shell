#!/bin/bash
# Manual Audit Tests - Run from project directory

SHELL_BIN="./target/release/zero-shell"

echo "=== 0-SHELL MANUAL AUDIT TESTS ==="
echo ""

# Test 1: Basic shell prompt
echo "TEST 1: Shell displays prompt and exits properly"
echo "exit" | $SHELL_BIN
echo "Status: $?"
echo ""

# Test 2: Echo with quotes
echo "TEST 2: echo \"something!\""
echo 'echo "something!"' | $SHELL_BIN | tail -2
echo ""

# Test 3: Echo without quotes
echo "TEST 3: echo something else"
echo 'echo something else' | $SHELL_BIN | tail -2
echo ""

# Test 4: pwd
echo "TEST 4: pwd"
echo 'pwd' | $SHELL_BIN | tail -2
echo ""

# Test 5: Create test environment
echo "TEST 5: Creating test environment"
mkdir -p /tmp/audit_test
cd /tmp/audit_test
echo ""

# Test 6: mkdir and cd
echo "TEST 6: mkdir parent, mkdir parent/child1, mkdir parent/child2"
(echo "mkdir parent"; echo "mkdir parent/child1"; echo "mkdir parent/child2"; echo "ls -F") | $SHELL_BIN | tail -5
ls -F
echo ""

# Test 7: cd and pwd
echo "TEST 7: cd parent && pwd"
(echo "cd parent"; echo "pwd") | $SHELL_BIN | tail -3
echo ""

# Test 8: cd to specific path
echo "TEST 8: cd parent/child1 && pwd"
(echo "cd parent/child1"; echo "pwd") | $SHELL_BIN | tail -3
echo ""

# Test 9: cd with no args
echo "TEST 9: cd (no args) && pwd"
(echo "cd"; echo "pwd") | $SHELL_BIN | tail -3
echo ""

# Test 10: ls
echo "TEST 10: ls"
(echo "ls") | $SHELL_BIN | tail -5
echo ""

# Test 11: ls with flags
echo "TEST 11: ls -l -a -F"
(echo "ls -l -a -F") | $SHELL_BIN | tail -10
echo ""

# Test 12: mkdir multiple
echo "TEST 12: mkdir new_folder1 new_folder2"
(echo "mkdir new_folder1"; echo "mkdir new_folder2"; echo "ls -F") | $SHELL_BIN | tail -8
ls -F
echo ""

# Test 13: Create file and cp
echo "TEST 13: Create file and cp"
echo "Random text content" > new_folder1/new_doc.txt
(echo "cp new_folder1/new_doc.txt new_folder2/"; echo "ls new_folder2") | $SHELL_BIN | tail -5
ls new_folder2/
echo ""

# Test 14: cat
echo "TEST 14: cat new_folder1/new_doc.txt"
(echo "cat new_folder1/new_doc.txt") | $SHELL_BIN | tail -3
cat new_folder1/new_doc.txt
echo ""

# Test 15: mv
echo "TEST 15: mv new_folder2 new_folder1/"
(echo "mv new_folder2 new_folder1/"; echo "ls -F") | $SHELL_BIN | tail -6
ls -F
ls -F new_folder1/
echo ""

# Test 16: rm -r
echo "TEST 16: rm -r new_folder1"
(echo "rm -r new_folder1"; echo "ls") | $SHELL_BIN | tail -4
ls
echo ""

# Cleanup
cd /
rm -rf /tmp/audit_test

echo "=== AUDIT TESTS COMPLETE ==="
