#!/usr/bin/env python3

import os
import re

def fix_test_file(file_path):
    """Fix test error handling in a single file"""
    with open(file_path, 'r') as f:
        content = f.read()

    # Pattern to match the problematic error handling
    old_pattern = r'(\s+)match env_result \{\s*Ok\(env\) => \{[^}]+\}\s*Err\(e\) => \{\s*match e \{\s*SzError::Configuration \{ \.\. \} => \{\s*// Expected for singleton architecture\s*\}\s*_ => \{\s*eprintln!\("Environment initialization failed \(may be acceptable\): \{:\?\}", e\);\s*\}\s*\}\s*\}\s*\}'

    # New pattern that properly fails on initialization errors
    new_pattern = r'''\1match env_result {
\1    Ok(env) => {
\1        // Test that environment is available for testing
\1        eprintln!("Environment available for testing");
\1
\1        // Can test read-only operations like:
\1        let _is_destroyed = env.is_destroyed();
\1        let _active_config_result = env.get_active_config_id();
\1    }
\1    Err(e) => {
\1        // With proper synchronization, initialization should succeed
\1        // Any initialization failure is now a test failure
\1        return Err(e);
\1    }
\1}'''

    # Apply the fix
    content = re.sub(old_pattern, new_pattern, content, flags=re.MULTILINE | re.DOTALL)

    # Also fix the engine setup pattern
    engine_old_pattern = r'(\s+)match engine_result \{\s*Ok\(engine\) => \{[^}]+\}\s*Err\(e\) => \{\s*match e \{\s*SzError::Configuration \{ \.\. \} => \{\s*// Expected for singleton architecture\s*\}\s*_ => \{\s*eprintln!\("Engine setup failed \(may be acceptable\): \{:\?\}", e\);\s*\}\s*\}\s*\}\s*\}'

    engine_new_pattern = r'''\1match engine_result {
\1    Ok(engine) => {
\1        // Engine is available for testing
\1        eprintln!("Engine available for testing");
\1    }
\1    Err(e) => {
\1        // With proper synchronization, engine setup should succeed
\1        // Any engine setup failure is now a test failure
\1        return Err(e);
\1    }
\1}'''

    content = re.sub(engine_old_pattern, engine_new_pattern, content, flags=re.MULTILINE | re.DOTALL)

    # Check if any changes were made
    with open(file_path, 'r') as f:
        original_content = f.read()

    if content != original_content:
        with open(file_path, 'w') as f:
            f.write(content)
        print(f"Fixed: {file_path}")
        return True
    else:
        print(f"No changes needed: {file_path}")
        return False

def main():
    """Fix all test files"""
    test_dir = "tests"
    fixed_count = 0

    for filename in os.listdir(test_dir):
        if filename.endswith(".rs"):
            file_path = os.path.join(test_dir, filename)
            if fix_test_file(file_path):
                fixed_count += 1

    print(f"\nFixed {fixed_count} test files")

if __name__ == "__main__":
    main()