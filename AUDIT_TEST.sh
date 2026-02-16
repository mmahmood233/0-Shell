#!/bin/bash
# Comprehensive Audit Test Script for 0-Shell
# This script tests all audit requirements

SHELL_BIN="./target/release/zero-shell"
TEST_DIR="/tmp/0shell_audit_test_$$"
RESULTS_FILE="/tmp/audit_results_$$.txt"

echo "=== 0-Shell Audit Test Suite ===" | tee "$RESULTS_FILE"
echo "Test Directory: $TEST_DIR" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Create clean test environment
mkdir -p "$TEST_DIR"
cd "$TEST_DIR" || exit 1

# Helper function to run shell command
run_shell_cmd() {
    echo "$1" | $SHELL_BIN 2>&1 | grep -v "0-Shell v" | grep -v "^$"
}

echo "=== FUNCTIONAL TESTS ===" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"

# Test 1: Shell runs and displays prompt
echo "Test 1: Shell displays prompt" | tee -a "$RESULTS_FILE"
echo "exit" | $SHELL_BIN 2>&1 | head -2 | tail -1 | grep -q '\$'
if [ $? -eq 0 ]; then
    echo "✅ PASS: Shell displays $ prompt" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: Shell does not display prompt" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 2: Exit command
echo "Test 2: Exit command terminates properly" | tee -a "$RESULTS_FILE"
echo "exit" | timeout 2 $SHELL_BIN > /dev/null 2>&1
if [ $? -eq 0 ]; then
    echo "✅ PASS: Exit command works" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: Exit command failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 3: Echo with quotes
echo "Test 3: echo \"something!\"" | tee -a "$RESULTS_FILE"
SHELL_OUTPUT=$(run_shell_cmd 'echo "something!"' | tail -1)
BASH_OUTPUT=$(echo "something!")
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Bash output:  $BASH_OUTPUT" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$BASH_OUTPUT" ]; then
    echo "✅ PASS: Echo with quotes matches bash" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: Echo output differs" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 4: Echo without quotes
echo "Test 4: echo something else" | tee -a "$RESULTS_FILE"
SHELL_OUTPUT=$(run_shell_cmd 'echo something else' | tail -1)
BASH_OUTPUT=$(echo something else)
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Bash output:  $BASH_OUTPUT" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$BASH_OUTPUT" ]; then
    echo "✅ PASS: Echo without quotes matches bash" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: Echo output differs" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 5: pwd command
echo "Test 5: pwd displays current path" | tee -a "$RESULTS_FILE"
SHELL_OUTPUT=$(run_shell_cmd 'pwd' | tail -1)
EXPECTED="$TEST_DIR"
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Expected:     $EXPECTED" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$EXPECTED" ]; then
    echo "✅ PASS: pwd displays correct path" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: pwd output incorrect" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 6: mkdir and cd
echo "Test 6: mkdir parent/child1 child2, then cd and pwd" | tee -a "$RESULTS_FILE"
(echo "mkdir parent"; echo "mkdir parent/child1"; echo "mkdir parent/child2"; echo "cd parent"; echo "pwd") | $SHELL_BIN 2>&1 | grep -v "0-Shell" | grep -v "^$" > /tmp/test6_output.txt
SHELL_OUTPUT=$(tail -1 /tmp/test6_output.txt)
EXPECTED="$TEST_DIR/parent"
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Expected:     $EXPECTED" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$EXPECTED" ] && [ -d "parent/child1" ] && [ -d "parent/child2" ]; then
    echo "✅ PASS: mkdir and cd work correctly" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: mkdir or cd failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 7: cd to specific directory
echo "Test 7: cd to specific directory" | tee -a "$RESULTS_FILE"
(echo "cd parent/child1"; echo "pwd") | $SHELL_BIN 2>&1 | grep -v "0-Shell" | grep -v "^$" > /tmp/test7_output.txt
SHELL_OUTPUT=$(tail -1 /tmp/test7_output.txt)
EXPECTED="$TEST_DIR/parent/child1"
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Expected:     $EXPECTED" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$EXPECTED" ]; then
    echo "✅ PASS: cd to specific directory works" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: cd failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 8: cd with no arguments (home directory)
