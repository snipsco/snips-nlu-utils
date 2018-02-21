import io
import os

from setuptools import setup, find_packages
from setuptools_rust import RustExtension

try:
    from setuptools_rust import Binding

    rust_kwargs = {'binding': Binding.RustCPython}
except ImportError:
    rust_kwargs = dict()

packages = [p for p in find_packages() if "tests" not in p]

PACKAGE_NAME = "snips_nlu_utils"
ROOT_PATH = os.path.dirname(os.path.abspath(__file__))
PACKAGE_PATH = os.path.join(ROOT_PATH, PACKAGE_NAME)
README = os.path.join(ROOT_PATH, "README.rst")

CARGO_FILE_PATH = os.path.join(ROOT_PATH, 'snips_nlu_utils_py', 'Cargo.toml')
RUST_EXTENSION_NAME = 'snips_nlu_utils._snips_nlu_utils_py'

VERSION = "__version__"
with io.open(os.path.join(PACKAGE_PATH, VERSION)) as f:
    version = f.readline().strip()

with io.open(README, 'rt', encoding='utf8') as f:
    readme = f.read()

required = [
    "future==0.16.0"
]

setup(name=PACKAGE_NAME,
      description="NLU utils library for Snips NLU",
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
      ],
      install_requires=required,
      rust_extensions=[RustExtension(RUST_EXTENSION_NAME, CARGO_FILE_PATH,
                                     **rust_kwargs)],
      packages=packages,
      include_package_data=True,
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)
