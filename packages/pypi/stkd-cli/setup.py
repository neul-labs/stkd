#!/usr/bin/env python3
"""Setup script for stkd-cli PyPI package."""

import os
import platform
import subprocess
import sys
import urllib.request
import tarfile
import zipfile
import shutil

from setuptools import setup
from setuptools.command.install import install

REPO = "neul-labs/stkd"
BINARY_NAME = "gt"
VERSION = "0.1.0"

PLATFORMS = {
    ("Darwin", "x86_64"): "x86_64-apple-darwin",
    ("Darwin", "arm64"): "aarch64-apple-darwin",
    ("Linux", "x86_64"): "x86_64-unknown-linux-gnu",
    ("Linux", "aarch64"): "aarch64-unknown-linux-gnu",
}


def get_platform():
    system = platform.system()
    machine = platform.machine()
    key = (system, machine)

    target = PLATFORMS.get(key)
    if not target:
        print(f"Prebuilt binary not available for {system}-{machine}. Building from source...")
        build_from_source()
        return None
    return target


def build_from_source():
    try:
        subprocess.run(["cargo", "--version"], check=True, capture_output=True)
    except FileNotFoundError:
        print("cargo is not installed. Install Rust from https://rustup.rs")
        sys.exit(1)

    subprocess.run(["cargo", "install", "stkd-cli"], check=True)
    print("Installed from source via cargo.")


def download_binary(target):
    ext = "zip" if target.endswith("windows-msvc") else "tar.gz"
    asset_name = f"gt-{VERSION}-{target}.{ext}"
    url = f"https://github.com/{REPO}/releases/download/v{VERSION}/{asset_name}"

    print(f"Downloading gt v{VERSION} for {target}...")

    archive_path = os.path.join(os.getcwd(), asset_name)
    urllib.request.urlretrieve(url, archive_path)

    # Extract
    extract_dir = os.path.join(os.getcwd(), f"gt-{VERSION}-{target}")
    os.makedirs(extract_dir, exist_ok=True)

    if ext == "zip":
        with zipfile.ZipFile(archive_path, "r") as z:
            z.extractall(extract_dir)
    else:
        with tarfile.open(archive_path, "r:gz") as t:
            t.extractall(extract_dir, filter="data")

    os.remove(archive_path)

    # Find binary
    binary_ext = ".exe" if target.endswith("windows-msvc") else ""
    binary_name = f"{BINARY_NAME}{binary_ext}"

    # The binary is inside the extracted directory
    extracted_binary = os.path.join(extract_dir, binary_name)
    if os.path.exists(extracted_binary):
        return extracted_binary

    # Search recursively
    for root, _, files in os.walk(extract_dir):
        for f in files:
            if f == binary_name:
                return os.path.join(root, f)

    raise RuntimeError(f"Binary {binary_name} not found in extracted archive")


class InstallCommand(install):
    """Custom install command that downloads the prebuilt binary."""

    def run(self):
        install.run(self)

        target = get_platform()
        if target is None:
            return  # Built from source, cargo put it in PATH

        # Download binary
        binary_path = download_binary(target)

        # Install to scripts directory
        scripts_dir = os.path.join(self.install_scripts, os.path.basename(binary_path))
        os.makedirs(self.install_scripts, exist_ok=True)
        shutil.copy2(binary_path, scripts_dir)
        os.chmod(scripts_dir, 0o755)

        # Clean up
        shutil.rmtree(os.path.dirname(binary_path), ignore_errors=True)

        print(f"Installed gt to {scripts_dir}")


setup(
    name="stkd-cli",
    version=VERSION,
    description="Stack CLI - Stacked diffs for Git",
    long_description=open("README.md").read() if os.path.exists("README.md") else "",
    long_description_content_type="text/markdown",
    author="Neul Labs",
    author_email="hello@neullabs.com",
    url="https://github.com/neul-labs/stkd",
    license="Apache-2.0",
    classifiers=[
        "Development Status :: 4 - Beta",
        "Environment :: Console",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3.12",
        "Programming Language :: Rust",
        "Topic :: Software Development :: Version Control :: Git",
    ],
    python_requires=">=3.8",
    cmdclass={"install": InstallCommand},
    entry_points={
        "console_scripts": [
            "gt=stkd_cli:main",
        ],
    },
    packages=["stkd_cli"],
    package_dir={"stkd_cli": "stkd_cli"},
    zip_safe=False,
)