echo "Test 8: cd with no arguments goes to HOME" | tee -a "$RESULTS_FILE"
(echo "cd"; echo "pwd") | $SHELL_BIN 2>&1 | grep -v "0-Shell" | grep -v "^$" > /tmp/test8_output.txt
SHELL_OUTPUT=$(tail -1 /tmp/test8_output.txt)
EXPECTED="$HOME"
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Expected:     $EXPECTED" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$EXPECTED" ]; then
    echo "✅ PASS: cd with no args goes to HOME" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: cd without args failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 9: ls command
echo "Test 9: ls command output" | tee -a "$RESULTS_FILE"
SHELL_OUTPUT=$(run_shell_cmd 'ls' | tail -1)
if [ -n "$SHELL_OUTPUT" ] && echo "$SHELL_OUTPUT" | grep -q "parent"; then
    echo "✅ PASS: ls displays directory contents" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: ls output incorrect" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 10: ls with flags
echo "Test 10: ls -l -a -F" | tee -a "$RESULTS_FILE"
run_shell_cmd 'ls -l -a -F' > /tmp/shell_ls.txt
ls -l -a -F > /tmp/bash_ls.txt
SHELL_LINES=$(cat /tmp/shell_ls.txt | wc -l)
BASH_LINES=$(cat /tmp/bash_ls.txt | wc -l)
echo "Shell output lines: $SHELL_LINES" | tee -a "$RESULTS_FILE"
echo "Bash output lines:  $BASH_LINES" | tee -a "$RESULTS_FILE"
if [ "$SHELL_LINES" -gt 0 ] && grep -q "parent/" /tmp/shell_ls.txt; then
    echo "✅ PASS: ls -l -a -F works" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: ls with flags failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 11: mkdir multiple folders
echo "Test 11: mkdir new_folder1 and new_folder2" | tee -a "$RESULTS_FILE"
(echo "mkdir new_folder1"; echo "mkdir new_folder2") | $SHELL_BIN > /dev/null 2>&1
if [ -d "new_folder1" ] && [ -d "new_folder2" ]; then
    echo "✅ PASS: Multiple directories created" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: mkdir failed to create directories" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 12: cp command
echo "Test 12: Create file and cp to another folder" | tee -a "$RESULTS_FILE"
echo "Random text content" > new_folder1/new_doc.txt
(echo "cp new_folder1/new_doc.txt new_folder2/") | $SHELL_BIN > /dev/null 2>&1
if [ -f "new_folder2/new_doc.txt" ]; then
    echo "✅ PASS: cp command works" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: cp command failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 13: cat command
echo "Test 13: cat command output" | tee -a "$RESULTS_FILE"
SHELL_OUTPUT=$(run_shell_cmd 'cat new_folder1/new_doc.txt' | tail -1)
BASH_OUTPUT=$(cat new_folder1/new_doc.txt)
echo "Shell output: $SHELL_OUTPUT" | tee -a "$RESULTS_FILE"
echo "Bash output:  $BASH_OUTPUT" | tee -a "$RESULTS_FILE"
if [ "$SHELL_OUTPUT" = "$BASH_OUTPUT" ]; then
    echo "✅ PASS: cat output matches bash" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: cat output differs" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 14: mv command
echo "Test 14: mv new_folder2 into new_folder1" | tee -a "$RESULTS_FILE"
(echo "mv new_folder2 new_folder1/") | $SHELL_BIN > /dev/null 2>&1
if [ -d "new_folder1/new_folder2" ] && [ ! -d "new_folder2" ]; then
    echo "✅ PASS: mv command works" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: mv command failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Test 15: rm -r command
echo "Test 15: rm -r new_folder1" | tee -a "$RESULTS_FILE"
(echo "rm -r new_folder1") | $SHELL_BIN > /dev/null 2>&1
if [ ! -d "new_folder1" ]; then
    echo "✅ PASS: rm -r removes directory" | tee -a "$RESULTS_FILE"
else
    echo "❌ FAIL: rm -r failed" | tee -a "$RESULTS_FILE"
fi
echo "" | tee -a "$RESULTS_FILE"

# Cleanup
cd /
rm -rf "$TEST_DIR"

echo "=== AUDIT TEST COMPLETE ===" | tee -a "$RESULTS_FILE"
echo "" | tee -a "$RESULTS_FILE"
echo "Results saved to: $RESULTS_FILE" | tee -a "$RESULTS_FILE"

# Display results file
cat "$RESULTS_FILE"
