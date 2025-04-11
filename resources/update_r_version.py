import sys
import os
import subprocess
import re
from pathlib import Path


def update_r_version(description_path: Path, new_version: str) -> None:
    """
    Update the version string in R DESCRIPTION file.

    Parameters
    ----------
    description_path : Path
        Path to the DESCRIPTION file
    new_version : str
        New version string to set

    Returns
    -------
    None

    Raises
    ------
    FileNotFoundError
        If the DESCRIPTION file does not exist at the specified path
    ValueError
        If the DESCRIPTION file cannot be parsed or updated
    """
    if not description_path.exists():
        raise FileNotFoundError(f"DESCRIPTION file not found at {description_path}")

    # Read the DESCRIPTION file
    content = description_path.read_text()

    # Find the current version line
    version_pattern = re.compile(r'^Version:\s*([\d\.]+)', re.MULTILINE)
    match = version_pattern.search(content)

    if not match:
        raise ValueError("No Version field found in the DESCRIPTION file")

    old_version = match.group(1)

    # If version hasn't changed, no need to update
    if old_version == new_version:
        print(f"Version is already set to {new_version}")
        return

    # Replace the version string in the content
    new_content = version_pattern.sub(f'Version: {new_version}', content)

    # Write the updated content back to the file
    description_path.write_text(new_content)
    print(f"Successfully updated version from {old_version} to {new_version}")


def get_current_version(description_path: Path) -> str:
    """
    Get the current version from the DESCRIPTION file.

    Parameters
    ----------
    description_path : Path
        Path to the DESCRIPTION file

    Returns
    -------
    str
        Current version string

    Raises
    ------
    FileNotFoundError
        If the DESCRIPTION file does not exist at the specified path
    ValueError
        If the DESCRIPTION file cannot be parsed or the version is not found
    """
    if not description_path.exists():
        raise FileNotFoundError(f"DESCRIPTION file not found at {description_path}")

    content = description_path.read_text()

    # Find the current version line
    version_pattern = re.compile(r'^Version:\s*([\d\.]+)', re.MULTILINE)
    match = version_pattern.search(content)

    if not match:
        raise ValueError("No Version field found in the DESCRIPTION file")

    return match.group(1)


def increment_patch_version(version_str: str) -> str:
    """
    Increment the patch version of a semver string.

    Parameters
    ----------
    version_str : str
        Version string in format 'major.minor.patch[.distance]'

    Returns
    -------
    str
        Version with incremented patch number
    """
    # Extract the base version (remove any fourth digit if it exists)
    base_version = re.match(r'(\d+\.\d+\.\d+)(?:\.\d+)?', version_str).group(1)

    if not base_version:
        print(f"Warning: Could not parse version string '{version_str}'")
        return version_str

    # Split into major, minor, patch
    parts = base_version.split('.')
    if len(parts) != 3:
        print(f"Warning: Version '{base_version}' doesn't follow semver major.minor.patch format")
        return version_str

    # Increment patch version
    try:
        major, minor, patch = parts
        new_patch = int(patch) + 1
        return f"{major}.{minor}.{new_patch}"
    except (ValueError, IndexError) as e:
        print(f"Warning: Failed to increment patch version: {e}")
        return version_str


def get_distance_from_version_tag(version_str):
    """
    Get the distance (number of commits) from the tag matching the current version.

    Parameters
    ----------
    version_str : str
        The current version string to find matching tag

    Returns
    -------
    str
        The distance from the tag, or an empty string if git command fails
    """
    try:
        # Clean version string for tag matching (remove any fourth digit if it exists)
        base_parts = version_str.split('.')
        if len(base_parts) > 3:
            base_version = '.'.join(base_parts[:3])
        else:
            base_version = version_str

        # Format for tag search
        tag_prefix = f"v{base_version}"

        # First check if the tag exists
        result = subprocess.run(['git', 'tag', '-l', tag_prefix],  # skipcq: BAN-B607
                               capture_output=True, text=True, check=True)

        if not result.stdout.strip():
            print(f"Warning: No tag found matching '{tag_prefix}'")
            return "0"  # Return 0 if no tag found

        # Get the distance from tag to HEAD
        result = subprocess.run(['git', 'rev-list', f'{tag_prefix}..HEAD', '--count'],  # skipcq: BAN-B607
                               capture_output=True, text=True, check=True)
        distance = result.stdout.strip()
        return distance
    except (subprocess.SubprocessError, FileNotFoundError):
        print("Warning: Could not calculate distance from version tag")
        return "0"  # Default to 0 if there's an error


def generate_version_with_distance(base_version: str) -> str:
    """
    Generate a new version string by incrementing the patch version and
    appending the distance from the current version tag as a fourth digit.

    Parameters
    ----------
    base_version : str
        Base version string (e.g., '0.1.12' or '0.1.12.5')

    Returns
    -------
    str
        New version string with incremented patch and tag distance as fourth digit
        (e.g., '0.1.13.5') where 5 is the number of commits since the tag matching
        the original base version
    """
    # Clean the base version first (remove any existing fourth digit)
    base_parts = base_version.split('.')
    if len(base_parts) > 3:
        base_version = '.'.join(base_parts[:3])

    # Increment the patch version
    incremented_version = increment_patch_version(base_version)

    # Add the distance from version tag as the fourth digit
    distance = get_distance_from_version_tag(base_version)
    return f"{incremented_version}.{distance}" if distance else incremented_version


def extract_and_print_version(description_path: Path) -> str | None:
    """
    Extract the version from the DESCRIPTION file and print it to stdout.
    This is cross-platform compatible (works on Windows, macOS, Linux).

    Parameters
    ----------
    description_path : Path
        Path to the DESCRIPTION file

    Returns
    -------
    str
        The current version string
    """
    try:
        version = get_current_version(description_path)
        print(f"Current version: {version}")

        # For GitHub Actions: set as output
        if 'GITHUB_OUTPUT' in os.environ:
            with open(os.environ['GITHUB_OUTPUT'], 'a') as f:
                f.write(f"r_version={version}\n")

        # For regular environments: set an environment variable
        os.environ['PHYLO2VEC_R_VERSION'] = version
        print(f"Set environment variable PHYLO2VEC_R_VERSION={version}")

        return version
    except Exception as e:
        print(f"Error extracting version: {e}")
        return None


def main():
    """
    Main function to update the version in R DESCRIPTION file.

    This function can either use a provided version from command line arguments,
    or generate a new version based on the current version plus distance from tag.

    Parameters
    ----------
    None

    Returns
    -------
    None

    Notes
    -----
    Exits with code 1 if there's an error.

    Usage:
    - With argument: python update_r_version.py NEW_VERSION
    - Without argument: python update_r_version.py
      (will generate version based on current version + tag distance)
    - With --extract-only: python update_r_version.py --extract-only
      (will only extract and print the version, without modifying the file)
    """
    description_path = Path(__file__).parents[1] / "r-phylo2vec" / "DESCRIPTION"

    try:
        # Check if the user just wants to extract the version
        if len(sys.argv) > 1 and sys.argv[1] == "--extract-only":
            extract_and_print_version(description_path)
            return

        if len(sys.argv) > 1 and sys.argv[1] != "--extract-only":
            # Use provided version
            new_version = sys.argv[1]
        else:
            # Generate version with distance from version tag
            current_version = get_current_version(description_path)
            new_version = generate_version_with_distance(current_version)
            print(f"Generated new version: {new_version}")

        # Update the version in the DESCRIPTION file
        update_r_version(description_path, new_version)

        # Extract and output the updated version
        extract_and_print_version(description_path)

    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)


if __name__ == "__main__":
    main()
