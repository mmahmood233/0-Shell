#!/bin/bash
# Comprehensive test script for 0-Shell

echo "=== Testing 0-Shell Commands ==="
echo ""

# Create test directory
mkdir -p test_env
cd test_env

# Create test files
echo "Hello World" > test1.txt
echo "Test File 2" > test2.txt
mkdir subdir

echo "Test environment created"
echo ""

# Test commands via echo piping to shell
echo "=== Test 1: pwd ==="
echo "pwd" | ../target/release/zero-shell

echo ""
echo "=== Test 2: ls ==="
echo "ls" | ../target/release/zero-shell

echo ""
echo "=== Test 3: ls -l ==="
echo "ls -l" | ../target/release/zero-shell

echo ""
echo "=== Test 4: ls -a ==="
echo "ls -a" | ../target/release/zero-shell

echo ""
echo "=== Test 5: ls -F ==="
echo "ls -F" | ../target/release/zero-shell

echo ""
echo "=== Test 6: cat test1.txt ==="
echo "cat test1.txt" | ../target/release/zero-shell

echo ""
echo "=== Test 7: echo Hello There ==="
echo "echo Hello There" | ../target/release/zero-shell

echo ""
echo "=== Test 8: cp test1.txt test_copy.txt ==="
echo "cp test1.txt test_copy.txt" | ../target/release/zero-shell
echo "ls" | ../target/release/zero-shell

echo ""
echo "=== Test 9: mkdir newdir ==="
echo "mkdir newdir" | ../target/release/zero-shell
echo "ls -F" | ../target/release/zero-shell

echo ""
echo "=== Test 10: mv test_copy.txt moved.txt ==="
echo "mv test_copy.txt moved.txt" | ../target/release/zero-shell
echo "ls" | ../target/release/zero-shell

echo ""
echo "=== Test 11: rm moved.txt ==="
echo "rm moved.txt" | ../target/release/zero-shell
echo "ls" | ../target/release/zero-shell

echo ""
echo "=== Test 12: rm -r newdir ==="
echo "rm -r newdir" | ../target/release/zero-shell
echo "ls" | ../target/release/zero-shell

echo ""
echo "=== Test 13: cd subdir && pwd ==="
(echo "cd subdir"; echo "pwd") | ../target/release/zero-shell

echo ""
echo "=== Test 14: Invalid command ==="
echo "something" | ../target/release/zero-shell

echo ""
echo "=== Test 15: Multiple echo args ==="
echo 'echo "Hello There"' | ../target/release/zero-shell

# Cleanup
cd ..
rm -rf test_env

echo ""
echo "=== All tests completed ==="
