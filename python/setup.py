import io
import os
import sys

from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

packages = [p for p in find_packages() if "tests" not in p]

PACKAGE_NAME = "snips_nlu_utils"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(ROOT_PATH, PACKAGE_NAME)
README = os.path.join(ROOT_PATH, "README.rst")
VERSION = "__version__"

RUST_EXTENSION_NAME = 'snips_nlu_utils.dylib.libsnips_nlu_utils_rs'
CARGO_ROOT_PATH = os.path.join(ROOT_PATH, 'ffi')
CARGO_FILE_PATH = os.path.join(CARGO_ROOT_PATH, 'Cargo.toml')
CARGO_TARGET_DIR = os.path.join(CARGO_ROOT_PATH, 'target')
os.environ['CARGO_TARGET_DIR'] = CARGO_TARGET_DIR

with io.open(os.path.join(PACKAGE_PATH, VERSION)) as f:
    version = f.readline().strip()

with io.open(README, 'rt', encoding='utf8') as f:
    readme = f.read()

rust_extension = RustExtension(
    RUST_EXTENSION_NAME, CARGO_FILE_PATH, debug="develop" in sys.argv,
    args=["--verbose"] if "--verbose" in sys.argv else None,
    binding=Binding.NoBinding)

install_requires = [
    "pathlib>=1.0,<2.0; python_version<'3.4'"
]

setup(name=PACKAGE_NAME,
      description="Python wrapper of the snips-nlu-utils Rust crate",
      long_description=readme,
      version=version,
      author="Adrien Ball",
      author_email="adrien.ball@snips.ai",
      license="Apache 2.0",
      classifiers=[
          "Programming Language :: Python :: 2",
          "Programming Language :: Python :: 2.7",
          "Programming Language :: Python :: 3",
          "Programming Language :: Python :: 3.4",
          "Programming Language :: Python :: 3.5",
          "Programming Language :: Python :: 3.6",
          "Programming Language :: Python :: 3.7",
      ],
      install_requires=install_requires,
      extras_require={"test": ["future>=0.16,<0.18"]},
      rust_extensions=[rust_extension],
      packages=packages,
      include_package_data=True,
      zip_safe=False)
